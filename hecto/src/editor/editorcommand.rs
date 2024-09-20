use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use super::terminal::Size;

pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Left,
    Right,
    Down,
}

pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Quit,
    Insert(char),
}

impl TryFrom<Event> for EditorCommand {
    type Error = String;

    #[allow(clippy::as_conversions)]
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => Ok(Self::Insert(c)),
                (KeyCode::Down, _) => Ok(Self::Move(Direction::Down)),
                (KeyCode::Up, _) => Ok(Self::Move(Direction::Up)),
                (KeyCode::Left, _) => Ok(Self::Move(Direction::Left)),
                (KeyCode::Right, _) => Ok(Self::Move(Direction::Right)),
                (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUp)),
                (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                (KeyCode::End, _) => Ok(Self::Move(Direction::End)),
                _ => Err(format!("Key Code not supported: {code:?}")),
            },
            Event::Resize(width_u16, height_u16) => Ok(Self::Resize(Size {
                width: width_u16 as usize,
                height: height_u16 as usize,
            })),
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}
