use std::io::prelude::*;
use std::net::{TcpStream};
use std::path::Path;
use uuid::Uuid;

use ssh2::Session;

pub struct Ssh<'a> {
  sess: Session,
  user: &'a str,
  server: &'a str
 }

impl<'a> Ssh<'a> {
  pub fn new(user: &'a str, server: &'a str) -> Ssh<'a> {
    Ssh { 
      sess: Session::new().unwrap(),
      user: user,
      server: server
    }
  }

  fn connect(&mut self) {
    let tcp = TcpStream::connect(self.server).unwrap();
    self.sess.handshake(&tcp).unwrap();
    self.sess.userauth_agent(self.user).unwrap();
  }

  pub fn execute(&mut self, command: &str) -> String {
    self.connect();
    let mut channel = self.sess.channel_session().unwrap();
    channel.exec(command).unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    channel.wait_close().unwrap();
    format!("{}, exit code: {}", s, channel.exit_status().unwrap())
  }

  pub fn upload_code(&mut self, code: &str) -> String {
    self.connect();
    let file_name = Uuid::new_v4();
    let mut remote_file = self.sess.scp_send(Path::new(&format!("/tmp/{}", file_name)),
                                              0o644, 10, None).unwrap();
    remote_file.write(code.as_bytes()).unwrap();
    file_name.to_string()
  }
}
