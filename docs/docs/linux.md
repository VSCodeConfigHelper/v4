# Linux 配置说明

## 重要说明

- 本软件仅适用于 x64 架构。
- 本软件只在 Ubuntu 下测试过。不保证本软件在其它发行版上是否可用。
- 如果你使用 WSL，建议你参考 CLI 配置说明。如果你倾向于使用图形界面，请：
  1. 确保你的 WSL 启用了 WSLg；
  2. 确保安装了中文字体。

> WSL/Ubuntu 下中文字体安装方法：
> 1. 修改 `/etc/locale.gen`，使得 `zh_CN.UTF-8 UTF-8` 行可用。
> 2. 执行以下命令：  
>   ```sh
>   sudo locale-gen
>   sudo apt install fonts-wqy-microhei ttf-wqy-zenhei > language-pack-zh-hans language-pack-gnome-zh-hans > language-pack-kde-zh-hans manpages-zh
>   ```
> 3. 在 Windows 中重启 WSL 实例：  
>   ```powershell
>   wsl --shutdown
>   ```

## 第一步：下载 VS Code

前往[官方网站](https://code.visualstudio.com/)获取 Linux 版 VS Code 并安装。

## 第二步：获取本软件

直接在[网站首页](https://v4.vscch.tk/)下载即可。Linux 版共有两种下载：
1. （默认）AppImage。体积较大，但保证可以在各大发行版运行。
2. 可执行文件。体积较小，但可能需要外部链接库才能正常运行。

下载完成后，请赋予其可执行权限。
```sh
chmod +x ./vscch_blah_blah_blah.AppImage
```

## 第三步：开始配置

双击运行，或在终端启动本程序。你将看到如下界面：

