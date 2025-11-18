# NebulaChain

A production‑minded, educational Layer‑1 blockchain prototype built entirely from scratch in Rust.

NebulaChain demonstrates real blockchain node internals: Proof‑of‑Work consensus, block validation, persistent storage, async peer‑to‑peer networking, RPC interfaces, and modular architecture. It is designed as a hybrid project—clean and professional enough for portfolios, yet simple enough for learning.

---

# 🚀 Features

* **Layer‑1 Blockchain** fully implemented in Rust
* **Proof‑of‑Work (SHA‑256)** hash‑based mining
* **Block & Header Model** with Merkle‑ready transaction body
* **Persistent Storage** using sled embedded database
* **Async P2P Gossip Protocol** (INV / GETDATA / BLOCK design in progress)
* **RPC API** using Warp HTTP server
* **Modular Architecture** with clean separation of components
* **Beginner‑friendly code**, heavy documentation, and clear flow

---

# 📁 Project Structure

```
NebulaChain/
├── src/
│   ├── main.rs              # CLI + Node bootstrap
│   ├── chain/
│   │   ├── mod.rs           # module exports
│   │   ├── block.rs         # Block & BlockHeader structs + hashing
│   │   └── chain.rs         # ChainState: validation, insertion, head mgmt
│   ├── storage/
│   │   └── sled_store.rs    # DB backend for blocks
│   ├── miner/
│   │   └── worker.rs        # Mining engine
│   ├── p2p/
│   │   └── gossip.rs        # Async TCP gossip protocol
│   └── rpc/
│       └── http.rs          # Warp-based RPC server
├── scripts/
│   ├── run-three-nodes.ps1  # Windows 3-node demo
│   └── run-three-nodes.sh   # Linux/macOS 3-node demo
├── docs/
│   ├── architecture.md
│   ├── consensus.md
│   ├── mining.md
│   └── p2p_design.md
└── Cargo.toml
```

---

# 🏛️ Architecture Overview

NebulaChain consists of **six major subsystems**, each isolated into modules.

## 1. Chain Module (`chain/`)

Responsible for the blockchain's core logic:

* Block structure (header + body)
* Hashing logic
* Validation placeholders (will evolve into full rules)
* ChainState holds:

  * current head
  * block metadata
  * DB interfaces
* Methods to insert, lookup, and validate blocks

**Why important:** This module becomes the heart of consensus.

---

## 2. Storage Layer (`storage/`)

Uses **sled**, an embedded, high‑performance, crash‑safe key/value database.

Stores:

* blocks keyed by their hash
* future metadata (heads, chain tips, indexes)

**Benefits:**

* No external DB required
* Fast, persistent, embedded storage

---

## 3. Mining Engine (`miner/`)

Implements:

* Header hashing
* Nonce attempts
* Difficulty checking
* Block creation loop

Currently uses simplified PoW, but built to expand into:

* multi-thread mining
* real difficulty targets
* timestamp rules

---

## 4. P2P Networking (`p2p/`)

Async TCP listener using Tokio runtime.

Will evolve into full gossip protocol:

* INV (block announcement)
* GETDATA (block request)
* BLOCK (block transmission)
* Peer discovery

**Goal:** Let nodes broadcast mined blocks and sync from each other.

---

## 5. RPC Server (`rpc/`)

Runs a Warp HTTP server offering:

* `/status`: node health
* `/block/<hash>`: retrieve blocks

Will expand into:

* mining control
* metrics
* chain inspection

---

## 6. Node Bootstrap (`main.rs`)

The entrypoint that:

* Parses CLI flags using `clap`
* Boots storage
* Starts P2P listener
* Starts RPC
* Starts miner
* Handles bootstrap peers

Supports flags:

```
--db <path>
--p2p <addr>
--rpc <addr>
--bootstrap <peer>
--difficulty <n>
```

---

# 🧱 Block Structure

NebulaChain uses a Bitcoin‑like `BlockHeader`:

```
BlockHeader {
    prev_hash: [u8; 32],
    merkle_root: [u8; 32],
    timestamp: u64,
    nonce: u64,
    difficulty: u32,
    height: u64,
}
```

**Block hash = SHA‑256(header)**

The block body is a list of raw transaction bytes:

```
Vec<Vec<u8>>
```

---

# 🔗 High-Level Architecture Diagram

```
          ┌──────────────────────────────────────────┐
          │                 Node CLI                 │
          └───────────────┬──────────────────────────┘
                          │
     ┌────────────────────┴──────────────────────┐
     │                Node Runner                │
     └───────┬────────────┬────────────┬────────┘
             │            │            │
    ┌────────▼───┐ ┌──────▼──────┐ ┌───▼────────┐
    │   Chain    │ │    Miner    │ │    P2P     │
    └─────┬──────┘ └──────┬──────┘ └────┬───────┘
          │               │             │
      ┌───▼────┐     ┌────▼───┐   ┌────▼────────┐
      │Storage │     │ Blocks │   │   Network    │
      └───┬────┘     └────────┘   └──────────────┘
          │
      ┌───▼──────────┐
      │     RPC      │
      └──────────────┘
```

---

# 🏁 Running a Single Node

```
cargo run -- --db ./data/node1 --p2p 127.0.0.1:9000 --rpc 127.0.0.1:18000
```

---

# 🧪 Running a 3‑Node Local Network

### Windows:

```
scripts/run-three-nodes.ps1
```

### Linux/Mac:

```
chmod +x scripts/run-three-nodes.sh
scripts/run-three-nodes.sh
```

Three windows/nodes will appear and begin mining + gossiping.

---

# 🧭 Roadmap

NebulaChain follows a structured, multi-phase roadmap designed to evolve from a minimal PoW prototype into a production-grade blockchain node. Each phase builds on the previous one.

---

## **Phase 1 — Foundation Layer (Completed / In Progress)**

### 🎯 Goal: Establish core blockchain primitives

* [x] Block & BlockHeader structures
* [x] SHA-256 hashing implementation
* [x] Basic Block → Hash mechanism
* [x] Modular project architecture
* [x] Persistent sled storage
* [x] Minimal ChainState (head tracking, block insertion)
* [x] RPC `/status` and `/block/<hash>` endpoints
* [x] Basic mining loop (single-thread PoW)
* [ ] Genesis block implementation

---

## **Phase 2 — Networking & P2P Gossip (In Progress)**

### 🎯 Goal: Allow nodes to talk, sync, and propagate blocks

* [ ] TCP peer listener (Tokio)
* [ ] Peer connection manager
* [ ] Gossip message types:

  * [ ] `INV` (announce new blocks)
  * [ ] `GETDATA` (request missing blocks)
  * [ ] `BLOCK` (send full block)
* [ ] Serialization via `bincode`
* [ ] Heartbeat + ping/pong messages
* [ ] Basic peer discovery (bootstrap peers)
* [ ] Simple sync: "request blocks until tip"

---

## **Phase 3 — Consensus Rules Expansion**

### 🎯 Goal: Add real PoW, validation rules, and fork choice

* [ ] Convert difficulty → target
* [ ] Full PoW target check (`hash < target`)
* [ ] Difficulty adjustment algorithm (Bitcoin-style)
* [ ] Timestamp rules & drift limits
* [ ] `prev_hash` chain continuity check
* [ ] Block height verification
* [ ] Fork-choice rule: *most cumulative work wins*
* [ ] Orphan block handling

---

## **Phase 4 — Transaction System**

### 🎯 Goal: Add transactions & Merkle proofs

* [ ] Transaction struct (v1: simple payments)
* [ ] Mempool for unconfirmed transactions
* [ ] Transaction gossip
* [ ] Merkle Tree implementation
* [ ] Merkle root calculation & verification
* [ ] Transaction validation rules
* [ ] Fee + reward model

---

## **Phase 5 — Wallets, Keys & Signing**

### 🎯 Goal: Add cryptographic identity & signing

* [ ] Ed25519 or Secp256k1 key generation
* [ ] Wallet CLI (generate address, view keys)
* [ ] Transaction signing
* [ ] Signature verification in block validation
* [ ] Address format + checksum

---

## **Phase 6 — Node Sync, Reorgs & Reliability**

### 🎯 Goal: Make the node production-stable

* [ ] Full chain sync from genesis
* [ ] Headers-first sync (performance)
* [ ] Reorg handling & rollback logic
* [ ] Peer scoring & banning
* [ ] Snapshot + pruning system
* [ ] Fork detection and resolution

---

## **Phase 7 — Developer Experience & Tooling**

### 🎯 Goal: Improve debugging, testing, and usability

* [ ] Full documentation (mdBook or Docusaurus)
* [ ] Architecture diagrams (PNG/SVG)
* [ ] Logging improvements with tracing spans
* [ ] Prometheus-compatible metrics
* [ ] Benchmarks: mining, hashing, networking
* [ ] Docker Compose multi-node setup
* [ ] CI/CD pipeline (GitHub Actions)

---

## **Phase 8 — Hardening & Advanced Research Features**

### 🎯 Goal: Move toward experimental production readiness

* [ ] Fuzz testing & security checks
* [ ] Custom networking protocol optimizations
* [ ] Mempool sorting (fees, priority)
* [ ] State machine isolation
* [ ] Optional WASM smart contract sandbox
* [ ] Optional signature aggregation

# 📜 License

MIT License (recommended for open‑source Rust projects).
