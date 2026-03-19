# linkura-cli

[English](../../README.md) | [日本語](../ja/README.md)

当前仓库主要包含两个程序：
- `linkura-cli`
- `linkura-motion-cli`


## 仓库功能

### linkura-cli

1. 获取最近的直播信息，以及最近的回放信息。
2. 获取指定的回放信息。
3. 启用mcp服务器

#### MCP服务器功能

##### tools

- list_live_streaming_info: 列出当前预告的信息
- list_archives: 列出回放信息
- get_archive_detail: 获取某一个回放的详细信息

### linkura-motion-cli

1. 支持**多线程下载** iarc/md 格式的动捕数据信息
2. 支持**多线程上传**动捕数据至指定S3兼容存储服务器
3. 支持**分析**动捕数据包并输出分析结果
4. 支持**提取**动捕数据包中的音频并输出为 opus 格式
5. 支持将**直播**动捕数据包**转换**为**回放**动捕数据包

## 仓库结构

- `bin/linkura-cli`
  - 主 CLI 程序
  - 包含 MCP 服务实现
- `bin/linkura-motion-cli`
  - 动捕数据相关工具
- `crates/api`
  - 高层与底层 API 封装
- `crates/packet`
  - 动捕数据协议与数据包处理
- `crates/downloader`
  - 动捕数据下载/上传相关逻辑
- `crates/common`
  - 通用辅助函数与类型
- `crates/i18n`
  - 多语言国际化支持
- `deps/`
  - 构建期生成代码所需的外部元数据

## 构建

### 依赖

- 与本工作区兼容的 Rust 工具链
- `protoc cmake`
- 当构建启用 `audio` 特性的 `linkura-motion-cli` 时，需要安装 `libopus`，具体依赖要求请参照此仓库 [opusic-sys](https://github.com/DoumanAsh/opusic-sys/)

### 步骤

```bash
git clone https://github.com/ChocoLZS/linkura-cli.git
cd linkura-cli
cargo build -p linkura-cli
cargo build -p linkura-motion-cli
```

示例：

```bash
cargo run -p linkura-cli
```

```bash
cargo run -p linkura-motion-cli
```

```bash
cargo run -p linkura-motion-cli --features audio
```

## 免责声明

- 本仓库为非官方项目，与游戏运营方及相关权利方无隶属关系。
- 仓库内容主要用于个人学习、研究与工具开发探索。
- 作者不对正确性、完整性、长期兼容性或特定用途适配性提供任何保证。
- 使用者需自行确保其使用方式符合当地法律法规、平台规则以及第三方权利要求。
