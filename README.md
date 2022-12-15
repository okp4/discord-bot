# 🤖 Discord bot

> Discord bot used @okp4 to bridge the Dataverse!

[![version](https://img.shields.io/github/v/release/okp4/discord-bot?style=for-the-badge&logo=github)](https://github.com/okp4/discord-bot/releases)
[![build](https://img.shields.io/github/workflow/status/okp4/discord-bot/Build?label=build&style=for-the-badge&logo=github)](https://github.com/okp4/discord-bot/actions/workflows/build.yml)
[![lint](https://img.shields.io/github/workflow/status/okp4/discord-bot/Lint?label=lint&style=for-the-badge&logo=github)](https://github.com/okp4/discord-bot/actions/workflows/lint.yml)
[![test](https://img.shields.io/github/workflow/status/okp4/discord-bot/Test?label=test&style=for-the-badge&logo=github)](https://github.com/okp4/discord-bot/actions/workflows/test.yml)
[![codecov](https://img.shields.io/codecov/c/github/okp4/discord-bot?style=for-the-badge&token=K5CYM8TQQY&logo=codecov)](https://codecov.io/gh/okp4/discord-bot)
[![conventional commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg?style=for-the-badge&logo=conventionalcommits)](https://conventionalcommits.org)
[![contributor covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg?style=for-the-badge)](https://github.com/okp4/.github/blob/main/CODE_OF_CONDUCT.md)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg?style=for-the-badge)](https://opensource.org/licenses/BSD-3-Clause)

## ✨ Purpose

This project is a general-purpose Discord bot around the [Cosmos ecosystem](https://cosmos.network) that bridges Discord and the blockchains of that ecosystem.

The aim of the bot is to cover a wide range of features:

- requesting tokens for a testnet Cosmos (aka as Faucet feature) ;
- consultation of blockchain transactions ;
- consultation of wallet balances ;
- consultation of application states of modules or smart contracts ;
- interaction with smart-contracts ;
- ...

This bot is currently used for the [OKP4 Discord](https://discord.gg/okp4) server, providing the many features that help us keep the server running smoothly while providing many services to our community. The aim is to provide the following range of features:

- consultation of data spaces, including governance rules
- consultation of the progress of a workflow execution
- consultation of dataset and service metadata
- ...

## 📄 How to use

### Command `start`

#### CLI

Boot 🚀 the discord bot.

```sh
./discord_bot start --help
```

```sh
discord_bot-start 
Boot the discord bot

USAGE:
    discord_bot start [OPTIONS]

OPTIONS:
    -g, --guild-id <GUILD_ID>
            The guild ID (Server ID)

    -h, --help
            Print help information

    -p, --prometheus-endpoint <PROMETHEUS_ENDPOINT>
            The prometheus endpoint. Optional. Configures an HTTP exporter that functions as a
            scrape endpoint for prometheus. The value is an IPv4 or IPv6 address and a port number,
            separated by a colon. For instance: 0.0.0.0:9000

    -t, --token <TOKEN>
            The discord token
```

#### Example

```sh
 start -t MTIzNDU2Nzg5MDEyMzQ1Njc4OQ.Zm9vYmFy.Fa_SK4L9Sdk4Ndk4Sl5ZLkrjs4fk456DHKsqED -g 1234567890123456789
```

#### Configuration

The bot is configured using a `toml` configuration file. Here is an example that configures the chain and the faucet (to be adapted to your needs):

```toml
[chain]
chain_id = "okp4-nemeton-1"
denom = "uknow"
grpc_address = "https://grpc.testnet.okp4.network"
prefix = "okp4"
batch_transaction_window = { secs = 15, nanos = 0 }

[faucet]
channel_id = 1034483599996620820
amount_send = 1000000
fee_amount = 0
gas_limit = 200000
memo = "Sent by økp4 discord bot 🤑"
explorer_url = "https://explore.okp4.network/okp4%20testnet/tx/{txhash}"
```

#### Prometheus Metrics

If you want to expose Prometheus metrics you need to set the `-p <PROMETHEUS_ENDPOINT>` to the command line:

```sh
 start -t MTIzNDU2Nzg5MDEyMzQ1Njc4OQ.Zm9vYmFy.Fa_SK4L9Sdk4Ndk4Sl5ZLkrjs4fk456DHKsqED -g 1234567890123456789 -p 127.0.0.1:9000
```

##### `discord_interactions_total`

- type: `counter`
- description: The total number of interactions received by the bot from Discord, labeled with: interaction, command.
- example:

```text
discord_interactions_total{interaction="application-command",command="ping"} 96
```

##### `discord_interactions_duration`

- type: `histogram`
- description: Timing statistics (percentiles) for Discord interaction processing durations, labeled with: interaction, command, quantile.
- example:

```text
discord_interactions_duration{interaction="application-command",command="ping",quantile="0"} 0.36191104
discord_interactions_duration{interaction="application-command",command="ping",quantile="0.5"} 0.3619316097353429
discord_interactions_duration{interaction="application-command",command="ping",quantile="0.9"} 0.3619316097353429
discord_interactions_duration{interaction="application-command",command="ping",quantile="0.95"} 0.3619316097353429
discord_interactions_duration{interaction="application-command",command="ping",quantile="0.99"} 0.3619316097353429
discord_interactions_duration{interaction="application-command",command="ping",quantile="0.999"} 0.3619316097353429
discord_interactions_duration{interaction="application-command",command="ping",quantile="1"} 0.36191104
```

## Prerequisites

Be sure you have [Rust](https://www.rust-lang.org/tools/install) properly installed with [cargo-make](https://github.com/sagiegurari/cargo-make).

## Build

```sh
cargo make
```

## Docker image 🐳

A docker image is also available. Usage example :

```bash
docker run okp4/discord-bot:latest start -t $TOKEN -g $GUILD_ID
```

## You want to get involved? 😍

Please check out OKP4 health files :

- [Contributing](https://github.com/okp4/.github/blob/main/CONTRIBUTING.md)
- [Code of conduct](https://github.com/okp4/.github/blob/main/CODE_OF_CONDUCT.md)
