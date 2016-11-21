use super::SpircManager;
use protocol;
use protocol::spirc::{MessageType, State};
use protobuf::RepeatedField;
use std::iter::FromIterator;

#[must_use]
pub struct CommandSender<'a> {
    manager: &'a mut SpircManager,
    cmd: MessageType,
    recipient: Option<String>,
    state: State,
    state_update_id: i64,
}

impl<'a> CommandSender<'a> {
    pub fn new(manager: &'a mut SpircManager, cmd: MessageType) -> CommandSender {
        CommandSender {
            manager: manager,
            cmd: cmd,
            recipient: None,
            state: State::new(),
            state_update_id: 0,
        }
    }

    pub fn recipient<T>(mut self, r: T) -> CommandSender<'a>
        where T: Into<Option<String>>
    {
        self.recipient = r.into();
        self
    }

    pub fn state(mut self, s: State, update_id: i64) -> CommandSender<'a> {
        self.state = s;
        self.state_update_id = update_id;
        self
    }

    pub fn send(self) {
        let manager = self.manager;

        let frame = protobuf_init!(protocol::spirc::Frame::new(), {
            version: 1,
            ident: manager.ident.clone(),
            protocol_version: "2.0.0",
            seq_nr: manager.next_seq(),
            typ: self.cmd,
            recipient: RepeatedField::from_iter(self.recipient),
            device_state: manager.device_state(),
            state_update_id: self.state_update_id,
            state: self.state,
        });

        manager.send_frame(frame);
    }
}
