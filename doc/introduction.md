# Jormungandr User Guide

Welcome to the Jormungandr User Guide.

``Jormungandr 사용자 가이드에 오신 것을 환영합니다.``

Jormungandr is a node implementation, written in rust, with the
initial aim to support the Ouroboros type of consensus protocol.

``Jormungandr는 Ouroboros 유형의 합의 프로토콜을 지원하기위한 초기 목표를 가진 'RUST'로 작성된 노드 구현입니다.``


A node is a participant of a blockchain network, continuously making,
sending, receiving, and validating blocks. Each node is responsible
to make sure that all the rules of the protocol are followed.

``노드는 블록체인 네트워크의 참여자로서 블록을 지속적으로 만들고, 보내고, 받고, 검증합니다. 각 노드는 프로토콜의 모든 규칙을 준수해야합니다.``

## Mythology

Jörmungandr refers to the _Midgard Serpent_ in Norse mythology. It is a hint to
_Ouroboros_, the Ancient Egyptian serpent, who eat its own tail, as well as the
[IOHK paper](https://eprint.iacr.org/2016/889.pdf) on proof of stake.

``요르문간드는 북유럽 신화에서 미드가르드 뱀을 가리 킵니다. 그것은 자신의 꼬리를 먹는 고대 이집트의 뱀인 Ouroboros 로 이어졌으며 지분 증명에 관한 IOHK 논문 입니다.``


# General Concepts

This chapter covers the general concepts of the blockchain, and their application
in the node, and is followed by the node organisation and the user interaction with it.

``이 장에서는 블록체인의 일반적인 개념, 노드에서의 적용, 노드 구성, 사용자와의 상호 작용을 설명합니다.``

# Blockchain concepts

## Time

Slots represent the basic unit of time in the blockchain, and at each slot
a block could be present.

``슬롯은 블록체인의 기본 시간 단위를 나타내며 각 슬롯에는 블록이 존재할 수 있습니다.``

Consecutive slots are grouped into epochs, which have updatable size defined
by the protocol.

``연속 슬롯은 업데이트 가능한 프로토콜에 의해 정의된 에포크 그룹 입니다.``


## Fragments

Fragments are part of the blockchain data that represent all the possible
events related to the blockchain health (e.g. update to the protocol), but
also and mainly the general recording of information like transactions and
certificates.

``프래그먼트는 블록체인 상태 (예 : 프로토콜 업데이트)와 관련된 모든 가능한 이벤트 뿐만 아니라 주로 거래 및 인증서와 같은 정보의 일반적인 기록을 나타내는 블록체인 데이터의 일부입니다.``

## Blocks

Blocks represent the spine of the blockchain, safely and securely linking
blocks in a chain, whilst grouping valid fragments together.

``블록은 블록체인의 척추 입니다. 유효한 프레이그먼트를 그룹화합니다. 또한 블록체인의 보안을 준수하고 안전하게 연결합니다.``

Blocks are composed of 2 parts:

``블록은 두 부분으로 구성됩니다.``

* The header
* The content

The header link the content with the blocks securely together, while the
content is effectively a sequence of fragments.

``헤더는 컨텐츠를 블록과 안전하게 연결하는 반면 컨텐츠는 사실상 일련의 프래그먼트입니다.``

## Blockchain

The blockchain is the general set of rules and the blocks that are periodically created.
Some of the rules and settings, can be changed dynamically in the system by updates,
while some other are hardcoded in the genesis block (first block of the blockchain).

``블록체인은 주기적으로 생성되는 규칙과 블록의 집합입니다. 일부 규칙 및 설정은 업데이트를 통해 시스템에서 동적으로 변경 될 수 있습니다. 또 다른 규칙 및 설정은 생성 제네시스 블록에 하드 코딩됩니다.``

```
    +-------+      +-------+
    |Genesis+<-----+Block 1+<--- ....
    |Header |      |Header |
    +---+---+      +---+---+
        |              |
    +---v---+      +---v---+
    |Genesis|      |Block 1|
    |Content|      |Content|
    +-------+      +-------+
```

## Consensus

The node currently support the following consensus protocol:

``노드는 현재 다음 합의 프로토콜을 지원합니다.``

* Ouroboros BFT (OBFT)
* Ouroboros Genesis-Praos

Ouroboros BFT is a simple Byzantine Fault Tolerant (BFT) protocol where the
block makers is a known list of leaders that successively create a block and
broadcast it on the network.

``Ouroboros BFT는 간단한 비잔틴 결함 허용(BFT) 프로토콜입니다. BFT는 블록을 만들고 네트워크에서 브로드캐스트 할 수 있는 leaders 목록입니다. ``

Ouroboros Genesis Praos is a proof of stake (PoS) protocol where the block
maker is made of a lottery where each stake pool has a chance proportional to
their stake to be elected to create a block. Each lottery draw is private to
each stake pool, so that the overall network doesn't know in advance who can
or cannot create blocks.

``Ouroboros Genesis Praos는 블록 제작자가 복권(각 스테이크 풀이 자신의 스테이크에 비례하여 블록을 만들 수 있는)으로 만든 PoS 프로토콜입니다. 각 추첨은 각 스테이크 풀에 대해 비공개이므로 전체 네트워크가 사전에 누가 블록을 생성하는지 또는 생성못하는지 여부를 미리 알 수 없습니다.``

In Genesis-Praos slot time duration is constant, however the frequency of 
creating blocks is not stable, since the creation of blocks is a probability 
that is linked to the stake and consensus_genesis_praos_active_slot_coeff.

``Genesis-Praos에서 슬롯 시간 지속 시간은 일정하지만 블록 생성 빈도는 안정적이지 않습니다. 블록 생성은 스테이크 및 consensus_genesis_praos_active_slot_coeff 와 연결된 확률이기 때문입니다.``

**Note**: In Genesis-Praos, if there is no stake in the system, no blocks will be 
created anymore starting with the next epoch.

``참고 : Genesis-Praos에서 시스템에 스테이크가 없으면 다음 에포크부터 더 이상 블록이 생성되지 않습니다.``

## Leadership

The leadership represent in abstract term, who are the overall leaders of the
system and allow each individual node to check that specific blocks are
lawfully created in the system.

``리더십은 시스템 전체의 리더인 추상 용어로 표현되며 각 개별 노드가 시스템에서 특정 블록이 합법적으로 생성되었는지 확인할 수 있도록합니다.``

The leadership is re-evaluated at each new epoch and is constant for the
duration of an epoch.

``리더십은 각각의 새로운 에포크에서 재평가되며, 에포크 동안 지속됩니다.``

## Leader

Leader are an abstraction related to the specific actor that have the ability
to create block; In OBFT mode, the leader just the owner of a cryptographic
key, whereas in Genesis-Praos mode, the leader is a stake pool.

``리더는 블록을 생성 할 수 있는 어떤 생성자와 관련된 추상화입니다. OBFT 모드에서는 리더가 암호 키의 소유자 일뿐 아니라 Genesis-Praos 모드에서는 리더가 스테이크 풀입니다.``

## Transaction

Transaction forms the cornerstone of the blockchain, and is one type of fragment
and also the most frequent one.

``거래는 블록 체인의 초석을 이루며, 한 가지 유형의 조각이자 가장 빈번한 것입니다.``

Transaction is composed of inputs and outputs; On one side, the inputs represent
coins being spent, and on the other side the outputs represent coins being received.

``트랜잭션은 입력과 출력으로 구성됩니다. 한쪽은 입력 된 동전을 나타내고 다른 쪽은 수신되는 동전을 나타냅니다.``


```
    Inputs         Alice (80$)        Bob (20$)
                        \             /
                         \           /
                          -----------
                                100$
                             --------- 
                            /         \
    Outputs            Charlie (50$)  Dan (50$)
```

Transaction have fees that are defined by the blockchain settings and the following invariant hold:

``거래에는 블록 체인 설정과 다음과 같은 고정 보류로 정의되는 수수료가 있습니다.``

\\[ \sum Inputs = \sum Outputs + fees \\]

Transaction need to be authorized by each of the inputs in the transaction by their respective witness.
In the most basic case, a witness is a cryptographic signature, but depending on the type of input can
the type of witness vary.

``거래는 각 증인에 의해 거래마다 입력 승인이 필요합니다. 가장 기본적인 경우 감시는 암호화 서명이지만 입력 유형에 따라 감시 유형이 다를 수 있습니다.``

## Accounting

The blockchain has two methods of accounting which are interoperable:

``블록체인에는 상호 운용 가능한 두 가지 회계 방법이 있습니다.``

* Unspent Transaction Output (UTXO)
* Accounts

UTXO behaves like cash/notes, and work like fixed denomination ticket that are
cumulated. This is the accounting model found in Bitcoin. A UTXO is uniquely
reference by its transaction ID and its index.

``UTXO는 현금/노트처럼 행동하며 누적 된 고정 명칭 티켓처럼 작동합니다. 이것은 비트코인에서 발견 된 회계 모델입니다. UTXO는 트랜잭션 ID와 색인으로 고유하게 참조됩니다.``

Accounts behaves like a bank account, and are simpler to use since exact amount
can be used. This is the accounting model found in Ethereum. An account is
uniquely identified by its public key.

``계정은 은행 계좌처럼 작동하며 정확한 금액을 사용할 수 있으므로 사용하기가 더 쉽습니다. 이것은 이더리움에서 발견 된 회계 모델입니다. 계정은 공개 키로 고유하게 식별됩니다.``

Each inputs could refer arbitrarily to an account or a UTXO, and similarly
each outputs could refer to an account or represent a new UTXO.

``각 입력은 임의로 계정 또는 UTXO를 참조 할 수 있습니다. 마찬가지로 각 출력은 계정을 참조하거나 새로운 UTXO를 나타낼 수 있습니다.``


# Stake

In a proof of stake, participants are issued a stake equivalent to the amount
of coins they own. The stake is then used to allow participation in the protocol,
simply explained as:

``스테이크 증거로 참가자는 자신이 소유 한 코인의 양에 해당하는 스테이크를 발행합니다. 그런 다음 스테이크는 프로토콜에 참여하는 데 사용되며 다음과 같이 간단히 설명됩니다.``

> The more stake one has, the more likely one will participate in the good health of the network.

> 스테이크가 많을수록 네트워크의 건강 상태에 참여할 가능성이 높습니다.

When using the BFT consensus, the stake doesn't influence how the system
runs, but stake can still be manipulated for a later transition of the chain
to another consensus mode.

``BFT 합의를 사용할 때 스테이크는 시스템 운영 방식에 영향을 미치지 않지만 체인을 나중에 다른 합의 모드로 전환하기 위해 스테이크를 조작 할 수 있습니다.``

## Stake in the Account Model

Account are represented by 1 type of address and are just composed of a public key.
The account accumulate moneys and its stake power is directly represented by the amount it contains

``계정은 1가지 유형의 주소로 표시되며 공개 키로 만 구성됩니다. 계정은 돈을 축적하고 계정 지분은 해당 금액으로 직접 표시됩니다.``

For example:

```

    A - Account with 30$ => Account A has stake of 30
    B - Account with 0$ => Account B has no stake

```

The account might have a bigger stake than what it actually contains, since it could
also have associated UTXOs, and this case is covered in the next section.

``계정에 UTXO가 연결될 수 있으므로 계정에 실제로 포함 된 것보다 더 큰 지분이있을 수 있으며 이 사례는 다음 섹션에서 다룹니다.``


## Stake in the UTXO Model

UTXO are represented by two kind of addresses:

``UTXO 는 두 가지 종류의 주소로 표시됩니다.``

* single address: those type of address have no stake associated
* group address: those types of address have an account associated which receive the stake power of the UTXOs value

* single address: 이러한 유형의 주소에는 스테이크가 연결되어 있지 않습니다
* group address: 이러한 유형의 주소에는 UTXO 가치의 지분을 받는 계정이 연결되어 있습니다.

For example with the following utxos:

```
    UTXO1 60$ (single address) => has stake of 0

    UTXO2 50$ (group address A) \
                                 ->- A - Account with 10$ => Account A has stake of 100
    UTXO3 40$ (group address A) /

    UTXO4 20$ (group address B) -->- B - Account with 5$ => Account B has stake of 25
```

## Stake pool

Stake pool are the trusted block creators in the genesis-praos system. A pool
is declared on the network explicitely by its owners and contains, metadata
and cryptographic material.

``스테이크 풀은 genesis-praos 시스템에서 신뢰할 수 있는 블록 제작자입니다. 풀은 그것의 소유자에 의해 명시 적으로 네트워크에 선언하고, 메타 데이터와 암호화 된 자료를 포함합니다.``

Stake pool has no stake power on their own, but participants in the network
delegate their stake to a pool for running the operation.

``스테이크 풀은 자체적으로 스테이크 권한이 없지만 네트워크의 참가자는 운영을 위해 풀에 스테이크를 위임합니다.``


## Stake Delegation

Stake can and need to be delegated to stake pool in the system. They can
change over time with a publication of a new delegation certificate.

``스테이크는 스테이크 풀에 위임 될 수 있습니다. 새 위임 인증서를 게시하면 시간이 지남에 따라 변경 될 수 있습니다.``

Delegation certificate are a simple declaration statement in the form of:

``위임 인증서는 다음과 같은 형식의 간단한 선언문입니다.``

```
    Account 'A' delegate to Stake Pool 'Z'
```

Effectively it assign the stake in the account and its associated UTXO stake
to the pool it delegates to until another delegation certificate is made.

``효과적으로 다른 위임 인증서가 작성 될 때까지 계정의 지분 및 관련 UTXO 지분을 위임하는 풀에 할당합니다.``


# Node organisation


## Secure Enclave

The secure enclave is the component containing the secret cryptographic
material, and offering safe and secret high level interfaces to the rest of
the node.

`` secure encloave 는 비밀 암호화 자료를 포함하고 나머지 노드에 안전하고 비밀이 높은 수준의 인터페이스를 제공하는 구성 요소입니다.``

## Network

The node's network is 3 components:

``노드의 네트워크 3 가지 구성 요소입니다.``

* Intercommunication API (GRPC)
* Public client API (REST)
* Control client API (REST)

More detailed information [here](./network.md)

### Intercommunication API (GRPC)

This interface is a binary, efficient interface using the protobuf format and
GRPC standard. The protobuf files of types and interfaces are available in
the source code.

``이 인터페이스는 protobuf 형식과 GRPC 표준을 사용하는 이진 효율적인 인터페이스입니다. 유형 및 인터페이스의 프로토 타입 파일은 소스 코드에서 사용 가능합니다.``

The interface is responsible to communicate with other node in the network:

``인터페이스는 네트워크의 다른 노드와 통신해야합니다.``

* block sending and receiving
* fragments (transaction, certificates) broadcast
* peer2peer gossip

### Public API REST

This interface is for simple queries for clients like:

``이 인터페이스는 다음과 같은 클라이언트에 대한 간단한 쿼리를 위한 것입니다.``

* Wallet Client & Middleware
* Analytics & Debugging tools
* Explorer

it's recommended for this interface to not be opened to the public.

``이 인터페이스를 공개하지 않는 것이 좋습니다.``

TODO: Add a high level overview of what it does

``TODO : 기능에 대한 개괄적인 개요 추가``

### Control API REST

This interface is not finished, but is a restricted interface with ACL,
to be able to do maintenance tasks on the process:

``이 인터페이스는 완료되지 않았지만 프로세스에서 유지 보수 작업을 수행 할 수 있도록 ACL이 있는 제한된 인터페이스입니다.``

* Shutdown
* Load/Retire cryptographic material

TODO: Detail the ACL/Security measure

``TODO : ACL/보안조치 세부 사항``

Jörmungandr network capabilities are split into:

``Jörmungandr 네트워크 기능은 다음과 같이 나뉩니다.``

1. the REST API, used for informational queries or control of the node;
2. the gRPC API for blockchain protocol exchange and participation;

``1. 정보 조회 또는 노드 제어에 사용되는 REST API;``
``2. 블록 체인 프로토콜 교환 및 참여를위한 gRPC API;``

Here we will only talk of the later, the REST API is described in another
chapter already: [go to REST documentation](../quickstart/03_rest_api.md)

``여기서는 나중에 이야기 할 것입니다. REST API는 이미 다른 장에서 설명 합니다. REST 문서로 이동``


# The protocol

The protocol is based on commonly used in the industry tools: HTTP/2 and RPC.
More precisely, Jörmungandr utilises [`gRPC`](https://www.grpc.io).

``이 프로토콜은 산업 도구에서 일반적으로 사용되는 HTTP/2 및 RPC를 기반으로합니다. 좀 더 정확하게 말씀드리면 Jörmungandr는 gRPC를 활용 합니다.``

This choice has been made for it is already widely supported across the world,
it is utilising HTTP/2 which makes it easier for Proxy and Firewall to recognise
the protocol and allow it.

``이 선택은 이미 전 세계에서 널리 지원되고 있기 때문에 프록시와 방화벽이 프로토콜을 인식하고 허용하는 것을 보다 쉽게하는 HTTP/2를 사용하고 있습니다.``

## Type of queries

The protocol allows to send multiple types of messages between nodes:

``이 프로토콜을 사용하면 노드간에 여러 유형의 메시지를 보낼 수 있습니다.``

* sync block to remote peer's _Last Block_ (`tip`).
* propose new fragments (new transactions, certificates, ...):
  this is for the fragment propagation.
* propose new blocks: for the block propagation.

* 원격 피어의 마지막 블록 ( tip)에 블록을 동기화 합니다.
* 새로운 프래그먼트(새 트랜잭션, 인증서)를 제안합니다. 이것은 프래그먼트 전파를 위한 것입니다.
* 블록 전파를 위해 새로운 블록을 제안하십시오.

There are other commands to optimise the communication and synchronisation
between nodes.

``노드 간 통신 및 동기화를 최적화하는 다른 명령이 있습니다.``

Another type of messages is the `Gossip` message. It allows Nodes to exchange
information (gossips) about other nodes on the network, allowing the peer
discovery.

``또 다른 유형의 메시지는 Gossip메시지입니다. 이를 통해 노드는 네트워크의 다른 노드에 대한 정보(가십)를 교환하여 피어 발견을 허용합니다.``

## Peer discovery

Peer discovery is done via [`Poldercast`](https://hal.inria.fr/hal-01555561/document)'s Peer to Peer (P2P) topology
construction. The idea is to allow the node to participate actively into
building the decentralized topology of the p2p network.

``피어 검색은 Poldercast의 피어 투 피어 (P2P) 토폴로지 구성을 통해 수행됩니다 . 아이디어는 노드가 p2p 네트워크의 분산 토폴로지를 구축하는 데 적극적으로 참여할 수 있도록 하는 것입니다.``

This is done through gossiping. This is the process of sharing with others
topology information: who is on the network, how to reach them and what are
they interested about.

``이것은 가십을 통해 이루어집니다. 이것은 다른 토폴로지 정보를 공유하는 프로세스입니다. 네트워크에있는 사람, 연락하는 방법 및 관심있는 대상.``

In the poldercast paper there are 3 different modules implementing 3 different
strategies to select nodes to gossip to and to select the gossiping data:

`` poldercast 논문에는 가십 노드를 선택하고 gossiping 데이터를 선택하는 3 가지 전략을 구현하는 3 가지 모듈이 있습니다.``

1. Cyclon: this module is responsible to add a bit of randomness in the gossiping
   strategy. It also prevent nodes to be left behind, favouring contacting Nodes
   we have the least used;
2. Vicinity: this module helps with building an interest-induced links between
   the nodes of the topology. Making sure that nodes that have common interests
   are often in touch.
3. Rings: this module create an oriented list of nodes. It is an arbitrary way to
   link the nodes in the network. For each topics, the node will select a set of
   close nodes.


``1. Cyclon :이 모듈은 gossiping 전략에 약간의 무작위성을 추가합니다. 또한 노드가 남겨지는 것을 방지하여 가장 적게 사용 된 노드와의 접촉을 선호합니다.``

``2. Vicinity :이 모듈은 토폴로지 노드 사이에 관심 유도 링크를 작성하는 데 도움이됩니다. 공통의 관심사를 가진 노드가 종종 연락해야합니다.``

``3. Rings :이 모듈은 지향적 인 노드 목록을 만듭니다. 네트워크의 노드를 연결하는 임의의 방법입니다. 각 주제에 대해 노드는 닫기 노드 세트를 선택합니다.``
