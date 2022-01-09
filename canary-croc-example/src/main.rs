use canary::routes::GLOBAL_ROUTE;
use canary::Addr;
use canary::Result;
use srpc::IntoClient;
use srpc::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
enum Opt {
    Send {
        #[structopt(short, long, parse(try_from_str = Addr::new))]
        addr: Addr,
        path: String,
    },
    Cluster {
        #[structopt(short, long, parse(try_from_str = Addr::new))]
        addr: Addr,
    },
    Get {
        #[structopt(short, long, parse(try_from_str = Addr::new))]
        addr: Addr,
        room: String,
    },
}

#[canary::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::Cluster { addr } => {
            addr.bind().await?;
            let meta = Arc::new(RwLock::new(CrocCluster::default()));
            GLOBAL_ROUTE.add_service_at::<CrocCluster>("cluster", meta)?;
            std::future::pending().await
        }
        Opt::Send { addr, path } => {
            println!("connecting to cluster and sending {path}");
            let mut cluster = addr
                .service("cluster")
                .connect()
                .await?
                .client::<CrocCluster>();
            let file = async_std::fs::read(&path).await?;
            let room = cluster.send_file(path, file).await?;
            println!("room: `{}`", room);
            Ok(())
        }
        Opt::Get { addr, room } => {
            println!("connecting to cluster in room {room}");
            let mut cluster = addr
                .service("cluster")
                .connect()
                .await?
                .client::<CrocCluster>();
            match cluster.get_file(room).await? {
                Some((name, file)) => {
                    async_std::fs::write(name, file).await?;
                },
                None => panic!("file not found :(")
            };
            Ok(())
        },
    }
}

#[srpc::rpc]
#[derive(Default)]
struct CrocCluster {
    rooms: HashMap<String, Option<(String, Vec<u8>)>>,
}

#[srpc::rpc]
impl CrocCluster {
    async fn send_file(&mut self, name: String, file: Vec<u8>) -> String {
        let room = parity_wordlist::random_phrase(5);
        self.rooms.insert(room.clone(), Some((name, file)));
        room
    }
    async fn get_file(&mut self, room: String) -> Option<(String, Vec<u8>)> {
        let file = self.rooms.get_mut(&room)?.take()?;
        Some(file)
    }
}


