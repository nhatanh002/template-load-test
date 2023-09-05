# [load-test](https://github.com/fbielejec/ws-load-test)

This is a template for load testing repo for backend project. Currently supporting:
- websocket client
- http client based on goose
- gRPC client (untested)

### build and run

```bash
cargo run --bin ws-load-test -- --url ws://146.190.82.101:8001/ws/ping -v info -c 10000
cargo run --bin http-load-test -- -H http://146.190.82.101:8001 --test-plan "10000,1s;0,0s"
```
