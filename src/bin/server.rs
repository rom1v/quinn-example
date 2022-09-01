use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use quinn::{Endpoint, ServerConfig};
use futures_util::StreamExt;
use futures_util::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let names = vec!["localhost".into()];
    let cert = rcgen::generate_simple_self_signed(names)?;
    let cert_der = cert.serialize_der()?;
    let priv_key = cert.serialize_private_key_der();

    tokio::fs::write("tmp.cert", &cert_der).await?;

    let certificate = rustls::Certificate(cert_der);
    let private_key = rustls::PrivateKey(priv_key);

    let cert_chain = vec![certificate];

    let server_config = ServerConfig::with_single_cert(cert_chain, private_key)?;

    let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1234);
    let (endpoint, mut incoming) = Endpoint::server(server_config, bind_addr)?;

    let incoming_conn = incoming.next().await.unwrap(); // Option, not Result
    let mut new_conn = incoming_conn.await?;
    println!("[server] connection accepted: addr={}", new_conn.connection.remote_address());

    println!("[server] waiting stream opening...");
    let _stream = new_conn.bi_streams.try_next().await?;
    println!("[server] stream opened");

    // ... read the stream

    endpoint.wait_idle().await;

    Ok(())
}
