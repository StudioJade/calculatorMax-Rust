[![CI](https://github.com/StudioJade/calculatorMax-Rust/actions/workflows/ci.yml/badge.svg)](https://github.com/StudioJade/calculatorMax-Rust/actions/workflows/ci.yml)  
# 计算器 Max - Rust 版本

一个功能强大的计算器应用程序，使用 Rust 编写，具有图形用户界面和广泛的数学功能。

## 语言版本 (Language Versions)

- [简体中文](README.md)
- [繁體中文（台灣）](README_TW.md)
- [繁體中文（香港）](README_HK.md)
- [English](README_EN.md)
- [Русский](README_RU.md)
- [Cat Language (喵星语)](README_CAT.md)

## 功能特色

- **安全表达式评估**：使用 `meval` crate 进行安全的数学表达式解析
- **丰富数学函数**：支持三角函数、对数函数和其他高级函数
- **几何计算**：三角形、矩形、圆形和梯形的面积计算
- **特殊公式**：海伦公式计算三角形面积、毕达哥拉斯定理
- **历史记录追踪**：保存先前的计算记录
- **可配置安全性**：可在安全模式和扩展模式间切换
- **现代化图形界面**：使用 egui 建立响应式用户界面

## 安装说明

### 必要条件

- Rust 工具链（建议使用最新稳定版本）

### 构建项目

```bash
cargo build --release
```

### 运行程序

```bash
cargo run
```

## 使用方式

1. 在输入字段中输入数学表达式
2. 点击"计算"或按下 Enter 键
3. 在结果字段中查看结果
4. 使用"历史记录"按钮查看先前的计算
5. 切换"安全模式"以控制运算限制

## 数学函数

计算器支持广泛的函数：

- 基本运算：`+`、`-`、`*`、`/`、`%`、`^`
- 常数：`pi`、`e`
- 三角函数：`sin`、`cos`、`tan` 等
- 对数函数：`log`、`log10`、`ln`

## 架构设计

应用程序分为几个模块：

- `calculator`：核心计算逻辑
- `ui`：图形用户界面
- `config`：应用程序设置

## 许可证

MIT
