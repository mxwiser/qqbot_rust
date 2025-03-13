use super::MessageEvent;
use super::bot_error;

pub trait BotTrait{
  async fn message_process(_message_event: &MessageEvent) -> Result<(), bot_error::Error>;
}


