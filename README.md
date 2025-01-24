# 计算器项目

## 项目概述
这是一个用Rust编写的安全计算器项目，旨在提供一个可靠且安全的数学表达式计算工具。项目采用模块化设计，具有良好的可扩展性和可维护性。

## 主要功能
- 支持基本四则运算（加、减、乘、除）
- 支持括号优先级
- 支持浮点数运算
- 完善的错误处理机制
- 单元测试覆盖

## 项目结构
```
safety/
├── src/
│   ├── main.rs
│   ├── math/
│   │   ├── mod.rs
│   │   ├── calculator.rs
│   │   ├── tokenizer.rs
│   │   └── parser.rs
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## 使用方法
1. 克隆项目：
   ```bash
   git clone https://github.com/liweiyuan/safety.git
   ```

2. 进入项目目录：
   ```bash
   cd safety
   ```

3. 运行计算器：
   ```bash
   cargo run
   ```

4. 运行测试：
   ```bash
   cargo test
   ```

## 依赖
- Rust 1.65+
- Cargo

## 贡献指南
欢迎提交issue和pull request。请确保：
- 代码风格一致
- 添加相应的单元测试
- 更新相关文档

## 许可证
MIT License
