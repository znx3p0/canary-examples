#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::sync::Arc;
use canary::{Result, Channel, providers::Tcp, routes::GLOBAL_ROUTE, service::StaticService};
use srpc::{IntoClient, RwLock};
const ADDRS: &'static str = "127.0.0.1:8080";
fn main() -> Result<()> {
    async fn main() -> Result<()> {
        Tcp::bind(ADDRS).await?;
        GLOBAL_ROUTE.add_service_at::<DistributedFibCluster>(
            "cluster",
            Arc::new(RwLock::new(Default::default())),
        )?;
        let mut fib = Tcp::connect(ADDRS, "cluster")
            .await?
            .client::<DistributedFibCluster>();
        for i in 0..10 {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["adding node ", "\n"],
                    &match (&i,) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
            };
            let cluster = Tcp::connect(ADDRS, "cluster")
                .await?
                .client::<DistributedFibCluster>();
            let cluster = cluster.insert_node().await?;
            <DistributedFibNode as StaticService>::introduce(Arc::new(DistributedFibNode), cluster);
        }
        let res = fib.calculate_fib(5).await?;
        {
            ::std::io::_print(::core::fmt::Arguments::new_v1(
                &["", "\n"],
                &match (&res,) {
                    _args => [::core::fmt::ArgumentV1::new(
                        _args.0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ));
        };
        Ok(())
    }
    ::canary::runtime::block_on(main())
}
struct DistributedFibCluster {
    nodes: Vec<DistributedFibNodePeer>,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for DistributedFibCluster {
    #[inline]
    fn default() -> DistributedFibCluster {
        DistributedFibCluster {
            nodes: ::core::default::Default::default(),
        }
    }
}
impl ::srpc::canary::routes::RegisterEndpoint for DistributedFibCluster {
    const ENDPOINT: &'static str = "distributed_fib_cluster";
}
struct DistributedFibClusterPeer(pub ::srpc::canary::Channel, ::core::marker::PhantomData<()>);
impl From<::srpc::canary::Channel> for DistributedFibClusterPeer {
    fn from(c: ::srpc::canary::Channel) -> Self {
        DistributedFibClusterPeer(c, ::core::marker::PhantomData::default())
    }
}
impl ::srpc::Peer for DistributedFibCluster {
    type Struct = DistributedFibClusterPeer;
}
const _: () = {
    impl DistributedFibCluster {
        async fn insert_node(&mut self, chan: Channel) -> Result<()> {
            let chan = chan.client::<DistributedFibNode>();
            self.nodes.push(chan);
            Ok(())
        }
        async fn calculate_fib(&mut self, num: u32) -> u32 {
            if self.nodes.is_empty() {
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1(
                        &["calculating fib locally\n"],
                        &match () {
                            _args => [],
                        },
                    ));
                };
                return fibonacci(num);
            }
            let node_num = fastrand::usize(..self.nodes.len());
            let node = &mut self.nodes[node_num];
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["sending op to node\n"],
                    &match () {
                        _args => [],
                    },
                ));
            };
            match node.fibonacci(num).await {
                Ok(res) => res,
                Err(_) => {
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1(
                            &["calculating fib locally and removing node\n"],
                            &match () {
                                _args => [],
                            },
                        ));
                    };
                    self.nodes.remove(node_num);
                    fibonacci(num)
                }
            }
        }
    }
    #[allow(non_camel_case_types)]
    #[repr(u8)]
    enum __srpc_action {
        calculate_fib,
        insert_node,
    }
    impl serde::Serialize for __srpc_action {
        #[allow(clippy::use_self)]
        fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let value: u8 = match *self {
                __srpc_action::calculate_fib => __srpc_action::calculate_fib as u8,
                __srpc_action::insert_node => __srpc_action::insert_node as u8,
            };
            serde::Serialize::serialize(&value, serializer)
        }
    }
    impl<'de> serde::Deserialize<'de> for __srpc_action {
        #[allow(clippy::use_self)]
        fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct discriminant;
            #[allow(non_upper_case_globals)]
            impl discriminant {
                const calculate_fib: u8 = __srpc_action::calculate_fib as u8;
                const insert_node: u8 = __srpc_action::insert_node as u8;
            }
            match <u8 as serde::Deserialize>::deserialize(deserializer)? {
                discriminant::calculate_fib => {
                    core::result::Result::Ok(__srpc_action::calculate_fib)
                }
                discriminant::insert_node => core::result::Result::Ok(__srpc_action::insert_node),
                other => core::result::Result::Err(serde::de::Error::custom(
                    ::core::fmt::Arguments::new_v1(
                        &["invalid value: ", ", expected ", " or "],
                        &match (
                            &other,
                            &discriminant::calculate_fib,
                            &discriminant::insert_node,
                        ) {
                            _args => [
                                ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(_args.2, ::core::fmt::Display::fmt),
                            ],
                        },
                    ),
                )),
            }
        }
    }
    impl ::srpc::canary::service::Service for DistributedFibCluster {
        const ENDPOINT: &'static str = "distributed_fib_cluster";
        type Pipeline = ();
        type Meta = ::std::sync::Arc<::srpc::RwLock<DistributedFibCluster>>;
        fn service(
            __srpc_inner_meta: ::std::sync::Arc<::srpc::RwLock<DistributedFibCluster>>,
        ) -> Box<dyn Fn(::srpc::canary::igcp::BareChannel) + Send + Sync + 'static> {
            ::canary::service::run_metadata(
                __srpc_inner_meta,
                |__srpc_inner_meta: ::std::sync::Arc<::srpc::RwLock<DistributedFibCluster>>,
                 mut __srpc_inner_channel: ::srpc::canary::Channel| async move {
                    loop {
                        match __srpc_inner_channel.receive::<__srpc_action>().await? {
                            __srpc_action::insert_node => {
                                return __srpc_inner_meta
                                    .write()
                                    .await
                                    .insert_node(__srpc_inner_channel)
                                    .await;
                            }
                            __srpc_action::calculate_fib => {
                                #[allow(unused_parens)]
                                let (num): (u32) = __srpc_inner_channel.receive().await?;
                                let res = __srpc_inner_meta.write().await.calculate_fib(num).await;
                                __srpc_inner_channel.send(res).await?;
                            }
                        }
                    }
                },
            )
        }
    }
    impl ::srpc::canary::service::StaticService for DistributedFibCluster {
        type Meta = ::std::sync::Arc<::srpc::RwLock<DistributedFibCluster>>;
        type Chan = ::srpc::canary::Channel;
        fn introduce(
            __srpc_inner_meta: ::std::sync::Arc<::srpc::RwLock<DistributedFibCluster>>,
            mut __srpc_inner_channel: ::srpc::canary::Channel,
        ) -> ::srpc::canary::runtime::JoinHandle<::srpc::canary::Result<()>> {
            ::srpc::canary::runtime::spawn(async move {
                loop {
                    match __srpc_inner_channel.receive::<__srpc_action>().await? {
                        __srpc_action::insert_node => {
                            return __srpc_inner_meta
                                .write()
                                .await
                                .insert_node(__srpc_inner_channel)
                                .await;
                        }
                        __srpc_action::calculate_fib => {
                            #[allow(unused_parens)]
                            let (num): (u32) = __srpc_inner_channel.receive().await?;
                            let res = __srpc_inner_meta.write().await.calculate_fib(num).await;
                            __srpc_inner_channel.send(res).await?;
                        }
                    }
                }
            })
        }
    }
    impl DistributedFibClusterPeer {
        pub async fn insert_node(mut self) -> ::srpc::canary::Result<::srpc::canary::Channel> {
            self.0.send(__srpc_action::insert_node).await?;
            Ok(self.0)
        }
        pub async fn calculate_fib(
            &mut self,
            num: impl std::borrow::Borrow<u32>,
        ) -> ::srpc::canary::Result<u32> {
            self.0.send(__srpc_action::calculate_fib).await?;
            #[allow(unused_parens)]
            self.0.send((num.borrow())).await?;
            self.0.receive().await
        }
    }
};
struct DistributedFibNode;
impl ::srpc::canary::routes::RegisterEndpoint for DistributedFibNode {
    const ENDPOINT: &'static str = "distributed_fib_node";
}
struct DistributedFibNodePeer(pub ::srpc::canary::Channel, ::core::marker::PhantomData<()>);
impl From<::srpc::canary::Channel> for DistributedFibNodePeer {
    fn from(c: ::srpc::canary::Channel) -> Self {
        DistributedFibNodePeer(c, ::core::marker::PhantomData::default())
    }
}
impl ::srpc::Peer for DistributedFibNode {
    type Struct = DistributedFibNodePeer;
}
const _: () = {
    impl DistributedFibNode {
        async fn fibonacci(&self, num: u32) -> u32 {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["calculating fibonacci remotely\n"],
                    &match () {
                        _args => [],
                    },
                ));
            };
            fibonacci(num)
        }
    }
    #[allow(non_camel_case_types)]
    #[repr(u8)]
    enum __srpc_action {
        fibonacci,
    }
    impl serde::Serialize for __srpc_action {
        #[allow(clippy::use_self)]
        fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let value: u8 = match *self {
                __srpc_action::fibonacci => __srpc_action::fibonacci as u8,
            };
            serde::Serialize::serialize(&value, serializer)
        }
    }
    impl<'de> serde::Deserialize<'de> for __srpc_action {
        #[allow(clippy::use_self)]
        fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct discriminant;
            #[allow(non_upper_case_globals)]
            impl discriminant {
                const fibonacci: u8 = __srpc_action::fibonacci as u8;
            }
            match <u8 as serde::Deserialize>::deserialize(deserializer)? {
                discriminant::fibonacci => core::result::Result::Ok(__srpc_action::fibonacci),
                other => core::result::Result::Err(serde::de::Error::custom(
                    ::core::fmt::Arguments::new_v1(
                        &["invalid value: ", ", expected "],
                        &match (&other, &discriminant::fibonacci) {
                            _args => [
                                ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Display::fmt),
                            ],
                        },
                    ),
                )),
            }
        }
    }
    impl ::srpc::canary::service::Service for DistributedFibNode {
        const ENDPOINT: &'static str = "distributed_fib_node";
        type Pipeline = ();
        type Meta = ::std::sync::Arc<DistributedFibNode>;
        fn service(
            __srpc_inner_meta: ::std::sync::Arc<DistributedFibNode>,
        ) -> Box<dyn Fn(::srpc::canary::igcp::BareChannel) + Send + Sync + 'static> {
            ::canary::service::run_metadata(
                __srpc_inner_meta,
                |__srpc_inner_meta: ::std::sync::Arc<DistributedFibNode>,
                 mut __srpc_inner_channel: ::srpc::canary::Channel| async move {
                    loop {
                        match __srpc_inner_channel.receive::<__srpc_action>().await? {
                            __srpc_action::fibonacci => {
                                #[allow(unused_parens)]
                                let (num): (u32) = __srpc_inner_channel.receive().await?;
                                let res = __srpc_inner_meta.fibonacci(num).await;
                                __srpc_inner_channel.send(res).await?;
                            }
                        }
                    }
                },
            )
        }
    }
    impl ::srpc::canary::service::StaticService for DistributedFibNode {
        type Meta = ::std::sync::Arc<DistributedFibNode>;
        type Chan = ::srpc::canary::Channel;
        fn introduce(
            __srpc_inner_meta: ::std::sync::Arc<DistributedFibNode>,
            mut __srpc_inner_channel: ::srpc::canary::Channel,
        ) -> ::srpc::canary::runtime::JoinHandle<::srpc::canary::Result<()>> {
            ::srpc::canary::runtime::spawn(async move {
                loop {
                    match __srpc_inner_channel.receive::<__srpc_action>().await? {
                        __srpc_action::fibonacci => {
                            #[allow(unused_parens)]
                            let (num): (u32) = __srpc_inner_channel.receive().await?;
                            let res = __srpc_inner_meta.fibonacci(num).await;
                            __srpc_inner_channel.send(res).await?;
                        }
                    }
                }
            })
        }
    }
    impl DistributedFibNodePeer {
        pub async fn fibonacci(
            &mut self,
            num: impl std::borrow::Borrow<u32>,
        ) -> ::srpc::canary::Result<u32> {
            self.0.send(__srpc_action::fibonacci).await?;
            #[allow(unused_parens)]
            self.0.send((num.borrow())).await?;
            self.0.receive().await
        }
    }
};
#[inline(always)]
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
