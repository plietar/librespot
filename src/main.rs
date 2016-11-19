extern crate argparse;
extern crate env_logger;
extern crate futures;
extern crate librespot;
extern crate rpassword;
extern crate tokio_core as tokio;

use argparse::ArgumentParser;
use librespot::{Credentials, Session, SpircManager};
use rpassword::prompt_password_stdout;
use tokio::reactor::Core;

#[derive(Default)]
struct Args {
    name: String,
    username: String,
    password: Option<String>,
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
              .add_option(&["-u", "--username"], argparse::Store, "Username")
              .required();
            ap.refer(&mut args.password)
              .add_option(&["-p", "--password"], argparse::StoreOption, "Password");

            ap.parse_args_or_exit();
        }

        args
    }

    fn credentials(&self) -> Credentials {
        let password = self.password
            .as_ref()
            .map(String::to_owned)
            .unwrap_or_else(|| prompt_password_stdout("Password: ").unwrap());

        Credentials::with_password(self.username.to_owned(), password)
    }
}

pub fn main() {
    env_logger::init().unwrap();

    let args = Args::parse_or_exit();

    let mut core = Core::new().unwrap();
    let session = Session::new(&core.handle());

    session.spawn(session.connection().connect(args.credentials()));

    let ident = String::from("foobar");
    core.run(SpircManager::new(&session, ident, args.name.clone())).unwrap();
}
