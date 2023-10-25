use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let cert_der = tokio::fs::read("tmp.cert").await?;

    let certificate = kynet::cert::Certificate::new(cert_der);
    let certs = kynet::cert::RootCertStore::with_single_cert(&certificate)?;
    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1234);
    let conn = kynet::Connection::quinn_connect(
        server_addr,
        "localhost",
        certs,
        &Default::default(),
    ).await?;

    //println!("[client] [{}ms] connected: addr={}", start.elapsed().as_millis(), conn.remote_address());
    println!("[client] [{}ms] connected", start.elapsed().as_millis());

    let mut send = conn.open_uni().await?;
    println!("[client] [{}ms] stream opened", start.elapsed().as_millis());

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    send.write_all("Hello, world!".as_bytes()).await?;
    println!("[client] [{}ms] data sent", start.elapsed().as_millis());

    send.close().await?;

    conn.close(0u32, "done");

    Ok(())
}
