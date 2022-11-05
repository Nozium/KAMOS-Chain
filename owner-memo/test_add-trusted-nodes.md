## simulate network
alice : 12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
bob : 12D3KooWFDTVD19VEzFuGjPFT63jvzmHKNCwtSKWuHQd7DHy1zm7

## add node
https://docs.substrate.io/tutorials/get-started/add-trusted-nodes/
### Sr25519 aura
~~~
./target/release/node-template key generate --scheme Sr25519 --password-interactive
~~~

user1
~~~
Key password: kamo
Secret phrase:       section cram castle outer conduct first kite response female smart leader stage
Network ID:        substrate
Secret seed:       0xb86c73bdb829fd1b729bd218f657da6d0371a5b71ef9a29a71946248f491e8bc
Public key (hex):  0xea5ee9bddfadbcb43023aba8eba39184b8311ecb34a2356f8efc71bebede8d59
Account ID:        0xea5ee9bddfadbcb43023aba8eba39184b8311ecb34a2356f8efc71bebede8d59
Public key (SS58): 5HN1Fsdiz8EmppUgEpchyDd2btcdYFzfEq13N8ceeNZRMNTv
SS58 Address:      5HN1Fsdiz8EmppUgEpchyDd2btcdYFzfEq13N8ceeNZRMNTv
~~~

user2
~~~
Key password: mos
Secret phrase:       quarter pact post rapid catalog push fancy cause bridge actor globe winter
Network ID:        substrate
Secret seed:       0x559bd326ee5076af52567f542f52f499a4577f96fa27ddfbf2949a1bee7cc9d8
Public key (hex):  0xf278405064945bc5de8eddcf1bc6ccb8949322b988dd7848a501e7c872970661
Account ID:        0xf278405064945bc5de8eddcf1bc6ccb8949322b988dd7848a501e7c872970661
Public key (SS58): 5HYdAfAgJiYobyfZgPH2vZYCQFQugGukBCDyb72RAw51khc2
SS58 Address:      5HYdAfAgJiYobyfZgPH2vZYCQFQugGukBCDyb72RAw51khc2
~~~

### Ed2519 スキームでのファイナライズキー
user1 / generate
~~~
./target/release/node-template key inspect --password-interactive --scheme Ed25519 "section cram castle outer conduct first kite response female smart leader stage"
~~~
user1 / return 
~~~
Secret phrase:       section cram castle outer conduct first kite response female smart leader stage
Network ID:        substrate
Secret seed:       0xb86c73bdb829fd1b729bd218f657da6d0371a5b71ef9a29a71946248f491e8bc
Public key (hex):  0x70b610a26d357243055f5efc8e7ad75bbb23a1307dfa31c4d2c079b76c78d4a3
Account ID:        0x70b610a26d357243055f5efc8e7ad75bbb23a1307dfa31c4d2c079b76c78d4a3
Public key (SS58): 5EcVJdkBBV2mcVxQcLiJTpRFtDHXzyxjZxabhxCEtYt5tSve
SS58 Address:      5EcVJdkBBV2mcVxQcLiJTpRFtDHXzyxjZxabhxCEtYt5tSve
~~~

user2 / generate
~~~
./target/release/node-template key inspect --password-interactive --scheme Ed25519 "quarter pact post rapid catalog push fancy cause bridge actor globe winter"
~~~
user2 / return 
~~~
Secret phrase:       quarter pact post rapid catalog push fancy cause bridge actor globe winter
Network ID:        substrate
Secret seed:       0x559bd326ee5076af52567f542f52f499a4577f96fa27ddfbf2949a1bee7cc9d8
Public key (hex):  0x306f2f71dcf658bd6967394dbf8427c7cf860e4a582cc909f50e92b3319e0be3
Account ID:        0x306f2f71dcf658bd6967394dbf8427c7cf860e4a582cc909f50e92b3319e0be3
Public key (SS58): 5DADBf9CYit3nfhinsgAuW1q9AvAzMHCQPGQFbVoxyvojMDz
SS58 Address:      5DADBf9CYit3nfhinsgAuW1q9AvAzMHCQPGQFbVoxyvojMDz
~~~

### customSpec.json 
上記の二件をaura / granpaに入れて
~~~
./target/release/node-template build-spec --chain=customSpec.json --raw --disable-default-bootnode > customSpecRaw.json
~~~

## 起動
### KAMOS01
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
> kamos

~~~
./target/release/node-template key insert --base-path /tmp/node01 \
  --chain customSpecRaw.json \
  --scheme Sr25519 \
  --suri "section cram castle outer conduct first kite response female smart leader stage" \
  --password-interactive \
  --key-type aura
~~~

~~~
./target/release/node-template key insert \
  --base-path /tmp/node01 \
  --chain customSpecRaw.json \
  --scheme Ed25519 \
  --suri "section cram castle outer conduct first kite response female smart leader stage" \
  --password-interactive \
  --key-type gran
~~~

### KAMOS02
~~~
./target/release/node-template \
  --base-path /tmp/node02 \
  --chain ./customSpecRaw.json \
  --port 30334 \
  --ws-port 9946 \
  --rpc-port 9934 \
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
  --validator \
  --rpc-methods Unsafe \
  --name KAMOS02 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWCiUpe3jvuvotMirNPFfapHJ84x6YRX3e9pH4rfFYmaSD \
  --password-interactive 
~~~
> kamos

~~~
./target/release/node-template key insert --base-path /tmp/node02 \
  --chain customSpecRaw.json \
  --scheme Sr25519 \
  --suri "quarter pact post rapid catalog push fancy cause bridge actor globe winter" \
  --password-interactive \
  --key-type aura
~~~

~~~
./target/release/node-template key insert \
  --base-path /tmp/node02 \
  --chain customSpecRaw.json \
  --scheme Ed25519 \
  --suri "quarter pact post rapid catalog push fancy cause bridge actor globe winter" \
  --password-interactive \
  --key-type gran
~~~

## 成功? とりあえずnodeを二つ実行することはできた

## add acount from polkadot UI
alice-kamos-000001
~~~
bottom drive obey lake curtain smoke basket hold race lonely fit walk
~~~
stop node and account will be lost?
→ template ui には含まれていないがlocal ネットには含まれているようである
