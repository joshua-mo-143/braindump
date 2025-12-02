## braindump

This library primarily aims to provide a reliable, competent memory engine for AI agents in Rust.

## What is memory?
Memory can be defined as the ability of an AI agent to store, recall and use information from past interactions to personalise interactions with users, adapt its behaviour over time and increase its accuracy over time by reviewing conversations, summarizing them and storing facts about them.

In programming terms: this means creating an abstraction that can do the following:
- Inject context into system/user prompts based on previous interactions with a user
- Summarize past and current conversations, and then store them somewhere
- Manage aforementioned memories, carrying out operations like memory compaction, consolidation, and more

The idea is primarily backed by context engineering (the practice of engineering the agent's environment rather than the prompt) becoming a very crucial aspect of agent development currently.

## Features
- Comes with an in-mem impl for 100% in-process memory storage
- Generic interfaces for vector stores, embedding and memory generation
- Core lib is WASM compatible (see `wasm` section)

## WASM/WebAssembly compatibility
To enable WASM, you need to enable the `wasm` feature then compile to any kind of `wasm32` target.

WASM is incompatible with the `fastembed` feature due to it using some not-WASM friendly components.

## TODO before release
- [x] Check all `FIXME` and `TODO`, as well as `unimplemented`
- [x] Check that everything compiles for all intended targets

## Roadmap
- [x] In-memory vector store implementation for testing as well as quickly spinning up a hot cache
- [x] Generic embedding interface for inserting memories
- [x] Integrate with at least one embedding provider to avoid manual re-wiring
- [x] Generic memory generation interface
- [x] WASM compatibility
- [ ] Rig integration
