//! Telegram bot methods.

pub mod get_updates;
pub mod send_message;
pub mod forward_message;
pub mod send_photo;
pub mod send_audio;
pub mod send_document;
pub mod send_video;
pub mod send_voice;

pub use self::get_updates::*;
pub use self::send_message::*;
pub use self::forward_message::*;
pub use self::send_photo::*;
pub use self::send_audio::*;
pub use self::send_document::*;
pub use self::send_video::*;
pub use self::send_voice::*;
