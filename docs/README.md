
## 在本项目内开发时的使用方式
```bash
# Extract package name from package.json
cargo run -- get --field name

# Strip quotes from string values
cargo run -- get --field name --strip-quotes
cargo run -- get --field version --strip-quotes
cargo run -- get --field description --strip-quotes
cargo run -- get --field repository.url --strip-quotes
cargo run -- get --field "keywords" --strip-quotes
cargo run -- get --field "author" --strip-quotes

cargo run -- set -f package.json -k version -v "1.1.0" --in-place
# 更新描述
cargo run -- set -f package.json -k description -v "An enhanced Node.js application" --in-place

# 更新作者
# cargo run -- set -f package.json -k author -v "Jane Smith <jane@example.com>" --in-place

# 更新许可证
# cargo run -- set -f package.json -k license -v "Apache-2.0" --in-place

```

## 安装后的二进制使用方式

```bash
i_edit_json get --field name --strip-quotes
i_edit_json get --field version --strip-quotes
i_edit_json get --field description --strip-quotes
i_edit_json get --field repository.url --strip-quotes
i_edit_json get --field "keywords" --strip-quotes
i_edit_json get --field "author" --strip-quotes

i_edit_json set -f package.json -k version -v "1.1.0" --in-place
# 更新描述
i_edit_json set -f package.json -k description -v "An enhanced Node.js application" --in-place

# 更新作者
# i_edit_json set -f package.json -k author -v "Jane Smith <jane@example.com>" --in-place

# 更新许可证
# i_edit_json set -f package.json -k license -v "Apache-2.0" --in-place
```

## demo for node package.json
```bash
# 获取项目名称
i_edit_json get -f package.json -k name --strip-quotes
# 输出: my-node-app

# 获取版本号
i_edit_json get -f package.json -k version --strip-quotes
# 输出: 1.0.0

# 获取描述
i_edit_json get -f package.json -k description --strip-quotes
# 输出: A sample Node.js application

# 获取入口文件
i_edit_json get -f package.json -k main --strip-quotes
# 输出: index.js

# 获取许可证
i_edit_json get -f package.json -k license --strip-quotes
# 输出: MIT
```

### get scripts
```bash
# 获取所有脚本
i_edit_json get -f package.json -k scripts --output json-pretty
# 输出:
# {
#   "start": "node index.js",
#   "dev": "nodemon index.js",
#   "test": "jest",
#   "build": "tsc",
#   "lint": "eslint ."
# }

# 获取特定脚本
i_edit_json get -f package.json -k scripts.start --strip-quotes
# 输出: node index.js

i_edit_json get -f package.json -k scripts.dev --strip-quotes
# 输出: nodemon index.js

# 获取脚本数量
i_edit_json get -f package.json -k scripts --output json | jq 'keys | length'
# 输出: 5
```

### get dependencies
```bash
# 获取生产依赖
i_edit_json get -f package.json -k dependencies --output json-pretty
# 输出:
# {
#   "express": "^4.18.2",
#   "axios": "^1.3.0",
#   "dotenv": "^16.0.3"
# }

# 获取开发依赖
i_edit_json get -f package.json -k devDependencies --output json-pretty
# 输出:
# {
#   "jest": "^29.5.0",
#   "nodemon": "^2.0.22",
#   "typescript": "^5.0.4",
#   "@types/node": "^20.2.5",
#   "@types/express": "^4.17.17"
# }

# 获取特定依赖版本
i_edit_json get -f package.json -k dependencies.express --strip-quotes
# 输出: ^4.18.2

i_edit_json get -f package.json -k devDependencies.typescript --strip-quotes
# 输出: ^5.0.4

# 统计依赖数量
echo "生产依赖: $(i_edit_json get -f package.json -k dependencies --output json | jq 'keys | length')"
echo "开发依赖: $(i_edit_json get -f package.json -k devDependencies --output json | jq 'keys | length')"
```

### get others
```bash
# 获取引擎要求
i_edit_json get -f package.json -k engines --output json-pretty
# 输出:
# {
#   "node": ">=16.0.0",
#   "npm": ">=8.0.0"
# }

# 获取仓库信息
i_edit_json get -f package.json -k repository.url --strip-quotes
# 输出: https://github.com/username/my-node-app.git

# 获取关键词
i_edit_json get -f package.json -k keywords --output json
# 输出: ["node","javascript","api"]

# 获取作者信息
i_edit_json get -f package.json -k author --strip-quotes
# 输出: John Doe <john@example.com>

```