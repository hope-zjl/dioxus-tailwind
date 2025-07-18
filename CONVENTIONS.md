# 项目规范及说明

## 项目规范

> 代码提交规范

- feat:(新功能) 增加了新的功能特性。
- fix:(bug修复) 修复了已知的bug。
- docs:(文档) 修改了文档，如README、注释等。
- style:(代码格式) 调整了代码格式，如空格、缩进、分号等，但不影响代码逻辑。
- refactor:(重构) 对代码进行了重构，既不是新增功能，也不是修复bug。
- perf:(性能优化) 对代码进行了性能优化。
- test:(测试) 增加了测试代码或修改了测试代码。
- chore:(构建过程或辅助工具的变动) 修改了构建脚本、依赖库等。
- revert:(回滚) 回滚到之前的某个提交版本。

## 项目说明

> DX命令说明

```shell
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
