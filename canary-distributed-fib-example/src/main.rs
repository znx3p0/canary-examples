
use std::sync::Arc;

use canary::{Result, Channel, providers::Tcp, routes::GLOBAL_ROUTE, service::StaticService, Addr};
use srpc::{IntoClient, RwLock};
use structopt::StructOpt;

const ADDRS: &'static str = "127.0.0.1:8080";

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Node {
        #[structopt(short, long, parse(try_from_str = Addr::new))]
        addr: Addr
    },
    Cluster {
        #[structopt(short, long, parse(try_from_str = Addr::new))]
        addr: Addr
    },
    Client {
        #[structopt(short, long, parse(try_from_str = Addr::new))]
        addr: Addr,
        number: u64
    }
}

#[canary::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt.command {
        Command::Cluster { addr } => {
            println!("starting cluster at {:?}", addr);
            addr.bind().await?;
            GLOBAL_ROUTE.add_service_at::<DistributedFibCluster>("cluster", Arc::new(RwLock::new(Default::default())))?;
            std::future::pending::<()>().await;
        },
        Command::Node { addr } => {
            println!("starting node and connecting to {:?}", addr);
            let cluster = Tcp::connect(ADDRS, "cluster").await?.client::<DistributedFibCluster>();
            let cluster = cluster.insert_node().await?;
            <DistributedFibNode as StaticService>::introduce(Arc::new(DistributedFibNode), cluster).await?;
        },
        Command::Client { addr, number } => {
            println!("calculating fib of {} remotely", number);
            let mut cluster = addr.service("cluster").connect().await?.client::<DistributedFibCluster>();
            let res = cluster.calculate_fib(number).await?;
            println!("calculated `{res}` remotely");
        },
    }
<<<<<<< HEAD
=======

>>>>>>> 945f427 (added croc example)
    Ok(())
}



#[srpc::rpc]
#[derive(Default)]
struct DistributedFibCluster {
    nodes: Vec<DistributedFibNodePeer>,
}

#[srpc::rpc]
impl DistributedFibCluster {
    #[consume]
    async fn insert_node(&mut self, chan: Channel) -> Result<()> {
        let chan = chan.client::<DistributedFibNode>();
        self.nodes.push(chan);
        Ok(())
    }
    async fn calculate_fib(&mut self, num: u64) -> u64 {
        if self.nodes.is_empty() {
            println!("calculating fib locally");
            // no nodes :(
            // calculate fib locally instead
            return fibonacci(num)
        }
        let node_num = fastrand::usize(..self.nodes.len());
        let node = &mut self.nodes[node_num];
        println!("sending op to node");
        match node.fibonacci(num).await {
            Ok(res) => res,
            Err(_) => {
                println!("calculating fib locally and removing node");
                // error :(
                // calculate fib locally instead
                // and remove this node since it is unreliable
                self.nodes.remove(node_num);
                fibonacci(num)
            }
        }

    }
}

#[srpc::rpc]
struct DistributedFibNode;

#[srpc::rpc(none)]
impl DistributedFibNode {
    async fn fibonacci(&self, num: u64) -> u64 {
        println!("calculating fibonacci remotely");
        fibonacci(num)
    }
}

#[inline(always)]
fn fibonacci(number: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..number {
        let tmp = a;
        a = b;
        b = a + tmp;
    }
    b
}

