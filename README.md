# [OnlyTrades](https://onlytrades.live)

### Trade Automation [Refined.](https://onlytrades.live)

Homepage: [https://onlytrades.live](https://onlytrades.live)

_Note: The project is being actively developed so things will be improved. While it is in beta, there will be no major down time but updates will happen throughout the day so there may be a few seconds of downtime when the service updates. Once the service is ready, updates will be done on weekends during market breaks_

### Financing & Profit Sharing

While the project is open source and you are welcome to self-host, I offer a hosted version with billing based on compute usage. 

Subscribers to the hosted service are financing the development of this project.

Any contributions made will allow contributors to opt-into profit sharing from the hosted service. I haven't worked out the details 
on what that looks like exactly, but the concept is, you will get a % of profits proportionate to your contributions.

I will be transparent with project financing and will offer contributors reports on the finances.

### License & Usage

This project is distributed under an AGPLv3 copy-left license.

*It is free and permitted to self-host this project for personal use*

However, if the code is used for producing a commercial product, that fork must be made 
open-source and improvements must be contributed back to this codebase.

### Feedback

To request a feature, [post here](https://github.com/alshdavid/onlytrades.live/issues)

To report an issue, [post here](https://github.com/alshdavid/onlytrades.live/issues)

To see example bots and bot documentation, check out the [examples folder](https://github.com/alshdavid/onlytrades.live/tree/main/examples_bots).

The first version of the bot API is essentially a 1:1 mapping of the [CTrader Protobuf API](https://help.ctrader.com/open-api/messages/).

This API will be improved on but the initial release will be a simple mapping.

### Early Access

Early access is free. To join early access, you can contact me at alshdavid@gmail.com.

### Security & Performance

Everything is sandboxed and scoped. Bots cannot access other trader information. I would still recommend only trading with a demo account.

The service is written in Rust and has a low latency link to the broker server. Trades are executed over a persistent socket connect to the broker and each bot has its own socket connection.

### Supported Languages

Currently TypeScript is supported. I will be adding Rust bots too for extra speedy low latency/cost trading. Rust is also great for LLMs because they are able to work with the strict compiler to figure out what's wrong.

### TradingView

TradingView webhooks are supported. Just set up a trigger and paste the trigger code as the webhook body.

Currently only v5 pinescript is supported. For some reason, strategies written in pinescript v6 sends broken alert IDs.
