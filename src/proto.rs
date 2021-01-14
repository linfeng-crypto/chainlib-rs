/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const COSMOS_SDK_VERSION: &str = include_str!("proto/COSMOS_SDK_COMMIT");
pub const TENDERMINT_VERSION: &str = include_str!("proto/TENDERMINT_COMMIT");

/// Cosmos protobuf definitions.
pub mod cosmos {
    /// Authentication of accounts and transactions.
    pub mod auth {
        pub mod v1beta1 {
            include!("proto/cosmos.auth.v1beta1.rs");
        }
    }

    pub mod bank {
        pub mod v1beta1 {
            include!("proto/cosmos.bank.v1beta1.rs");
        }
    }

    /// Proof-of-Stake layer for public blockchains.
    pub mod staking {
        pub mod v1beta1 {
            include!("proto/cosmos.staking.v1beta1.rs");
        }
    }

    /// Base functionality.
    pub mod base {
        /// Application BlockChain Interface (ABCI).
        ///
        /// Interface that defines the boundary between the replication engine
        /// (the blockchain), and the state machine (the application).
        pub mod abci {
            pub mod v1beta1 {
                include!("proto/cosmos.base.abci.v1beta1.rs");
            }
        }

        /// Key-value pairs.
        pub mod kv {
            pub mod v1beta1 {
                include!("proto/cosmos.base.kv.v1beta1.rs");
            }
        }

        /// Query support.
        pub mod query {
            pub mod v1beta1 {
                include!("proto/cosmos.base.query.v1beta1.rs");
            }
        }

        /// Reflection support.
        pub mod reflection {
            pub mod v1beta1 {
                include!("proto/cosmos.base.reflection.v1beta1.rs");
            }
        }

        /// Snapshots containing Tendermint state sync info.
        pub mod snapshots {
            pub mod v1beta1 {
                include!("proto/cosmos.base.snapshots.v1beta1.rs");
            }
        }

        /// Data structure that holds the state of the application.
        pub mod store {
            pub mod v1beta1 {
                include!("proto/cosmos.base.store.v1beta1.rs");
            }
        }

        pub mod v1beta1 {
            include!("proto/cosmos.base.v1beta1.rs");
        }
    }

    /// Cryptographic primitives.
    pub mod crypto {
        /// Multi-signature support.
        pub mod multisig {
            pub mod v1beta1 {
                include!("proto/cosmos.crypto.multisig.v1beta1.rs");
            }
        }
    }

    /// Transactions.
    pub mod tx {
        /// Transaction signing support.
        pub mod signing {
            pub mod v1beta1 {
                include!("proto/cosmos.tx.signing.v1beta1.rs");
            }
        }

        pub mod v1beta1 {
            include!("proto/cosmos.tx.v1beta1.rs");
        }
    }
}

/// IBC protobuf definitions.
pub mod ibc {
    /// IBC applications.
    pub mod applications {
        /// Transfer support.
        pub mod transfer {
            pub mod v1 {
                include!("proto/ibc.applications.transfer.v1.rs");
            }
        }
    }

    /// IBC core.
    pub mod core {
        /// IBC channels.
        pub mod channel {
            pub mod v1 {
                include!("proto/ibc.core.channel.v1.rs");
            }
        }

        /// IBC client.
        pub mod client {
            pub mod v1 {
                include!("proto/ibc.core.client.v1.rs");
            }
        }

        /// IBC commitments.
        pub mod commitment {
            pub mod v1 {
                include!("proto/ibc.core.commitment.v1.rs");
            }
        }

        /// IBC connections.
        pub mod connection {
            pub mod v1 {
                include!("proto/ibc.core.connection.v1.rs");
            }
        }

        /// IBC types.
        pub mod types {
            pub mod v1 {
                include!("proto/ibc.core.types.v1.rs");
            }
        }
    }

    /// IBC light clients.
    pub mod lightclients {
        pub mod localhost {
            pub mod v1 {
                include!("proto/ibc.lightclients.localhost.v1.rs");
            }
        }
        pub mod solomachine {
            pub mod v1 {
                include!("proto/ibc.lightclients.solomachine.v1.rs");
            }
        }
        pub mod tendermint {
            pub mod v1 {
                include!("proto/ibc.lightclients.tendermint.v1.rs");
            }
        }
    }
}

/// ICS23 protobuf definitions.
pub mod ics23 {
    include!("proto/ics23.rs");
}

pub mod tendermint {
    pub mod abci {
        include!("proto/tendermint.abci.rs");
    }
    pub mod crypto {
        include!("proto/tendermint.crypto.rs");
    }
    pub mod p2p {
        include!("proto/tendermint.p2p.rs");
    }
    pub mod types {
        include!("proto/tendermint.types.rs");
    }
    pub mod version {
        include!("proto/tendermint.version.rs");
    }
    pub mod rpc {
        pub mod grpc {
            include!("proto/tendermint.rpc.grpc.rs");
        }
    }
}
