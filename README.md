# 项目说明

> 项目使用`Dioxus+Tailwindcss`开发

## 启动编译命令

``` bash
# 启动命令
dx serve --platform desktop
# 编译命令
dx bundle --platform desktop
# 监听Tailwind
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
# 构建CSS文件
npm run build:css

```
