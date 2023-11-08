# Examples
The examples section should reflect the two-fold nature of this library. 

Some examples are focused on predominantly traders looking to take their on-chain trading to the next level, while others are more searchers-focused, showing how to use aori with the artemis framework to possibly fill and offer trades for the traders.



## The philosophy of having tons of examples
We firmly believe that every method should have a ready-to-run example that just literally after pressing "cargo run" - 
don't want you guys to be spending time on figuring out how to work with aori, instead I want you to be spending time on more useful acitvities (like which strategies to run and how o optimise them).

I've been personally confused by many codebases and really, if you think something is missing either create a new branch and send me a pull request or just post an issue! Aori is meant also for people who never coded before, so the examples section should hopefully reflect that.

Rust can be confusing enough for y'all to be confused about the codebase even more.

-> if there is something (can be any method, feature, anything) that doesn't have an example and you'd want to have it, post us an issue and we'll add it!


## How to run those examples
Navigate into the examples folder, then do the following:
```
export WALLET_ADDRESS=0x_your_wallet_address
export PRIVATE_KEY=your_private_key (without the 0x prefix)
export NODE_URL=(wss node url) // can use 'wss://ethereum-goerli.publicnode.com' for example //
```

```
cargo run -- bin EXAMPLE_NAME
```
for example:
```
cargo run --bin account_orders
```

Play around - adjust the example to fit your needs, try out new stuff, try to break things.

## Requests examples
These examples should cover all methods relevant for accessing the api, as well as multiple ways of accessing the endpoint - 
using the aori provider and pre-built methods, building the query yourself manually or using requests.rs and then sending using AoriProvider.send().

There are some benefits of utilising something like an aori collector from the artemis folder if you want to sync multiple data streams,
but for the basic bots, this should be sufficent.


## Artemis example
Artemis is a framework for writing MEV bots in Rust.

As of now, this part is work in progress, but soon, there will be a simple implementation of an aori artemis bot.

Currently there is a collector confirming to artemis standard by outputting a stream, you should be able to plug this in into any artemis strategy.

We will have AoriExecutor, as well as an example strategy soon.