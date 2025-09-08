# Mini-Redis (Redis Clone)

A simplified Redis-like in-memory key–value store built from scratch for learning and showcasing systems programming concepts.
This project demonstrates how to implement a database server with networking, concurrency, persistence, and custom data structures.

---

## Features (Planned & Implemented)

### Minimal Key–Value Store
- [x] In-memory storage with `HashMap<String, String>`
- [x] Basic commands:
  - `SET key value`
  - `GET key`
  - `DEL key`
- [x] Simple text-based command parsing

### Networking
- [x] TCP server socket
- [x] Accept multiple client connections
- [x] Handle concurrent clients (multi-threading / async I/O)
- [x] Command/response over custom protocol

### Persistence
- [ ] Snapshotting (periodically save DB to file)
- [x] Append-only log (AOF) for crash recovery

### Data Structures
- [ ] Lists (`LPUSH`, `RPOP`)
- [ ] Sets (`SADD`, `SMEMBERS`)
- [ ] Pub/Sub (`PUBLISH`, `SUBSCRIBE`)

### Extras
- [ ] Key expiry (`SETEX key seconds value`)
- [ ] Transactions (`MULTI`, `EXEC`)
- [ ] Simple clustering / sharding

---

## Roadmap

1. **Core storage** — Implement in-memory key–value operations with a parser for `SET`, `GET`, and `DEL`.
2. **Networking** — Add TCP server, handle multiple clients, and support a simple request/response protocol.
3. **Persistence** — Save snapshots to disk and/or implement append-only logs.
4. **Data structures** — Extend functionality with lists, sets, and pub/sub features.
5. **Extras** — Implement expiry, transactions, and explore distributed features.

---

## Tech Stack

- **Language**: Rust
- **Concurrency**: Multi-threading or async event loop
- **Networking**: TCP sockets
- **Persistence**: File I/O (snapshot & append-only log)
- **Testing**: Unit tests for commands and networking

---

## Learning Goals

By building this project, I aim to strengthen skills in:

- Systems programming (memory management, file I/O, networking)
- Socket programming (TCP client–server communication)
- Concurrency (threads, async I/O, locks/channels)
- Data structure implementation (hash maps, lists, sets)
- Database design concepts (persistence, transactions, pub/sub)
- Writing clean, modular, and testable code
- Performance optimisation & benchmarking

---

## Running the Project

```bash
# Clone the repository
git clone https://github.com/your-username/mini-redis.git
cd mini-redis

# Build the project
cargo build
```

### Start the server

```bash
cargo run
```

### Connect with a client (e.g. `telnet` or `nc`)

```bash
telnet localhost 6379
SET mykey hello
GET mykey
DEL mykey
```

---

## Example Usage

```bash
> SET name "Alice"
OK
> GET name
"Alice"
> DEL name
(integer) 1
```
