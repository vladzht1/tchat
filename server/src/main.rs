// extern crate mio;

mod entities;
mod sockets;

// use mio::*;
// use entities::room::{RoomPool};
use entities::user::{User};
// use sockets::server::types::WebSocketServer;

// fn main() {
//   // let mut room_pool = RoomPool::new();

//   let mut event_loop = EventLoop::new().unwrap();
//   let mut handler = WebSocketServer;
//   event_loop.run(&mut handler).unwrap();
// }

use std::{
  collections::HashMap,
  env,
  io::Error as IoError,
  net::SocketAddr,
  sync::{Mutex, Arc},
};

use tokio::net::{TcpListener, TcpStream};

type PeerMap = Arc<Mutex<HashMap<SocketAddr, User>>>;

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
  println!("Incoming TCP connection from: {}", addr);

  let ws_stream = tokio_tungstenite::accept_async(raw_stream)
    .await
    .expect("Error during the websocket handshake occurred");
  println!("WebSocket connection established: {}", addr);

  peer_map.lock().unwrap().insert(addr, User::new(1, "Vlad".to_string(), addr));

  peer_map.lock().unwrap().iter().for_each(|user| println!("{:#?}", user.1));

  println!("{} disconnected", &addr);
  peer_map.lock().unwrap().remove(&addr);
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
  let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:2034".to_string());

  let state = PeerMap::new(Mutex::new(HashMap::new()));

  let try_socket = TcpListener::bind(&addr).await;
  let listener = try_socket.expect("Failed to bind");
  println!("Listening on: {}", addr);

  while let Ok((stream, addr)) = listener.accept().await {
    tokio::spawn(handle_connection(state.clone(), stream, addr));
  }

  Ok(())
}