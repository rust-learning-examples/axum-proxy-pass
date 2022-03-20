# 启动
```
RUST_LOG=debug cargo run
```

# 使用

### 302 Redirect

```
// eg: http://localhost:3000/redirect/http://localhost:4000/api/v1/vue/hello?a=1&b=2
path: 'http://localhost:3000/redirect/__origin__'
```

### Proxy Request

```
// eg: http://localhost:3000/proxy/http://localhost:4000/api/v1/vue/hello?a=1&b=2
path: 'http://localhost:3000/proxy/__origin__'
```