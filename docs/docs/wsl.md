# WSL 使用说明

## Step 0：确保 WSLg 可用

尽管本项目可以在命令行下运行，但弹窗运行功能仍然需要图形界面。建议您检查 WSLg 是否在您的宿主机上可用。访问 [WSLg 文档](https://github.com/microsoft/wslg)以获得更多信息。

## Step 1：下载程序

请在[网站下载处](https://vscch.guyutongxue.site/#download)点击 Linux 版 - 可执行文件。AppImage 不好使。

## Step 2：安装中文字体

修改 `/etc/locale.gen`，使得 `zh_CN.UTF-8 UTF-8` 行可用。

```sh
sudo locale-gen
sudo apt install fonts-wqy-microhei ttf-wqy-zenhei language-pack-zh-hans language-pack-gnome-zh-hans language-pack-kde-zh-hans manpages-zh
```

在 Windows 中重启 WSL 实例：

```powershell
wsl --shutdown
```

## Step 3：安装必需软件

```sh
# 终端模拟器，用于弹窗运行
sudo apt install gnome-terminal
```

## Step 4：启动程序并开始配置

