
use canary::{service, Channel, Result, routes::GLOBAL_ROUTE, providers::Tcp};
use canary::igcp::serialization::formats::Json;

// #[canary::main]
// async fn main() -> Result<()> {
//     Tcp::bind("127.0.0.1:8888").await?;
//     GLOBAL_ROUTE.add_service_at::<ping_service>("ping", ())?;

//     let mut channel = Tcp::connect("127.0.0.1:8888", "ping").await?;
//     channel.send(vec![123u8; 8 * 1024]).await?;
//     std::future::pending().await
//     // Ok(())
// }

// #[service]
// async fn ping_service(mut channel: Channel) -> Result<()> {
//     let ping: Vec<u8> = channel.receive().await.unwrap();
//     println!("received length {}", ping.len());
//     Ok(())
// }

#[canary::main]
async fn main() -> Result<()> {
    Tcp::bind("127.0.0.1:8888").await?;
    GLOBAL_ROUTE.add_service_at::<ping_service>("ping", ())?;

    let mut channel = Tcp::connect("127.0.0.1:8888", "ping").await?;
    channel.send("Ping!").await?;
    let pong: String = channel.receive().await?;
    println!("received {pong}");
    Ok(())
}

#[service]
async fn ping_service(mut channel: Channel) -> Result<()> {
    let ping: String = channel.receive().await?;
    println!("received {}", ping);
    channel.send("Pong!").await?;
    Ok(())
}





