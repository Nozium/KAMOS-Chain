## Tutorialからの修正項目
~~~
[dependencies]
pallet-node-authorization = { default-features = false, version = "4.0.0-dev", git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.28" }
~~~
> **warn**
> polkadot-v0.9.31に修正が必要

2. Implement the Config trait 
挿入先のTalletが結構修正されている印象. そのまま修正してもruntimeチェックは通る


> charie node を接続情報を元に登録できているはずだが、peer nodeが増えてない.
> dave node : charie nodeが接続できないため
