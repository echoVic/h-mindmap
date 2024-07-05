# H-MindMap 项目

这是一个用于渲染思维导图的项目，Rust 用于后端渲染，而 React 用于前端显示。

## 项目结构

### 目录结构
```
📦h-mindmap
┣ 📂.git                    # Git版本控制
┃ ┣ 📂branches              # Git分支
...[其他.git文件夹和文件]...
┣ 📂public                  # 公共资源文件
┃ ┣ 📜favicon.ico           # 网站图标
┃ ┣ 📜index.html            # 入口 HTML 文件
┃ ┣ 📜logo192.png           # 192x192 尺寸的logo
┃ ┣ 📜logo512.png           # 512x512 尺寸的logo
┃ ┣ 📜manifest.json         # PWA配置文件
┃ ┗ 📜robots.txt            # 爬虫协议文件
┣ 📂rust-render             # Rust 渲染逻辑
┃ ┣ 📂src
┃ ┃ ┣ 📜canvas.rs           # 画布管理
┃ ┃ ┣ 📜drawing.rs          # 绘图相关
┃ ┃ ┣ 📜events.rs           # 事件处理
┃ ┃ ┣ 📜lib.rs              # 入口库文件
┃ ┃ ┗ 📜models.rs           # 数据模型
┃ ┣ 📂target               # 编译生成文件
┃ ┃ ┣ ...                  # 具体内容省略
┃ ┗ 📜Cargo.toml            # Rust 项目配置
┣ 📂src                     # React 前端源码
┃ ┣ 📂components            # React 组件
┃ ┃ ┣ 📜Link.js             # 链接组件
┃ ┃ ┣ 📜MindMap.js          # 思维导图组件
┃ ┃ ┗ 📜Node.js             # 节点组件
┃ ┣ 📂store                 # 状态管理
┃ ┃ ┗ 📜index.js            # Redux或上下文API配置
┃ ┣ 📜App.css               # 应用样式
┃ ┣ 📜App.js                # 应用入口
┃ ┣ 📜App.test.js           # 测试文件
┃ ┣ 📜index.css             # 全局样式
┃ ┣ 📜index.js              # 应用主入口
┃ ┣ 📜logo.svg              # SVG 格式的 logo
┃ ┣ 📜reportWebVitals.js    # 性能监控
┃ ┗ 📜setupTests.js         # 测试配置
┣ 📜.eslintignore           # ESLint 忽略规则
┣ 📜.gitignore              # Git 忽略规则
┣ 📜README.md               # 项目说明文件
┣ 📜package.json            # 项目依赖及脚本
┗ 📜pnpm-lock.yaml          # 依赖锁定文件

```

### 相关命令

- `pnpm install` - 安装前端依赖
- `cargo build` - 编译 Rust 代码
- `pnpm start` - 启动 React 开发服务器
- `cargo run` - 运行 Rust 程序

## 项目说明

### 安装

1. 克隆项目仓库：
    ```bash
    git clone https://github.com/your-github-repo/h-mindmap.git
    cd h-mindmap
    ```

2. 安装前端依赖：
    ```bash
    pnpm install
    ```

3. 编译和运行 Rust 代码：
    ```bash
    cd rust-render
    cargo build
    cargo run
    ```

4. 启动 React 开发服务器：
    ```bash
    cd ..
    pnpm start
    ```

### 使用说明

1. 打开浏览器并访问 `http://localhost:3000`。
2. 在网页中操作思维导图，体验实时渲染效果。

### 贡献

欢迎提交问题和PRs！

### 许可

MIT License