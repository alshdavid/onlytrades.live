import type { IConnection } from "../../process/deno/code/types.ts";
import type { MessageEvent, IContext, SpotsOptions, SpotEvent } from "../../../../plugins/types.ts";

export class Context  implements IContext {
  #conn: IConnection;
  readonly account_id: string

  constructor(conn: IConnection) {
    this.#conn = conn
    this.account_id = ''
  }

  symbolsList(): Promise<Map<string, number>> {  
    return new Promise((res) => {
      const id = crypto.randomUUID();
      const dispose = this.#conn.subscribe((msg: MessageEvent) => {
        if (msg[2] !== id) {
          return
        }
        dispose()
        res(new Map(Object.entries(msg[1] || {})))
      });

      this.#conn.write(["symbols_list", null, id])
    })
  }

  onSpotEvent(callback: (value: SpotEvent) => unknown | Promise<unknown>): () => void {
    return this.#conn.subscribe((event: MessageEvent) => {
      if (event[0] === "spot_event") {
        callback(event[1] as SpotEvent)
      }
    })
  }

  subscribeSpots(options: SpotsOptions) {
    return new Promise<void>((res) => {
      const id = crypto.randomUUID();
      const dispose = this.#conn.subscribe((msg: MessageEvent) => {
        if (msg[2] !== id) {
          return
        }
        dispose()
        res()
      });

      this.#conn.write(["subscribe_spots", options, id])
    })
  }
}
