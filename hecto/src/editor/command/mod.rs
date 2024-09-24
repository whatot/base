mod edit_cmd;
mod move_cmd;
mod system_cmd;

use crossterm::event::Event;
pub use edit_cmd::Edit;
pub use move_cmd::Move;
pub use system_cmd::System;

use crate::prelude::*;

#[derive(Clone, Copy)]
pub enum Command {
    Move(Move),
    Edit(Edit),
    System(System),
}

impl TryFrom<Event> for Command {
    type Error = String;

    #[allow(clippy::as_conversions)]
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(key_event) => Edit::try_from(key_event)
                .map(Command::Edit)
                .or_else(|_| Move::try_from(key_event).map(Command::Move))
                .or_else(|_| System::try_from(key_event).map(Command::System))
                .map_err(|_err| format!("Event not supported: {key_event:?}")),

            Event::Resize(width_u16, height_u16) => Ok(Self::System(System::Resize(Size {
                width: width_u16 as usize,
                height: height_u16 as usize,
            }))),

            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}
