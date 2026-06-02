# [OnlyTrades](https://onlytrades.live)

### Trade Automation [Refined.](https://onlytrades.live)

Homepage: [https://onlytrades.live](https://onlytrades.live)

_Note: The project is being actively developed so things will be improved. While it is in beta, there will be no major down time but updates will happen throughout the day so there may be a few seconds of downtime when the service updates. Once the service is ready, updates will be done on weekends during market breaks_

To request a feature, [post here](https://github.com/alshdavid/onlytrades.live/issues)

To report an issue, [post here](https://github.com/alshdavid/onlytrades.live/issues)

To see example bots and bot documentation, check out the [examples folder](https://github.com/alshdavid/onlytrades.live/tree/main/examples).

The first version of the bot API is essebtailly a 1:1 mapping of the [CTrader Protobuf API](https://help.ctrader.com/open-api/messages/).

This API will be improved on but the inital release will be a simple mapping.

### Early Access

Early access is free. To join early access, you can contact me at alshdavid@gmail.com.

### Security & Performance

Everything is sandboxed and scoped. Bots cannot access other trader information. I would still reccomend only trading with a demo account.

The service is written in Rust and has a low latency link to the broker server. Trades are executed over a persistant socket connect to the broker and each bot has its own socket connection.
