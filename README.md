# Aori Artemis 
Artemis is a framework for writing MEV bots in Rust. It's designed to be simple, modular, and fast.

At its core, Artemis is architected as an event processing pipeline. The library is made up of three main components:

    Collectors: Collectors take in external events (such as pending txs, new blocks, marketplace orders, etc. ) and turn them into an internal event representation.
    Strategies: Strategies contain the core logic required for each MEV opportunity. They take in events as inputs, and compute whether any opportunities are available (for example, a strategy might listen to a stream of marketplace orders to see if there are any cross-exchange arbs). Strategies produce actions.
    Executors: Executors process actions, and are responsible for executing them in different domains (for example, submitting txs, posting off-chain orders, etc.).

Strategies

The following strategies have been implemented:

    Aori intra-orderbook arbitrage
