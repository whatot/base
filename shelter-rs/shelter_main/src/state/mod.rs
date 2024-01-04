use std::sync::Arc;

use crate::settings::Settings;
use anyhow::Ok;
use arc_swap::ArcSwap;

pub struct ApplicationState {
    pub settings: ArcSwap<Settings>,
}

impl ApplicationState {
    pub fn new(settings: &Settings) -> anyhow::Result<Self> {
        Ok(Self {
            settings: ArcSwap::new(Arc::new((*settings).clone())),
        })
    }
}
