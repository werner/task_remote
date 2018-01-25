use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
use uuid::Uuid;

use ssh2::Session;

pub struct Ssh<'a> {
  user: &'a str,
  server: &'a str,
}

impl<'a> Ssh<'a> {
  pub fn new(user: &'a str, server: &'a str) -> Ssh<'a> {
    Ssh {
      user: user,
      server: server,
    }
  }

  fn connect(&mut self) -> Result<Session, String> {
    match TcpStream::connect(format!("{}:22", self.server)) {
      Ok(tcp) => {
        let mut sess = Session::new().unwrap();
        sess.handshake(&tcp).unwrap();
        sess.userauth_agent(self.user).unwrap();
        Ok(sess)
      }
      Err(error) => Err(error.to_string()),
    }
  }

  pub fn execute(&mut self, command: &str) -> String {
    match self.connect() {
      Ok(sess) => {
        let mut channel = sess.channel_session().unwrap();
        channel.exec(command).unwrap();
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        channel.wait_close().unwrap();
        format!("{}, exit code: {}", s, channel.exit_status().unwrap())
      }
      Err(error) => error,
    }
  }

  pub fn upload_code(&mut self, code: &str) -> String {
    match self.connect() {
      Ok(sess) => {
        let file_name = Uuid::new_v4();
        match sess.scp_send(Path::new(&format!("/tmp/{}", file_name)), 0o644, 10, None) {
          Ok(mut remote_file) => {
            remote_file.write(code.as_bytes()).unwrap();
            file_name.to_string()
          }
          Err(error) => error.to_string(),
        }
      }
      Err(error) => error,
    }
  }

  // to debug
  fn show_agents(&mut self) {
    let sess = self.connect().unwrap();
    let mut agent = sess.agent().unwrap();

    agent.connect().unwrap();
    agent.list_identities().unwrap();

    for identity in agent.identities() {
      let identity = identity.unwrap();
      println!("{}", identity.comment());
    }
  }
}
