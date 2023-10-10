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
                let bytes_read = reader.read_line(&mut line).await.unwrap();

                if bytes_read == 0 {
                    break;
                }

                tx.send(line.clone()).unwrap();

                let msg = rx.recv().await.unwrap();

                writer.write_all(msg.as_bytes()).await.unwrap();
                line.clear();
            }
        });

    }
}
