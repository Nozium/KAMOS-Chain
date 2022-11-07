## presetup  for osx(intel) user 
- install rust lib from rustup commnad
~~~
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
~~~
- install yarn (brew was required)
~~~
brew install yarn
~~~

## clone and build
1. clone repo
~~~
git clone https://github.com/Nozium/KAMOS-Chain
cd KAMOS-Chain
~~~
2. build chain 
~~~
cd ./substrate-package-1.0/substrate-kamos
./scripts/init.sh
./scripts/build.sh
cargo build --release
~~~

3. start kamosuser chain(in new terminal)
~~~
cd KAMOS-Chain/substrate-package-1.0/substrate-kamos
./target/release/substrate-kamos --chain kamosuser --base-path /tmp/kamosuser --alice --validator
~~~

4. start kamosagent chain(in new terminal)
~~~
cd KAMOS-Chain/substrate-package-1.0/substrate-kamos
./target/release/substrate-kamos --chain kamosagent --base-path /tmp/kamosagent --alice --validator
~~~
※agent wallet will be extend with kamosuser

3. install front ui 
~~~
cd ./substrate-package-1.0/substrate-kamos-ui
yarn install 
~~~
※ front-ui is now on progress