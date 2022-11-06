# KAMOS-chain
## Overview
KAMOS-chainは、ユーザーのプライベートチェーン内で成長するアイディアAI : KAMOSを運用するための階層的な基盤チェーンである.

public(ETH-NFT) | kamosuser(substrain) -> kamosaget(substrain) の3階層で構成され.kamosagent内の取引は、全てAIによる取引のみで構成される.

kamosagentは、walletに含まれるkamosuserのアイディア(シードとして利用)を元に、類似のagentを探索し、交配することで新しいアイディアを成長させていく(新規walletの自動発行, initsupply あり).この時、類似agentの探索は、kamosuser間を跨いで実行される.(Private 自動DEX)

kamosagent層がAIのみでトランザクションが発生することから、トークンのトランスファーを可視化することによって、アイディアの構造と、生産された時系列、関係性を可視化していくことができる.つまり、kamosagentmの集合( KOKEDAMA )が、アイディアの構造を表すものとして複数人のkamosuserの寄与度を示すマップとして取り扱える.

kamosuserは、アイディアの構造を表すKOKEDAMAをNFTとしてpublicな領域にMINTすることで、ETHの獲得を実行することができる.この時NFTにふぞくさせるスマートコントラクトによって、KOKEDAMAの構造に基づいて、kamosuser / kamosagentに報酬が分配される.(Rain)

これらの階層性ブロックチェーンとAIによる自動取引、AIトランザクションを元にしたNFTと還元スマートコントラクトによって、継続的にKAMOS-XXのチェーン内に存在するtoken量が増えていく形を構成する.
- [designdoc](./designdocs.md)

## Product Image
![kamos-chain]()

## Install and Build
- [setup](./owner-memo/setup.md)

## Desc for Hackathon
### Technicality
> 取り組んでいる問題の複雑さ、またはその解決へのアプローチとは？

### Originality
> 新しい問題や未解決の問題に取り組んでいるか、既存の問題に対してユニークで創造的な解決策を生み出しているか？

### Practicality
> プロジェクトの完成度や機能性はどうか？想定される利用者が使用できる状態になっているか？

### Usability (UI/UX/DX)
> プロジェクトは使いやすいか？ユーザーとの摩擦をなくすための努力がなされているか？

### WOW factor
> 従来のプロダクトにはもたらし得なかった新しい視点はあるか？


## Reference Project
- substrate project template was used for create custom build block chain
    -  https://github.com/substrate-developer-hub/substrate-node-template
    - https://github.com/substrate-developer-hub/substrate-module-template
