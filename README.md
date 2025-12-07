# i_edit_json

[![Crates.io](https://img.shields.io/crates/v/i_edit_json)](https://crates.io/crates/i_edit_json)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.60%2B-blue.svg)](https://www.rust-lang.org)

一个轻量级、高性能的 JSON 字段提取和操作工具。

## 功能特性

- 使用直观的点分隔路径提取和修改 JSON 字段
- 支持嵌套结构和数组操作
- 类型感知的值处理
- 可作为 CLI 工具或 Rust 库使用
- 提供常见 JSON 操作的便捷函数

## 安装

### 从 crates.io 安装

```bash
cargo install i_edit_json
```

### 从源码安装

```bash
git clone https://github.com/your-username/i_edit_json
cd i_edit_json
cargo install --path .
```

```bash
git clone https://github.com/ymc-github/i_edit_json
cd i_edit_json
cargo install --path .
```

```bash
cargo install --git https://github.com/ymc-github/i_edit_json

cargo install --git https://github.com/ymc-github/i_edit_json --branch main

cargo install --git https://github.com/ymc-github/i_edit_json --tag v0.1.0
```

### 从 Docker Hub 安装
```bash
# 从 docker.io
# docker pull docker.io/ymc-github/i_edit_json:latest
docker pull ymc-github/i_edit_json:latest

# 从 ghcr.io
# ghcr.io/<owner>/<repo>:latest
docker pull ghcr.io/ymc-github/i_edit_json:latest
```

## 使用方法

### 作为命令行工具

#### 提取字段（get 命令）

```bash
# 从 package.json 提取包名
i_edit_json get --field name

# 去除字符串值的引号
i_edit_json get --field name --strip-quotes
i_edit_json get --field version --strip-quotes
i_edit_json get --field description --strip-quotes
i_edit_json get --field repository.url --strip-quotes
i_edit_json get --field "keywords" --strip-quotes
i_edit_json get --field "author" --strip-quotes

# 提取数组元素
i_edit_json get --field "keywords[0]" --strip-quotes

# 提取嵌套字段
i_edit_json get --field "dependencies.express" --strip-quotes

# 以 JSON 格式输出
i_edit_json get --field "scripts" --output json-pretty

# 提取多个字段
i_edit_json get -m name -m version -m description
```

#### 设置字段（set 命令）

```bash
# 设置版本号
i_edit_json set -f package.json -k version -v "1.1.0" --in-place

# 更新描述
i_edit_json set -f package.json -k description -v "An enhanced Node.js application" --in-place

# 更新作者
# i_edit_json set -f package.json -k author -v "Ye Mincong <cong@example.com>" --in-place

# 更新许可证
# i_edit_json set -f package.json -k license -v "Apache-2.0" --in-place

# 设置数组元素
i_edit_json set -k keywords[0] -v "rust" --in-place

# 指定值类型
i_edit_json set -k private -v "true" -t boolean --in-place

# 创建不存在的字段
i_edit_json set -k newField -v "value" --create-missing --in-place

# 设置数组元素（扩展数组）
i_edit_json set -k keywords[3] -v "cli" --create-missing --in-place
```

### 作为库使用

添加依赖到 `Cargo.toml`：

```toml
[dependencies]
i_edit_json = "0.1"
```

在代码中使用：

```rust
use i_edit_json::{get, set, ExtractConfig, SetConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 提取字段
    let get_config = ExtractConfig {
        file_path: "package.json".to_string(),
        field_path: "version".to_string(),
        output_format: None,
        strip_quotes: true,
    };
    let version = get::extract_field(&get_config)?;
    println!("当前版本: {}", version);

    // 设置字段
    let set_config = SetConfig {
        file_path: "package.json".to_string(),
        field_path: "version".to_string(),
        value: "1.2.3".to_string(),
        value_type: None,
        create_missing: false,
    };
    set::set_field_and_save(&set_config)?;
    println!("版本更新成功");

    // 提取多个字段
    let file_path = "package.json";
    let field_paths = vec!["name".to_string(), "version".to_string(), "description".to_string()];
    let result = get::extract_multiple_fields(file_path, &field_paths, true)?;
    
    for (field, value) in result.fields {
        println!("{}: {}", field, value);
    }

    Ok(())
}
```

## 更多示例

### 管理 package.json 依赖

```bash
# 添加新依赖
i_edit_json set -f package.json -k dependencies.express -v "^4.18.2" --create-missing --in-place

# 更新依赖版本
i_edit_json set -f package.json -k dependencies.axios -v "^1.4.0" --in-place

# 添加开发依赖
i_edit_json set -f package.json -k devDependencies.jest -v "^29.5.0" --create-missing --in-place

# 查看所有依赖
i_edit_json get -f package.json -k dependencies --output json-pretty
```

### 管理 npm 脚本

```bash
# 添加新脚本
i_edit_json set -f package.json -k scripts.build -v "tsc" --create-missing --in-place

# 更新现有脚本
i_edit_json set -f package.json -k scripts.test -v "jest --coverage" --in-place

# 查看所有脚本
i_edit_json get -f package.json -k scripts --output json-pretty
```

### 批量操作

```bash
# 提取关键信息
i_edit_json get -f package.json -m name -m version -m license -m main --output json

# 批量更新版本相关信息
i_edit_json set -f package.json -k version -v "2.0.0" --in-place
i_edit_json set -f package.json -k engines.node -v ">=18.0.0" --in-place
```

## 许可证

MIT OR Apache-2.0