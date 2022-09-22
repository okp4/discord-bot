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

This project is a Discord bot mainly intended to be used for the [OKP4 Discord](https://discord.gg/okp4) server. It offers many features that help us keep the server running smoothly while providing many services to our community.

The project also has a wider ambition to provide a general-purpose bot around the [Cosmos ecosystem](https://cosmos.network) that bridges Discord and the blockchains of that ecosystem.
So stay tuned!

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
