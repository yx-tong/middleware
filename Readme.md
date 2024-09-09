## 环境变量

```sh
# 设置端口
export APP_LISTEN='0.0.0.0:8080'
# 设置 OpenAPI 根路径
export API_ENDPOINT='http://localhost:8080/api'
```

## 测试

```sh
cargo run
```

## 部署

```sh
cross build --target x86_64-unknown-linux-gnu --release
```

