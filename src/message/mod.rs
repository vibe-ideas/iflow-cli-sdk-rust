//! Message processing utilities for iFlow SDK
//!
//! This module contains utilities for processing and converting messages
//! between different formats used in the iFlow SDK.

pub mod utils;
pub mod types;

pub use utils::{convert_content_block_to_text, create_message_from_content, process_session_update};
pub use types::{PlanEntry, PlanPriority, PlanStatus, Message, UserMessageChunk, UserMessage, ToolCallMessage};