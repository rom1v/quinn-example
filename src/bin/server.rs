use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Instant;
use quinn::{Endpoint, ServerConfig, VarInt};

async fn create_endpoint() -> Result<Endpoint, Box<dyn Error>> {
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

    Ok(endpoint)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let endpoint = create_endpoint().await?;

    let connecting = endpoint.accept().await.unwrap(); // Option, not Result
    let conn = connecting.await?;

    println!("[server] [{}ms] connection accepted: addr={}", start.elapsed().as_millis(), conn.remote_address());

    let conn2 = conn.clone();
    tokio::spawn(async move {
        println!("[server] [{}ms] waiting stream opening...", start.elapsed().as_millis());
        let res = conn2.accept_uni().await;
        if let Err(err) = res {
            println!("{err:?}");
        }
        println!("[server] [{}ms] stream opened", start.elapsed().as_millis());
    });

    conn.close(VarInt::from_u32(0), b"close");
    endpoint.wait_idle().await;
    drop(endpoint);

    println!("[server] Creating a new connection immediately");
    let endpoint = create_endpoint().await?;
    let connecting = endpoint.accept().await.unwrap(); // Option, not Result
    let conn = connecting.await?;
    conn.close(VarInt::from_u32(0), b"close");
    endpoint.wait_idle().await;

    Ok(())
}
