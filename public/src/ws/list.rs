use crate::prelude::*;
use crate::ws::session::{Context, Session, SessionTrait};
use types::{ListToClient, ListToServer};

pub struct ListSession;

impl SessionTrait for ListSession {
    type Sender = ListToServer;

    fn tag() -> &'static str {
        "list"
    }

    fn receive(&mut self, msg: String, _: &Context<Self>) -> (&str, JsValue) {
        let msg: ListToClient = serde_json::from_str(&*msg).unwrap();
        ("list", JsValue::from_serde(&msg).unwrap())
    }
}

#[wasm_bindgen]
pub struct List {
    session: Session<ListSession>,
}

#[wasm_bindgen]
impl List {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<List> {
        Ok(List {
            session: ListSession.start()?,
        })
    }

    pub fn on(&self, tag: String, callback: Function) {
        self.session.on(tag, callback);
    }

    pub fn subscribe(&self, room_id: &JsValue) {
        self.session
            .send(ListToServer::Subscribe(room_id.into_serde().unwrap()));
    }

    pub fn unsubscribe(&self, room_id: &JsValue) {
        self.session
            .send(ListToServer::Unsubscribe(room_id.into_serde().unwrap()));
    }
}
