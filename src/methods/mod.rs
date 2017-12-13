//! Telegram bot methods.

///// Sends the message silently. Users will receive a notification with no sound.
//#[proc_macro_derive(DisableNotification)]
//pub fn disable_notification(&mut self, v: Boolean) -> &mut Self {
//    self.disable_notification = Some(v);
//    self
//}

pub mod get_updates;
pub mod send_message;
pub mod forward_message;
pub mod send_photo;
pub mod send_audio;
pub mod send_document;
pub mod send_video;
pub mod send_voice;
pub mod send_video_note;
pub mod send_media_group;
pub mod send_location;

pub use self::get_updates::*;
pub use self::send_message::*;
pub use self::forward_message::*;
pub use self::send_photo::*;
pub use self::send_audio::*;
pub use self::send_document::*;
pub use self::send_video::*;
pub use self::send_voice::*;
pub use self::send_video_note::*;
pub use self::send_media_group::*;
pub use self::send_location::*;