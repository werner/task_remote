use std::io::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
use uuid::Uuid;

use ssh2::Session;

pub struct Ssh<'a> {
  user: &'a str,
  tcp: Result<TcpStream, Error>
}

impl<'a> Ssh<'a> {
  pub fn new(user: &'a str, server: &'a str) -> Ssh<'a> {
    Ssh {
      user: user,
      tcp: TcpStream::connect(format!("{}:22", server))
    }
  }

  pub fn connect(&mut self) -> Result<Session, String> {
    match &self.tcp {
      &Ok(ref tcp) => {
        if let Some(mut sess) = Session::new() {
          if let Err(error) = sess.handshake(&tcp) { println!("{}", error) };
          if let Err(error) = sess.userauth_agent(self.user) { println!("{}", error) };
          Ok(sess)
        } else {
          Err(String::from("Error getting session"))
        }
      }
      &Err(ref error) => Err(error.to_string()),
    }
  }

  pub fn execute(&mut self, sess: &Session, command: &str) -> String {
    match sess.channel_session() {
      Ok(mut channel) => {
        if let Err(error) = channel.exec(command) { println!("{}", error) };
        let mut s = String::new();
        if let Err(error) = channel.read_to_string(&mut s) { println!("{}", error) };
        format!("{}, exit code: {}", s, channel.exit_status().unwrap())
      }, Err(error) => error.to_string()
    }
  }

  pub fn upload_code(&mut self, sess: &Session, code: &str) -> String {
    let file_name = Uuid::new_v4();
    let content = code.as_bytes();
    match sess.scp_send(Path::new(&format!("/tmp/{}", file_name)), 0o644, content.len() as u64, None) {
      Ok(mut remote_file) => {
        if let Ok(_) = remote_file.write(content) {
          file_name.to_string()
        } else {
          String::from("Error loading file")
        }
      }
      Err(error) => error.to_string(),
    }
  }

  // to debug
  #[allow(dead_code)]
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
