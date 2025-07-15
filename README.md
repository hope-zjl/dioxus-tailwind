## 项目说明

> 项目使用`Dioxus+Tailwindcss`进行开发

## DX命令说明

```
build 构建 Dioxus 项目及其所有资源
translate 将源文件转换为 Dioxus 代码
serve 构建、监视并运行 Dioxus 项目及其所有资源
new 创建一个新的 Dioxus 项目
init 在当前目录（默认）初始化一个新的 Dioxus 项目，会尝试保持项目状态良好
clean 清理输出的构建产物
bundle 将 Dioxus 应用打包为可分发的对象
fmt 自动格式化 RSX 代码
check 检查项目是否存在问题
run 运行项目（不启用热重载）
config Dioxus 配置文件控制
help 显示本帮助信息或指定子命令的帮助

选项：
--verbose 使用详细输出模式 [默认值：false]
--trace 使用追踪输出模式 [默认值：false]
--json-output 以 JSON 格式输出日志
-h, --help 显示帮助
-V, --version 显示版本号

```

## 启动编译命令

``` bash
# 启动命令
dx serve --platform desktop
# 编译命令
dx bundle --platform desktop
# 监听Tailwind
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```