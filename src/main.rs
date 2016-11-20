extern crate argparse;
extern crate env_logger;
extern crate futures;
extern crate librespot;
extern crate rpassword;
extern crate tokio_core as tokio;

use argparse::ArgumentParser;
use futures::Stream;
use librespot::{Credentials, Session, SpircManager};
use librespot::connection::ConnectionChange;
use rpassword::prompt_password_stdout;
use tokio::reactor::Core;

#[derive(Default)]
pub struct Args {
    name: String,
    username: Option<String>,
    password: Option<String>,
    discovery: bool,
}

impl Args {
    fn parse_or_exit() -> Args {
        let mut args = Args::default();

        {
            let mut ap = ArgumentParser::new();
            ap.set_description("librespot");

            ap.refer(&mut args.name)
              .add_option(&["--name"], argparse::Store, "Device name")
              .required();
            ap.refer(&mut args.username)
              .add_option(&["-u", "--username"], argparse::StoreOption, "Username");
            ap.refer(&mut args.password)
              .add_option(&["-p", "--password"], argparse::StoreOption, "Password");
            discovery::add_flag(&mut ap, &mut args.discovery);

            ap.parse_args_or_exit();
        }

        args
    }

    fn credentials(&self) -> Option<Credentials> {
        match (&self.username, &self.password) {
            (&Some(ref username), &Some(ref password))
                => Some(Credentials::with_password(username.clone(), password.clone())),

            (&Some(ref username), &None) => {
                let password = prompt_password_stdout("Password: ").unwrap();
                Some(Credentials::with_password(username.clone(), password))
            }

            (&None, _) => None,
        }
    }
}

pub fn main() {
    env_logger::init().unwrap();

    let args = Args::parse_or_exit();
    if args.username.is_none() && !args.discovery {
        panic!("No username specified and discovery not enabled");
    }

    let mut core = Core::new().unwrap();
    let session = Session::new(&core.handle());

    if args.discovery {
        discovery::start(&args, &session);
    }

    if let Some(credentials) = args.credentials() {
        session.spawn(session.connection().connect(credentials));
    }

    session.spawn(session.connection().updates().for_each(|update| {
        let ConnectionChange::Connected(username) = update;
        println!("Authenticated as {}", username);
        Ok(())
    }));

    core.run(SpircManager::new(&session, args.name.clone())).unwrap();
}

#[cfg(not(target_os="windows"))]
mod discovery {
    pub use super::Args;
    use librespot::authentication::Discovery;
    use librespot::Session;
    use argparse::{self, ArgumentParser};
    use futures::Stream;

    pub fn add_flag<'a>(ap: &mut ArgumentParser<'a>, flag: &'a mut bool) {
        ap.refer(flag)
          .add_option(&["--discovery"], argparse::StoreTrue, "Enable discovery mode");
    }

    pub fn start(args: &Args, session: &Session) {
        let session_ = session.clone();
        let discovery = Discovery::new(&session.handle(), args.name.clone(), session.device_id()).unwrap();
        let task = discovery.map_err(From::from).and_then(move |creds| {
            session_.connection().connect(creds)
        }).for_each(|_| Ok(()));

        session.spawn(task);
    }
}

#[cfg(target_os="windows")]
mod discovery {
    pub use super::Args;
    use librespot::Session;
    use argparse::ArgumentParser;

    pub fn add_flag(_ap: &mut ArgumentParser, _flag: &mut bool) { }
    pub fn start(_args: &Args, _session: &Session) { }
}
