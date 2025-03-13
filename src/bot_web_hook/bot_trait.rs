use super::BotHook;
use super::MessageEvent;
use super::bot_error;
use super::message::MessageHelper;
use super::info;

pub trait BotTrait{
  async fn message_process(_message_event: &MessageEvent) -> Result<(), bot_error::Error>;
}


