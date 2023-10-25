use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let names = vec!["localhost".into()];
    let cert = rcgen::generate_simple_self_signed(names)?;
    let cert_der = cert.serialize_der()?;
    let priv_key = cert.serialize_private_key_der();

    tokio::fs::write("tmp.cert", &cert_der).await?;

    let certificate = kynet::cert::Certificate::new(cert_der);
    let private_key = kynet::cert::PrivateKey::new(priv_key);

    let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1234);
    let server = kynet::Connection::quinn_start_server_on_addr(
        bind_addr,
        certificate,
        private_key,
        &Default::default(),
    )?;
    let conn = server.accept().await?;

    //println!("[server] [{}ms] connection accepted: addr={}", start.elapsed().as_millis(), conn.remote_address());
    println!(
        "[server] [{}ms] connection accepted",
        start.elapsed().as_millis()
    );

    println!(
        "[server] [{}ms] waiting stream opening...",
        start.elapsed().as_millis()
    );
    let mut uni = conn.accept_uni().await?;
    println!("[server] [{}ms] stream opened", start.elapsed().as_millis());

    // ... read the stream

    while uni.read(&mut [0; 1024]).await?.is_some() {}
    Ok(())
}
