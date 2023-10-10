use tokio::{net::TcpListener, io::{BufReader, AsyncBufReadExt, AsyncWriteExt}, sync::broadcast};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (_tx, mut _rx) = broadcast::channel::<String>(100);

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();
        let tx = _tx.clone();
        let mut rx = _tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    // broadcast message to all clients
                    res = reader.read_line(&mut line) => {
                        if res.unwrap() == 0 {
                            break;
                        }

                        tx.send(line.clone()).unwrap();
                        line.clear();
                    }
                    // receive message from broadcast
                    res = rx.recv() => {
                        let msg = res.unwrap();
                        writer.write_all(msg.as_bytes()).await.unwrap();
                    }
                }
            }
        });

    }
}
