


## 重点检查

- 检查 [Cargo.lock](./Cargo.lock), 绝对禁止 `native-tls`

会导致

```txt
Function instance exited unexpectedly(code 127, message:key has expired) with start command '/var/fc/runtime/rloader'.
```

有可能通过 `reqwest`, `lettre` 等库间接引用.

