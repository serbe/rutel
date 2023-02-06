//! [Telegram bot API] implementation on [Rust Programming Language]. It handles all the difficult stuff so you can focus only on
//! your business logic.
//!
//! For a high-level overview, see [our GitHub repository](https://github.com/teloxide/teloxide).
//!
//! [[`examples/throw_dice.rs`](https://github.com/teloxide/teloxide/blob/master/examples/throw_dice.rs)]
//! ```no_run
//! # #[cfg(feature = "ctrlc_handler")]
//! use teloxide::prelude::*;
//!
//! # #[cfg(feature = "ctrlc_handler")]
//! # #[tokio::main]
//! # async fn main() {
//! pretty_env_logger::init();
//! log::info!("Starting throw dice bot...");
//!
//! let bot = Bot::from_env();
//!
//! teloxide::repl(bot, |bot: Bot, msg: Message| async move {
//!     bot.send_dice(msg.chat.id).await?;
//!     Ok(())
//! })
//! .await;
//! # } #[cfg(not(feature = "ctrlc_handler"))] fn main(){}
//! ```
//!
//! <div align="center">
//!   <kbd>
//!     <img src=https://github.com/teloxide/teloxide/raw/master/media/throw-dice.gif width=420px />
//!   </kbd>
//! </div>
//!
//! [Telegram bot API]: https://core.telegram.org/bots/api
//! [Rust Programming Language]: https://www.rust-lang.org/

#![warn(rustdoc::broken_intra_doc_links)]

//#![allow(dead_code)]
//#![allow(doc_markdown)]
//#![allow(empty_line_after_outer_attr)]
//#![allow(unused_attributes)]

pub mod bot;
pub mod error;
pub mod types;
