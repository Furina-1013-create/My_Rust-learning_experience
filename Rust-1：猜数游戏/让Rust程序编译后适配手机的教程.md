# 在 Android 手机上运行 Rust 程序的步骤

## 方法 1：使用 Termux（推荐，最简单）

1. 在 Google Play Store 安装 Termux
2. 在 Termux 中安装 Rust：
   ```bash
   pkg update
   pkg install rust
   ```
3. 将你的代码复制到手机上
4. 在 Termux 中编译和运行：
   ```bash
   cd /storage/emulated/0/Download  # 或你的代码位置
   cargo run
   ```

## 方法 2：交叉编译（更复杂但性能更好）

### 前置要求：
1. 安装 Android Studio
2. 安装 Android NDK
3. 设置环境变量

### 编译步骤：
1. 添加 Android 目标：
   ```cmd
   rustup target add aarch64-linux-android
   ```

2. 配置链接器（在 .cargo/config.toml 中）：
   ```toml
   [target.aarch64-linux-android]
   linker = "路径/到/aarch64-linux-android21-clang.exe"
   ```

3. 编译：
   ```cmd
   cargo build --target aarch64-linux-android --release
   ```

4. 将生成的二进制文件传输到 Android 设备
5. 使用 adb 或文件管理器安装

## 方法 3：创建 Android 应用

使用 `cargo-mobile2` 创建真正的 Android 应用：

```cmd
cargo install cargo-mobile2
cargo mobile init
cargo android build
```

## 注意事项：

- Android 可执行文件需要特定权限
- 可能需要 root 权限才能运行原生可执行文件
- Termux 是最用户友好的选择
- 考虑将程序改写为 Android 应用以获得更好的用户体验
