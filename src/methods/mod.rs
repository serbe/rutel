//! Telegram bot methods.

pub mod get_updates;
pub mod send_message;
pub mod forward_message;

pub use self::get_updates::*;
pub use self::send_message::*;
pub use self::forward_message::*;

