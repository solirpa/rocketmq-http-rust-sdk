# MQ RUST HTTP SDK  
Aliyun MQ Documents: http://www.aliyun.com/product/ons

Aliyun MQ Console: https://ons.console.aliyun.com


## Use

1. setup rust
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. add dependencies in `cargo.toml`
```toml
[dependencies]
mq-http-rust-sdk = "^0.11"
```

### Run Example

Run the following command to start the example:

```sh
# send message via producer
cargo run --example producer

# consume message via consumer
cargo run --example consumer
```

## 参考项目

[rocketmq-clients](https://github.com/apache/rocketmq-clients/blob/master/rust/README.md)
