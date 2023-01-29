# VS Code Config Helper v4 文档

## 关于本软件

本软件用于生成 Visual Studio Code（简称 VS Code）下，进行单文件 C++ 开发的开发环境。请注意：
- 本软件会少量修改计算机的全局设置/配置，请你知悉。
- 本软件**不适用于多文件 C++ 工程**。如你需要进行多文件开发，建议你使用 CMake、Xmake 等专业工具及配套扩展

## 使用指南

- 各系统配置说明
  - [Linux 配置说明](./linux)
  - [macOS 配置说明](./macos)
  - [Windows 配置说明](./windows)
- [配置选项说明](./options)
- [命令行界面](./cli)

## 本软件会修改的全局设置/配置

本软件会酌情安装/卸载 VS Code 扩展。
- 本软件总会安装微软开发的 [C/C++](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools) 扩展，以提供智能提示功能。如果你使用 GCC 或 MSVC 编译器，该扩展也将提供调试适配器功能。
- 如果你使用基于 LLVM 的编译器，本软件会安装 [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) 扩展，以提供调试适配器功能。
- 如果你未使用兼容模式，本软件会安装 [Console Pauser](https://marketplace.visualstudio.com/items?itemName=Guyutongxue.pause-console) 扩展，以提供外部弹窗运行功能。
- 如果你勾选了“卸载多余扩展”选项，本软件会**卸载**以下扩展：
  - Code Runner `formulahendry.code-runner`
  - C++ Intellisense `austin.code-gnu-global`
  - C/C++ Compile Run `danielpinto8zz6.c-cpp-compile-run`
  - C/C++ Clang Command Adapter `mitaki28.vscode-clang`
  - C/C++ GNU Global `jaycetyle.vscode-gnu-global`
  - C/C++ Runner `franneck94.c-cpp-runner`
  - Include Autocomplete `ajshort.include-autocomplete`
  - Clang-Format `xaver.clang-format`
  - C/C++ Advanced Lint `jbenden.c-cpp-flylint`

本软件会修改你的 VS Code 全局快捷键设置。
- 如果你未使用兼容模式，本软件会将“运行快捷键”（默认为 <kbd>F6</kbd>，可自行调整）绑定到外部弹窗运行任务。若该快捷键已存在内容，则原有设置**会被覆盖**。

本软件还会进行如下操作：
- 本软件默认会在数据目录（`AppData`、`Application Support`、`.local`）下生成用于调试和诊断的日志。如果你想禁用日志，请在 CLI 中指定 `-L /dev/null` 或 `-L NUL`。
- 本软件默认在配置成功时会触发 Count API，用于网站首页展示统计次数；在配置失败时会尝试将日志发送到开发者邮箱。如果你想禁用此行为，请在 CLI 中指定 `--no-stats` 或者在配置选项中关闭“发送统计数据”。
- 如果勾选了“将编译器添加到 Path”选项（MinGW 可用且默认勾选），则会修改你的用户环境变量 `Path`。本软件不会删除该环境变量的内容，但会添加（或调整）已选择的 MinGW 路径到顶部。
- 如果勾选了“添加桌面快捷方式”（仅 Windows 可用）选项，本软件会在你的桌面上创建指向工作文件夹的 VS Code 快捷方式。
- 如果勾选了“无法调试时警告”（仅 GCC MinGW 可用）选项，本软件会在数据目录下生成文件名检测脚本。
- 安装 Apple Clang 编译器时，软件会引导你执行 `xcode-select --install` 命令以安装 Xcode Command Line Tools。

除上述内容外，其它所有操作均在工作文件夹内完成。
