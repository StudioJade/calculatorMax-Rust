# 字体说明

为了正确显示中文界面，本应用程序需要支持中文的字体文件。

## 重要说明

本程序不包含嵌入式字体文件。程序会自动使用系统默认字体，因此请确保您的系统已安装支持中文的字体。

## 推荐字体

1. **Windows系统**：
   - 微软雅黑 (Microsoft YaHei)
   - 宋体 (SimSun)
   - 黑体 (SimHei)
   
2. **Linux系统**：
   - Noto Sans CJK
   - WenQuanYi Micro Hei
   - Fira Code CN

3. **macOS系统**：
   - PingFang SC
   - Hiragino Sans GB
   - SF Pro SC

## 安装方法

### Windows系统
大多数Windows系统已经预装了所需的中文字体，无需额外安装。

### Linux系统
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install fonts-noto-cjk

# CentOS/RHEL/Fedora
sudo dnf install google-noto-cjk-fonts

# Arch Linux
sudo pacman -S noto-fonts-cjk
```

### macOS系统
macOS通常已经包含了支持中文的字体。

## 故障排除

如果程序界面中中文显示为方块（口口），请尝试以下解决方案：

1. **检查系统字体**：
   - Windows: 控制面板 > 外观和个性化 > 字体
   - Linux: fc-list :lang=zh
   - macOS: 系统偏好设置 > 通用 > 字体

2. **重启程序**：
   - 关闭程序后重新启动
   - 有时需要重启系统才能使新安装的字体生效

3. **更换显示语言**：
   - 在程序中切换到其他语言再切换回来

4. **检查系统区域设置**：
   - 确保系统区域设置支持中文显示

## 技术细节

本程序使用egui图形界面库，默认会使用系统字体。egui在Windows和macOS上有较好的字体支持，在Linux上可能需要手动安装额外的中文字体包。