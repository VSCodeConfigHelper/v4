# VS Code Config Helper (v4)

全新版本的 VS Code C++ 配置器，基于 Tauri（Rust）和 Svelte.js。

## 开发

安装 Node.js 和 Rust 环境。安装 `pnpm` 作为包管理器。

### Linux dependencies

```sh
sudo apt install libwebkit2gtk-4.0-dev libssl-dev libgtk-3-dev libappindicator3-dev librsvg2-dev
```

```sh
# 安装包依赖
pnpm i

# 开发（热更新）
pnpm tauri dev

# 编译
pnpm tauri build
```
