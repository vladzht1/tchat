use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::{TcpListener}, sync::broadcast};

mod entities;

#[tokio::main]
async fn main() {
  let listener = TcpListener::bind("localhost:5000").await.unwrap();

  let (tx, _rx) = broadcast::channel(10);

  loop {
    let (mut socket, addr) = listener.accept().await.unwrap();

    let tx = tx.clone();
    let mut rx = tx.subscribe();

    tokio::spawn(async move {
      let (reader, mut writer) = socket.split();

      let mut reader = BufReader::new(reader);
      let mut message = String::new();

      loop {
        tokio::select! {
          result = reader.read_line(&mut message) => {
            if result.unwrap() == 0 {
              break;
            }

            tx.send((message.clone(), addr)).unwrap();
            message.clear();
          }
          result = rx.recv() => {
            let (text, address) = result.unwrap();

            if address != addr {
              writer.write_all(text.as_bytes()).await.unwrap();
            }
          }
        }
      }
    });
  }
}