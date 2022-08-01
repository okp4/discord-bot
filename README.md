# 🤖 Discord bot

> Discord bot used @okp4 to bridge the Dataverse!

[![version](https://img.shields.io/github/v/release/okp4/discord-bot?style=for-the-badge)](https://github.com/okp4/discord-bot/releases)
[![build](https://img.shields.io/github/workflow/status/okp4/discord-bot/Build?label=build&style=for-the-badge)](https://github.com/okp4/discord-bot/actions/workflows/build.yml)
[![lint](https://img.shields.io/github/workflow/status/okp4/discord-bot/Lint?label=lint&style=for-the-badge)](https://github.com/okp4/discord-bot/actions/workflows/lint.yml)
[![test](https://img.shields.io/github/workflow/status/okp4/discord-bot/Test?label=test&style=for-the-badge)](https://github.com/okp4/discord-bot/actions/workflows/test.yml)
[![codecov](https://img.shields.io/codecov/c/github/okp4/discord-bot?style=for-the-badge&token=K5CYM8TQQY)](https://codecov.io/gh/okp4/discord-bot)
[![conventional commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg?style=for-the-badge)](https://conventionalcommits.org)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg?style=for-the-badge)](https://opensource.org/licenses/BSD-3-Clause)

## ✨ Purpose

This project is a Discord bot mainly intended to be used for the [OKP4 Discord](https://discord.gg/okp4) server. It offers many features that help us keep the server running smoothly while providing many services to our community.

The project also has a wider ambition to provide a general-purpose bot around the [Cosmos ecosystem](https://cosmos.network) that bridges Discord and the blockchains of that ecosystem.
So stay tuned!

## 📄 How to use

### Command `start`

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
    -g <GUILD_ID>        The guild ID (Server ID)
    -h, --help           Print help information
    -t <TOKEN>           The discord token
```

## Prerequisites

Be sure you have [Rust](https://www.rust-lang.org/tools/install) properly installed with [cargo-make](https://github.com/sagiegurari/cargo-make).

## Build

```sh
cargo make
```
