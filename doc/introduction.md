# Jormungandr User Guide

Welcome to the Jormungandr User Guide.

``Jormungandr 사용 설명서에 오신 것을 환영합니다.``

Jormungandr is a node implementation, written in rust, with the
initial aim to support the Ouroboros type of consensus protocol.

Jormungandr는 Ouroboros 유형의 합의 프로토콜을 지원하기위한 초기 목표를 가지고 Rust로 작성된 노드 구현입니다.


A node is a participant of a blockchain network, continuously making,
sending, receiving, and validating blocks. Each node is responsible
to make sure that all the rules of the protocol are followed.

## Mythology

Jörmungandr refers to the _Midgard Serpent_ in Norse mythology. It is a hint to
_Ouroboros_, the Ancient Egyptian serpent, who eat its own tail, as well as the
[IOHK paper](https://eprint.iacr.org/2016/889.pdf) on proof of stake.



# General Concepts

This chapter covers the general concepts of the blockchain, and their application
in the node, and is followed by the node organisation and the user interaction with it.



# Blockchain concepts

## Time

Slots represent the basic unit of time in the blockchain, and at each slot
a block could be present.

Consecutive slots are grouped into epochs, which have updatable size defined
by the protocol.

## Fragments

Fragments are part of the blockchain data that represent all the possible
events related to the blockchain health (e.g. update to the protocol), but
also and mainly the general recording of information like transactions and
certificates.

## Blocks

Blocks represent the spine of the blockchain, safely and securely linking
blocks in a chain, whilst grouping valid fragments together.

Blocks are composed of 2 parts:

* The header
* The content

The header link the content with the blocks securely together, while the
content is effectively a sequence of fragments.

## Blockchain

The blockchain is the general set of rules and the blocks that are periodically created.
Some of the rules and settings, can be changed dynamically in the system by updates,
while some other are hardcoded in the genesis block (first block of the blockchain).

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

* Ouroboros BFT (OBFT)
* Ouroboros Genesis-Praos

Ouroboros BFT is a simple Byzantine Fault Tolerant (BFT) protocol where the
block makers is a known list of leaders that successively create a block and
broadcast it on the network.

Ouroboros Genesis Praos is a proof of stake (PoS) protocol where the block
maker is made of a lottery where each stake pool has a chance proportional to
their stake to be elected to create a block. Each lottery draw is private to
each stake pool, so that the overall network doesn't know in advance who can
or cannot create blocks.

In Genesis-Praos slot time duration is constant, however the frequency of 
creating blocks is not stable, since the creation of blocks is a probability 
that is linked to the stake and consensus_genesis_praos_active_slot_coeff.

**Note**: In Genesis-Praos, if there is no stake in the system, no blocks will be 
created anymore starting with the next epoch.

## Leadership

The leadership represent in abstract term, who are the overall leaders of the
system and allow each individual node to check that specific blocks are
lawfully created in the system.

The leadership is re-evaluated at each new epoch and is constant for the
duration of an epoch.

## Leader

Leader are an abstraction related to the specific actor that have the ability
to create block; In OBFT mode, the leader just the owner of a cryptographic
key, whereas in Genesis-Praos mode, the leader is a stake pool.

## Transaction

Transaction forms the cornerstone of the blockchain, and is one type of fragment
and also the most frequent one.

Transaction is composed of inputs and outputs; On one side, the inputs represent
coins being spent, and on the other side the outputs represent coins being received.

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

\\[ \sum Inputs = \sum Outputs + fees \\]

Transaction need to be authorized by each of the inputs in the transaction by their respective witness.
In the most basic case, a witness is a cryptographic signature, but depending on the type of input can
the type of witness vary.

## Accounting

The blockchain has two methods of accounting which are interoperable:

* Unspent Transaction Output (UTXO)
* Accounts

UTXO behaves like cash/notes, and work like fixed denomination ticket that are
cumulated. This is the accounting model found in Bitcoin. A UTXO is uniquely
reference by its transaction ID and its index.

Accounts behaves like a bank account, and are simpler to use since exact amount
can be used. This is the accounting model found in Ethereum. An account is
uniquely identified by its public key.

Each inputs could refer arbitrarily to an account or a UTXO, and similarly
each outputs could refer to an account or represent a new UTXO.


# Stake

In a proof of stake, participants are issued a stake equivalent to the amount
of coins they own. The stake is then used to allow participation in the protocol,
simply explained as:

> The more stake one has, the more likely one will participate in the good health of the network.

When using the BFT consensus, the stake doesn't influence how the system
runs, but stake can still be manipulated for a later transition of the chain
to another consensus mode.

## Stake in the Account Model

Account are represented by 1 type of address and are just composed of a public key.
The account accumulate moneys and its stake power is directly represented by the amount it contains

For example:

```

    A - Account with 30$ => Account A has stake of 30
    B - Account with 0$ => Account B has no stake

```

The account might have a bigger stake than what it actually contains, since it could
also have associated UTXOs, and this case is covered in the next section.

## Stake in the UTXO Model

UTXO are represented by two kind of addresses:

* single address: those type of address have no stake associated
* group address: those types of address have an account associated which receive the stake power of the UTXOs value

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

Stake pool has no stake power on their own, but participants in the network
delegate their stake to a pool for running the operation.

## Stake Delegation

Stake can and need to be delegated to stake pool in the system. They can
change over time with a publication of a new delegation certificate.

Delegation certificate are a simple declaration statement in the form of:

```
    Account 'A' delegate to Stake Pool 'Z'
```

Effectively it assign the stake in the account and its associated UTXO stake
to the pool it delegates to until another delegation certificate is made.



# Node organisation


## Secure Enclave

The secure enclave is the component containing the secret cryptographic
material, and offering safe and secret high level interfaces to the rest of
the node.

## Network

The node's network is 3 components:

* Intercommunication API (GRPC)
* Public client API (REST)
* Control client API (REST)

More detailed information [here](./network.md)

### Intercommunication API (GRPC)

This interface is a binary, efficient interface using the protobuf format and
GRPC standard. The protobuf files of types and interfaces are available in
the source code.

The interface is responsible to communicate with other node in the network:

* block sending and receiving
* fragments (transaction, certificates) broadcast
* peer2peer gossip

### Public API REST

This interface is for simple queries for clients like:

* Wallet Client & Middleware
* Analytics & Debugging tools
* Explorer

it's recommended for this interface to not be opened to the public.

TODO: Add a high level overview of what it does

### Control API REST

This interface is not finished, but is a restricted interface with ACL,
to be able to do maintenance tasks on the process:

* Shutdown
* Load/Retire cryptographic material

TODO: Detail the ACL/Security measure



Jörmungandr network capabilities are split into:

1. the REST API, used for informational queries or control of the node;
2. the gRPC API for blockchain protocol exchange and participation;

Here we will only talk of the later, the REST API is described in another
chapter already: [go to REST documentation](../quickstart/03_rest_api.md)

# The protocol

The protocol is based on commonly used in the industry tools: HTTP/2 and RPC.
More precisely, Jörmungandr utilises [`gRPC`](https://www.grpc.io).

This choice has been made for it is already widely supported across the world,
it is utilising HTTP/2 which makes it easier for Proxy and Firewall to recognise
the protocol and allow it.

## Type of queries

The protocol allows to send multiple types of messages between nodes:

* sync block to remote peer's _Last Block_ (`tip`).
* propose new fragments (new transactions, certificates, ...):
  this is for the fragment propagation.
* propose new blocks: for the block propagation.

There are other commands to optimise the communication and synchronisation
between nodes.

Another type of messages is the `Gossip` message. It allows Nodes to exchange
information (gossips) about other nodes on the network, allowing the peer
discovery.

## Peer discovery

Peer discovery is done via [`Poldercast`](https://hal.inria.fr/hal-01555561/document)'s Peer to Peer (P2P) topology
construction. The idea is to allow the node to participate actively into
building the decentralized topology of the p2p network.

This is done through gossiping. This is the process of sharing with others
topology information: who is on the network, how to reach them and what are
they interested about.

In the poldercast paper there are 3 different modules implementing 3 different
strategies to select nodes to gossip to and to select the gossiping data:

1. Cyclon: this module is responsible to add a bit of randomness in the gossiping
   strategy. It also prevent nodes to be left behind, favouring contacting Nodes
   we have the least used;
2. Vicinity: this module helps with building an interest-induced links between
   the nodes of the topology. Making sure that nodes that have common interests
   are often in touch.
3. Rings: this module create an oriented list of nodes. It is an arbitrary way to
   link the nodes in the network. For each topics, the node will select a set of
   close nodes.

