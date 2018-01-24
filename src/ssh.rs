use std::io::prelude::*;
use std::net::{TcpStream};
use ssh2::Session;

pub struct Ssh {
  sess: Session
 }

impl Ssh {
  pub fn new() -> Ssh {
    Ssh { sess: Session::new().unwrap() }
  }

  pub fn connect(&mut self, server: String, user: &str) {
    let tcp = TcpStream::connect(server).unwrap();
    self.sess.handshake(&tcp).unwrap();
    self.sess.userauth_agent(user).unwrap();
  }

  pub fn execute(&self, command: &str) -> String {
    let mut channel = self.sess.channel_session().unwrap();
    channel.exec(command).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    channel.wait_close();
    format!("{}, exit code: {}", s, channel.exit_status().unwrap())
  }
}
