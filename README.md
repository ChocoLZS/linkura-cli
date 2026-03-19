# linkura-cli

[简体中文](docs/zh-CN/README.md) | [日本語](docs/ja/README.md)

This repository currently contains two main programs:
- `linkura-cli`
- `linkura-motion-cli`

## Repository Capabilities

### linkura-cli

1. Fetch recent live streaming information and recent archive information.
2. Fetch detailed information for a specific archive.
3. Start an MCP server.

#### MCP Server Capabilities

##### tools

- `list_live_streaming_info`: List current live streaming schedule entries
- `list_archives`: List archive information
- `get_archive_detail`: Fetch detailed information for a specific archive

### linkura-motion-cli

1. Supports **multi-threaded download** of motion capture data in `iarc` / `md` formats
2. Supports **multi-threaded upload** of motion capture data to a specified S3-compatible storage server
3. Supports **analyzing** motion capture packets and outputting analysis results
4. Supports **extracting** audio from motion capture packets and exporting it in opus format
5. Supports **converting** **live** motion capture packets into **archive** motion capture packets

## Repository Layout

- `bin/linkura-cli`
  - Main CLI program
  - Includes the MCP service implementation
- `bin/linkura-motion-cli`
  - Motion capture data related tool
- `crates/api`
  - High-level and low-level API wrappers
- `crates/packet`
  - Motion capture protocol and packet handling
- `crates/downloader`
  - Motion capture data download / upload logic
- `crates/common`
  - Shared helper functions and types
- `crates/i18n`
  - Internationalization support
- `deps/`
  - External metadata required for build-time code generation

## Build

### Dependencies

- A Rust toolchain compatible with this workspace
- `protoc`
- `cmake`
- When building `linkura-motion-cli` with the `audio` feature enabled, `libopus` is required. For detailed dependency requirements, refer to [opusic-sys](https://github.com/DoumanAsh/opusic-sys/)

### Steps

```bash
git clone https://github.com/ChocoLZS/linkura-cli.git
cd linkura-cli
git submodule update --init --recursive
cargo build -p linkura-cli
cargo build -p linkura-motion-cli
```

Examples:

```bash
cargo run -p linkura-cli
```

```bash
cargo run -p linkura-motion-cli
```

```bash
cargo run -p linkura-motion-cli --features audio
```

## Disclaimer

- This repository is an unofficial project and is not affiliated with the game operator or related rights holders.
- The repository contents are primarily intended for personal study, research, and tooling development.
- The author provides no warranty regarding correctness, completeness, long-term compatibility, or fitness for any particular purpose.
- Users are responsible for ensuring that their usage complies with local laws, platform rules, and third-party rights requirements.
