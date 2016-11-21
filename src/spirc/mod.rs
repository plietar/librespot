use futures::{Async, Poll, Future, Stream, Sink};

use broadcast::BroadcastReceiver;
use connection::ConnectionChange;
use session::Session;
use types::*;
use player::{Player, PlayerEvent};
use protobuf::{self, Message};
use protocol;
use protocol::spirc::{Frame, DeviceState, MessageType, State, PlayStatus};
use util::SpotifyId;

mod command_sender;
use self::command_sender::CommandSender;

pub struct SpircManager {
    ident: String,
    name: String,
    volume: u32,

    is_active: bool,
    became_active_at: i64,

    state: State,
    state_update_id: i64,

    session: Session,
    connection_updates: BroadcastReceiver<ConnectionChange>,
    seq_nr: u32,

    subscription: Option<SpStream<'static, Frame>>,
    sender: Option<SpSink<'static, Frame>>,

    player: Player,
}

impl SpircManager {
    pub fn new(session: &Session, name: String) -> SpircManager {
        SpircManager {
            ident: session.device_id(),
            name: name,
            volume: 0xFFFF,

            is_active: false,
            became_active_at: 0,

            state: State::new(),
            state_update_id: 0,

            session: session.clone(),
            connection_updates: session.connection().updates(),
            seq_nr: 0,

            subscription: None,
            sender: None,

            player: Player::new(session.clone()),
        }
    }

    fn build_subscription<'a>(&self, username: String) -> SpStream<'a, Frame> {
        let uri = format!("hm://remote/user/{}", username);
        let ident = self.ident.clone();

        self.session
            .mercury()
            .subscribe(uri)
            .flatten_stream()
            .and_then(|pkt| {
                let data = pkt.payload.first().unwrap();
                Ok(protobuf::parse_from_bytes::<Frame>(data)?)
            })
            .map(|frame| {
                debug!("{:?} {:?} {} {} {} {:?}",
                       frame.get_typ(),
                       frame.get_device_state().get_name(),
                       frame.get_ident(),
                       frame.get_seq_nr(),
                       frame.get_state_update_id(),
                       frame.get_recipient());
                frame
            })
            .filter(move |frame| {
                let recipients = frame.get_recipient();

                frame.get_ident() != ident && (recipients.len() == 0 || recipients.contains(&ident))
            })
            .sp_boxed()
    }

    fn build_sender<'a>(&self, username: String) -> SpSink<'a, Frame> {
        let uri = format!("hm://remote/user/{}", username);
        self.session
            .mercury()
            .sender(uri)
            .with(|frame: Frame| Ok(frame.write_to_bytes()?))
            .sp_boxed()
    }

    fn handle_connection(&mut self, username: String) {
        debug!("connected(username={:?})", username);

        self.subscription = Some(self.build_subscription(username.clone()));
        self.sender = Some(self.build_sender(username));

        self.command(MessageType::kMessageTypeHello).send();
    }

    fn process_frame(&mut self, frame: Frame) {
        if frame.get_state_update_id() > self.state_update_id {
            self.state = frame.get_state().clone();
            self.state_update_id = frame.get_state_update_id();
        }

        if frame.get_device_state().get_is_active() &&
            self.is_active &&
            frame.get_device_state().get_became_active_at() > self.became_active_at
        {
            self.is_active = false;
            self.player.stop();

            self.notify(None);
        }

        let sender = frame.get_ident().to_owned();
        match frame.get_typ() {
            MessageType::kMessageTypeHello => self.notify(sender),

            MessageType::kMessageTypeVolume => {
                self.volume = frame.get_volume();
                self.notify(None);
            }

            MessageType::kMessageTypePlay => {
                self.state.set_status(PlayStatus::kPlayStatusPlay);
                self.player.play();
                self.notify(None);
            }

            MessageType::kMessageTypePause => {
                self.state.set_status(PlayStatus::kPlayStatusPause);
                self.player.pause();
                self.notify(None);
            }

            MessageType::kMessageTypeLoad => {
                if !self.is_active {
                    self.is_active = true;
                    self.became_active_at = self.session.time() as i64;
                }

                self.state = frame.get_state().clone();
                self.state_update_id = self.session.time() as i64;
                self.state.set_playing_from_fallback(true);

                let index = self.state.get_playing_track_index();
                let track = self.state.get_track().get(index as usize).cloned();

                if let Some(track) = track {
                    self.player.load(SpotifyId::from_raw(track.get_gid()));
                    self.state.set_position_ms(0);
                    self.state.set_position_measured_at(self.session.time());

                    if self.state.get_status() == PlayStatus::kPlayStatusPlay {
                        self.player.play();
                    }
                }

                self.notify(None);
            }

            _ => {
                self.notify(None);
            }
        }
    }

    fn next_seq(&mut self) -> u32 {
        self.seq_nr += 1;
        self.seq_nr
    }

    fn command(&mut self, cmd: MessageType) -> CommandSender {
        CommandSender::new(self, cmd)
    }

    fn send_frame(&mut self, frame: Frame) {
        if let Some(ref mut sender) = self.sender {
            sender.start_send(frame).expect("Send failed");
        } else {
            warn!("Not connected, dropping packet");
        }
    }

    fn notify<T>(&mut self, recipient: T)
        where T: Into<Option<String>>
    {
        let state = self.state.clone();
        let update_id = self.state_update_id;
        self.command(MessageType::kMessageTypeNotify)
            .recipient(recipient)
            .state(state, update_id)
            .send();
    }

    pub fn device_state(&self) -> DeviceState {
        protobuf_init!(DeviceState::new(), {
            sw_version: "librespot-v0.2",
            is_active: self.is_active,
            became_active_at: self.became_active_at,
            can_play: true,
            volume: self.volume,
            name: self.name.clone(),
            error_code: 0,
            became_active_at: 0,
            capabilities => [
                @{
                    typ: protocol::spirc::CapabilityType::kCanBePlayer,
                    intValue => [0]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kDeviceType,
                    intValue => [1]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kGaiaEqConnectId,
                    intValue => [1]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kSupportsLogout,
                    intValue => [1]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kSupportsRename,
                    intValue => [1]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kIsObservable,
                    intValue => [1]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kVolumeSteps,
                    intValue => [10]
                },
                @{
                    typ: protocol::spirc::CapabilityType::kSupportedContexts,
                    stringValue => []
                },
                @{
                    typ: protocol::spirc::CapabilityType::kSupportedTypes,
                    stringValue => [
                        "audio/local",
                        "audio/track",
                        "local",
                        "track",
                    ]
                }
            ],
        })
    }
}

impl Future for SpircManager {
    type Item = ();
    type Error = SpError;

    fn poll(&mut self) -> Poll<(), SpError> {
        loop {
            let mut progress = false;

            let poll_connection = self.connection_updates.poll()?;
            if let Async::Ready(Some(change)) = poll_connection {
                let ConnectionChange::Connected(username) = change;
                self.handle_connection(username);
                progress = true;
            }

            let poll_subscription = self.subscription
                .as_mut()
                .map(Stream::poll)
                .unwrap_or(Ok(Async::NotReady))?;

            if let Async::Ready(Some(frame)) = poll_subscription {
                self.process_frame(frame);

                progress = true;
            }

            if let Some(ref mut sender) = self.sender {
                sender.poll_complete()?;
            }

            match self.player.poll()? {
                Async::Ready(Some(PlayerEvent::TrackEnd)) => {
                    let index = self.state.get_playing_track_index() % self.state.get_track().len() as u32;
                    self.state.set_playing_track_index(index);

                    let track = self.state.get_track().get(index as usize).cloned();
                    if let Some(track) = track {
                        self.player.load(SpotifyId::from_raw(track.get_gid()));
                        self.player.play();
                        progress = true;
                    } else {
                        self.state.set_status(PlayStatus::kPlayStatusStop);
                        self.state.set_playing_track_index(0);
                    }

                    self.state_update_id = self.session.time() as i64;
                    self.state.set_position_ms(0);
                    self.state.set_position_measured_at(self.session.time());
                    self.notify(None);
                }

                Async::Ready(Some(PlayerEvent::Playing(position_ms))) => {
                    self.state.set_position_ms(position_ms);
                    self.state.set_position_measured_at(self.session.time());
                }
                _ => (),
            }

            if !progress {
                return Ok(Async::NotReady);
            }
        }
    }
}
