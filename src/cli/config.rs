//! DiscordBot Config
use std::net::SocketAddr;
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(default)]
/// Configuration type for the discord bot
pub struct DiscordBotConfig {
    /// discord section
    pub discord: DiscordSection,

    /// Metrics configuration
    pub metrics: MetricsSection,

    /// Chain configuration
    pub chain: ChainSection,

    /// Faucet configuration
    pub faucet: FaucetSection,
}

/// Discord section.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct DiscordSection {
    /// Token
    pub token: String,

    /// Guild ID (Server ID)
    pub guild_id: u64,

    /// Configure the sharding strategy for this process
    pub sharding: DiscordShardingSection,
}

impl Default for DiscordSection {
    fn default() -> Self {
        Self {
            token: "".to_owned(),
            guild_id: 0,
            sharding: DiscordShardingSection::default(),
        }
    }
}

/// Sharding strategy configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DiscordShardingSection {
    /// Shard index (default 0)
    pub shard: u64,
    /// Number of total shards (default 1)
    pub shards: u64,
}

impl Default for DiscordShardingSection {
    fn default() -> Self {
        Self {
            shard: 0,
            shards: 1,
        }
    }
}

/// Metrics configuration section.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MetricsSection {
    /// The address used for the Prometheus metrics endpoint.
    pub endpoint: Option<SocketAddr>,

    /// The refresh duration for system metrics (process).
    pub refresh: Duration,
}

impl Default for MetricsSection {
    fn default() -> Self {
        Self {
            endpoint: None,
            refresh: Duration::from_secs(1),
        }
    }
}

/// Chain configuration section
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ChainSection {
    /// The chain okp4 server url.
    pub grpc_address: String,

    /// The network chain ID.
    pub chain_id: String,

    /// Token denom.
    pub denom: String,

    /// Address prefix.
    pub prefix: String,

    /// Duration between two transaction batch.
    pub batch_transaction_window: Duration,

    /// Configure the maximum
    pub max_msg: usize,
}

impl Default for ChainSection {
    fn default() -> Self {
        Self {
            grpc_address: "http://[::1]:9090".to_string().parse().unwrap(),
            chain_id: "localnet-okp4-1".to_string(),
            denom: "know".to_string(),
            prefix: "okp4".to_string(),
            batch_transaction_window: Duration::from_secs(8),
            max_msg: 7,
        }
    }
}

/// Faucet configuration section
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct FaucetSection {
    /// The sender mnemonic.
    pub mnemonic: String,

    /// Fee amount.
    pub fee_amount: i64,

    /// Number of token sent
    pub amount_send: i64,

    /// Transaction description
    pub memo: String,

    /// Gas limit
    pub gas_limit: u64,

    /// Discord channel ID used for transactions feedback
    pub channel_id: u64,
}

impl Default for FaucetSection {
    fn default() -> Self {
        Self {
            mnemonic: "".to_string(),
            fee_amount: 0,
            amount_send: 1,
            memo: "Sent by økp4 discord bot".to_string(),
            gas_limit: 200000,
            channel_id: 123456789123,
        }
    }
}
