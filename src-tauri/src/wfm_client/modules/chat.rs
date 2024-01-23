use std::f32::consts::E;

use eyre::eyre;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    enums::LogLevel,
    error::{ApiResult, AppError},
    helper, logger,
    structs::{Item, ItemDetails},
    wfm_client::client::WFMClient,
};

pub struct ChatModule<'a> {
    pub client: &'a WFMClient,
}

impl<'a> ChatModule<'a> {
    pub async fn get_chats(&self) -> Result<Vec<ChatData>, AppError> {
        match self.client.get::<Vec<ChatData>>("im/chats", Some("chats")).await {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    "Chat:GetChats",
                    format!("{} was fetched.", payload.len()).as_str(),
                    None,
                );
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(AppError::new_api(
                    "WarframeMarket:Chat:GetChats",
                    error,
                    eyre!(""),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
    }

    pub async fn get_chat(&self, id: String) -> Result<Vec<ChatMessage>, AppError> {
        let url = format!("im/chats/{}", id);
        match self.client.get::<Vec<ChatMessage>>(&url, Some("messages")).await {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    "Chat:GetChat",
                    format!("{} chat messages were fetched.", payload.len()).as_str(),
                    None,
                );
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(AppError::new_api(
                    "WarframeMarket:Chat:GetChat",
                    error,
                    eyre!(""),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
    }

    pub async fn delete(&self, id: String) -> Result<String, AppError> {
        let url = format!("im/chats/{}", id);
        match self.client.delete(&url, Some("chat_id")).await {
            Ok(ApiResult::Success(payload, _headers)) => {
                self.client.debug(
                    "Chat:Delete",
                    format!("Chat {} was deleted.", id).as_str(),
                    None,
                );
                return Ok(payload);
            }
            Ok(ApiResult::Error(error, _headers)) => {
                return Err(AppError::new_api(
                    "WarframeMarket:Chat:Delete",
                    error,
                    eyre!(""),
                    LogLevel::Error,
                ));
            }
            Err(err) => {
                return Err(err);
            }
        };
    }
    pub fn emit(&self, operation: &str, data: serde_json::Value) {
        helper::emit_update("ChatMessages", operation, Some(data));
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ChatData {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "chat_with")]
    pub chat_with: Vec<ChatMessageWith>,

    #[serde(rename = "unread_count")]
    pub unread_count: i64,

    #[serde(rename = "chat_name")]
    pub chat_name: String,

    #[serde(rename = "messages")]
    pub messages: Vec<ChatMessage>,

    #[serde(rename = "last_update")]
    pub last_update: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ChatMessageWith {
    #[serde(rename = "reputation")]
    pub reputation: i64,

    #[serde(rename = "locale")]
    pub locale: String,

    #[serde(rename = "avatar")]
    pub avatar: Option<String>,

    #[serde(rename = "last_seen")]
    pub last_seen: String,

    #[serde(rename = "ingame_name")]
    pub ingame_name: String,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "region")]
    pub region: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    #[serde(rename = "message")]
    pub message: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "chat_id")]
    pub chat_id: String,

    #[serde(rename = "send_date")]
    pub send_date: String,

    #[serde(rename = "message_from")]
    pub message_from: String,

    #[serde(rename = "raw_message")]
    pub raw_message: Option<String>,
}
