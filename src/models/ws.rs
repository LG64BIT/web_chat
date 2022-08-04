use crate::models::lobby::Lobby;
use crate::models::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use actix::{fut, ActorContext, ActorFutureExt, ContextFutureSpawner, WrapFuture};
use actix::{Actor, Addr, Running, StreamHandler};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;
use std::time::{Duration, Instant};
use uuid::Uuid;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Struct for representing web socket connection
pub struct WsConn {
    room: Uuid,
    lobby_addr: Addr<Lobby>,
    hb: Instant,
    id: Uuid,
}

impl WsConn {
    /// Function that creates new [WsConn] instance for currently logged in user
    pub fn new(room: Uuid, lobby: Addr<Lobby>, user_id: Uuid) -> WsConn {
        WsConn {
            id: user_id, //Uuid::new_v4(),
            room,
            hb: Instant::now(),
            lobby_addr: lobby,
        }
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;
    /// Actor state function, gets called first when [ws::start] function is called
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.lobby_addr
            .send(Connect {
                addr: addr.recipient(),
                lobby_id: self.room,
                self_id: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }
    /// Actor state function, sends disconnect message to group and stops actor
    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.lobby_addr.do_send(Disconnect {
            id: self.id,
            room_id: self.room,
        });
        Running::Stop
    }
}

impl WsConn {
    /// Function that preforms heartbeating
    /// * Heartbeating is occasionally checking if client is still responsive
    /// by pinging him by some message.
    /// * [HEARTBEAT_INTERVAL] constant defines period of heartbeating
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting failed heartbeat");
                act.lobby_addr.do_send(Disconnect {
                    id: act.id,
                    room_id: act.room,
                });
                ctx.stop();
                return;
            }

            ctx.ping(b"hi");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    /// Implementing web socket message support for WsConn
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(Text(s)) => self.lobby_addr.do_send(ClientActorMessage {
                id: self.id,
                msg: s.to_string(),
                room_id: self.room,
            }),

            Err(e) => panic!("{}", e),
        }
    }
}

impl Handler<WsMessage> for WsConn {
    type Result = ();
    /// Method that specifies how WsConn should handle WsMessage
    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
