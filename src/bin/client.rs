use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Instant;
use quinn::{ClientConfig, Endpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let cert_der = tokio::fs::read("tmp.cert").await?;

    let certificate = rustls::Certificate(cert_der);
    let mut certs = rustls::RootCertStore::empty();
    certs.add(&certificate)?;

    let client_config = ClientConfig::with_root_certificates(certs);

    let endpoint = {
        let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
        let mut endpoint = Endpoint::client(bind_addr)?;
        endpoint.set_default_client_config(client_config);
        endpoint
    };

    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1234);

    let conn = endpoint.connect(server_addr, "localhost")?.await?;
    println!("[client] [{}ms] connected: addr={}", start.elapsed().as_millis(), conn.remote_address());

    let mut send = conn.open_uni().await?;
    println!("[client] [{}ms] stream opened", start.elapsed().as_millis());

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    send.write_all("Hello, world!".as_bytes()).await?;
    println!("[client] [{}ms] data sent", start.elapsed().as_millis());

    send.finish().await?;

    conn.close(0u32.into(), b"done");

    endpoint.wait_idle().await;

    Ok(())
}
