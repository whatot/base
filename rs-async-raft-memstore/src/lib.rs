use std::{
    clone,
    collections::{BTreeMap, HashMap},
    io::Cursor,
};

use anyhow::{Ok, Result};
use async_raft::{
    async_trait::async_trait,
    raft::{Entry, EntryPayload, MembershipConfig},
    storage::{CurrentSnapshotData, HardState, InitialState},
    AppData, AppDataResponse, NodeId, RaftStorage,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[cfg(test)]
mod test;

const ERR_INCONSISTENT_LOG: &str =
    "a query was received which was expecting data to be in place which does not exist in the log";

/// The application data request type which the `MemStore` works with.
///
/// Conceptually, for demo purposes, this represents an update to a client's status info,
/// returning the previously recorded status.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientRequest {
    /// The ID of the client which has sent the request.
    pub client: String,
    /// The serial number of this request.
    pub serial: u64,
    /// A string describing the status of the client. For a real application, this should probably
    /// be an enum representing all of the various types of requests / operations which a client
    /// can perform.
    pub status: String,
}

impl AppData for ClientRequest {}

/// The application data response type which the `MemStore` works with.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientResponse(Option<String>);

impl AppDataResponse for ClientResponse {}

/// Error used to trigger Raft shutdown from storage.
#[derive(Debug, Clone, Error)]
pub enum ShutdownError {
    #[error("unsafe storage error")]
    UnsafeStorageError,
}

/// The application snapshot type which the `MemStore` works with.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemStoreSnapshot {
    /// The last index covered by this snapshot.
    pub index: u64,
    /// The term of the last index covered by this snapshot.
    pub term: u64,
    /// The last memberhsip config included in this snapshot.
    pub menbership: MembershipConfig,
    /// The data of the state machine at the time of this snapshot.
    pub data: Vec<u8>,
}

/// The data of the state machine at the time of this snapshot.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MemStoreStateMachine {
    pub last_applied_log: u64,
    /// A mapping of client IDs to their state info.
    pub client_serial_responses: HashMap<String, (u64, Option<String>)>,
    /// The current status of a client by ID.
    pub client_status: HashMap<String, String>,
}

/// An in-memory storage system implementing the `async_raft::RaftStorage` trait.
pub struct MemStore {
    /// The ID of the Raft node for which this memory storage instances is configured.
    id: NodeId,
    /// The Raft log.
    log: RwLock<BTreeMap<u64, Entry<ClientRequest>>>,
    /// The Raft state machine.
    sm: RwLock<MemStoreStateMachine>,
    /// The current hard state.
    hs: RwLock<Option<HardState>>,
    /// The current snapshot.
    current_snapshot: RwLock<Option<MemStoreSnapshot>>,
}

impl MemStore {
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            log: RwLock::new(BTreeMap::new()),
            sm: RwLock::new(MemStoreStateMachine::default()),
            hs: RwLock::new(None),
            current_snapshot: RwLock::new(None),
        }
    }

    /// Create a new `MemStore` instance with some existing state (for testing).
    #[cfg(test)]
    pub fn new_with_state(
        id: NodeId,
        log: BTreeMap<u64, Entry<ClientRequest>>,
        sm: MemStoreStateMachine,
        hs: Option<HardState>,
        current_snapshot: Option<MemStoreSnapshot>,
    ) -> Self {
        Self {
            id,
            log: RwLock::new(log),
            sm: RwLock::new(sm),
            hs: RwLock::new(hs),
            current_snapshot: RwLock::new(current_snapshot),
        }
    }

    /// Get a handle to the log for testing purposes.
    pub async fn get_log(&self) -> RwLockWriteGuard<'_, BTreeMap<u64, Entry<ClientRequest>>> {
        self.log.write().await
    }

    /// Get a handle to the state machine for testing purposes.
    pub async fn get_state_machine(&self) -> RwLockWriteGuard<'_, MemStoreStateMachine> {
        self.sm.write().await
    }

    /// Get a handle to the current hard state for testing purposes.
    pub async fn read_hard_status(&self) -> RwLockReadGuard<'_, Option<HardState>> {
        self.hs.read().await
    }
}

#[async_trait]
impl RaftStorage<ClientRequest, ClientResponse> for MemStore {
    type Snapshot = Cursor<Vec<u8>>;
    type ShutdownError = ShutdownError;

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_membership_config(&self) -> Result<MembershipConfig> {
        let log = self.log.read().await;

        let cfg_opt = log.values().rev().find_map(|entry| match &entry.payload {
            EntryPayload::ConfigChange(cfg) => Some(cfg.membership.clone()),
            EntryPayload::SnapshotPointer(snap) => Some(snap.membership.clone()),
            _ => None,
        });

        Ok(match cfg_opt {
            Some(cfg) => cfg,
            None => MembershipConfig::new_initial(self.id),
        })
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_initial_state(&self) -> Result<InitialState> {
        let membership = self.get_membership_config().await?;
        let mut hs = self.hs.write().await;
        let log = self.log.read().await;
        let sm = self.sm.read().await;

        match &mut *hs {
            Some(inner) => {
                let (last_log_index, last_log_term) = match log.values().rev().next() {
                    Some(log) => (log.index, log.term),
                    None => (0, 0),
                };
                Ok(InitialState {
                    last_log_index,
                    last_log_term,
                    last_applied_log: sm.last_applied_log,
                    hard_state: inner.clone(),
                    membership,
                })
            }
            None => {
                let new = InitialState::new_initial(self.id);
                *hs = Some(new.hard_state.clone());
                Ok(new)
            }
        }
    }

    #[tracing::instrument(level = "trace", skip(self, hs))]
    async fn save_hard_state(&self, hs: &HardState) -> Result<()> {
        *self.hs.write().await = Some(hs.clone());
        // self.hs.write().await.replace(hs.clone());
        Ok(())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_log_entries(&self, start: u64, stop: u64) -> Result<Vec<Entry<ClientRequest>>> {
        // Invalid request, return empty vec.
        if start > stop {
            tracing::error!("invalid request: start={start} > stop={stop}");
            return Ok(vec![]);
        }
        let log = self.log.read().await;
        Ok(log.range(start..stop).map(|(_, val)| val.clone()).collect())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn delete_logs_from(&self, start: u64, stop: Option<u64>) -> Result<()> {
        if stop.map(|stop| start > stop).unwrap_or(false) {
            tracing::error!("invalid request: start > stop");
            return Ok(());
        }

        let mut log = self.log.write().await;

        if let Some(stop) = stop {
            // If a stop point was specified, delete from start until the given stop point.
            for key in start..stop {
                log.remove(&key);
            }
        } else {
            // Else, just split off the remainder.
            log.split_off(&start);
        }

        return Ok(());
    }

    #[tracing::instrument(level = "trace", skip(self, entry))]
    async fn append_entry_to_log(&self, entry: &Entry<ClientRequest>) -> Result<()> {
        todo!()
    }

    #[tracing::instrument(level = "trace", skip(self, entry))]
    async fn replicate_to_log(&self, entry: &[Entry<ClientRequest>]) -> Result<()> {
        todo!()
    }

    #[tracing::instrument(level = "trace", skip(self, data))]
    async fn apply_entry_to_state_machine(&self, index: &u64, data: &ClientRequest) -> Result<ClientResponse> {
        todo!()
    }

    #[tracing::instrument(level = "trace", skip(self, entries))]
    async fn replicate_to_state_machine(&self, entries: &[(&u64, &ClientRequest)]) -> Result<()> {
        todo!()
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn do_log_compaction(&self) -> Result<CurrentSnapshotData<Self::Snapshot>> {
        todo!()
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn create_snapshot(&self) -> Result<(String, Box<Self::Snapshot>)> {
        todo!()
    }

    #[tracing::instrument(level = "trace", skip(self, snapshot))]
    async fn finalize_snapshot_installation(
        &self,
        index: u64,
        term: u64,
        delete_through: Option<u64>,
        id: String,
        snapshot: Box<Self::Snapshot>,
    ) -> Result<()> {
        todo!()
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn get_current_snapshot(&self) -> Result<Option<CurrentSnapshotData<Self::Snapshot>>> {
        todo!()
    }
}
