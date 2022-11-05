# Pre-setup
- Rust environment for substrate [see here](https://docs.substrate.io/install/macos/)

# setup for build
## build kamos-node
1. build rust package
~~~
cd substrate-kamos-node
cargo build --release
~~~
2. start first node()
~~~
./target/release/node-template \
  --base-path /tmp/node01 \
  --chain ./customSpecRaw.json \
  --port 30333 \
  --ws-port 9945 \
  --rpc-port 9933 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --validator \
  --rpc-methods Unsafe \
  --name KAMOS01 \
  --password-interactive
~~~

## build kamos-ui
~~~
~~~