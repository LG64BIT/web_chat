use actix::prelude::{Message, Recipient};
use uuid::Uuid;

/// Basic Message struct for holding message: [String]
#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);
/// Message struct that sends connect information
#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub lobby_id: Uuid,
    pub self_id: Uuid,
}
/// Message struct that sends disconnect information
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
    pub room_id: Uuid,
}
/// Message struct for sending message to group chat
#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id: Uuid,
    pub msg: String,
    pub room_id: Uuid,
}
