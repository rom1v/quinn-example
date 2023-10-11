use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Instant;
use quinn::{Endpoint, ServerConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

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
    let endpoint = Endpoint::server(server_config, bind_addr)?;

    let connecting = endpoint.accept().await.unwrap(); // Option, not Result
    let conn = connecting.await?;

    println!("[server] [{}ms] connection accepted: addr={}", start.elapsed().as_millis(), conn.remote_address());

    println!("[server] [{}ms] waiting stream opening...", start.elapsed().as_millis());
    let _ = conn.accept_uni().await?;
    println!("[server] [{}ms] stream opened", start.elapsed().as_millis());

    // ... read the stream

    endpoint.wait_idle().await;

    Ok(())
}
