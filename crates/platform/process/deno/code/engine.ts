export type SerializablePrimitive = string | number | boolean | null;

export type SerializableObject = { [key: string]: Serializable };

export type SerializableArray = Array<Serializable>;

export type Serializable =
  | SerializablePrimitive
  | SerializableObject
  | SerializableArray;

export {}

class Connection {
  #conn: Promise<Deno.Conn>;
  #callbacks: Set<(value: unknown) => unknown | Promise<unknown>>;

  constructor({
    hostname,
    port,
  }: {
    hostname: string | undefined;
    port: string | undefined;
  }) {
    if (!port || !hostname) {
      throw new Error("env.PORT / env.HOSTNAME not provided");
    }
    this.#conn = Deno.connect({
      hostname,
      port: parseInt(port, 10),
    });

    this.#callbacks = new Set();

    setTimeout(async () => {
      const conn = await this.#conn;
      const decoder = new TextDecoder();

      while (true) {
        const lengthBytes = new Uint8Array(4);
        const success = await this.#readExact(conn, lengthBytes);
        if (!success) {
          console.log("Server closed the connection.");
          break;
        }

        const view = new DataView(lengthBytes.buffer);
        const bodyLength = view.getUint32(0, false); // false = Big-Endian

        const bodyBytes = new Uint8Array(bodyLength);
        const bodySuccess = await this.#readExact(conn, bodyBytes);
        if (!bodySuccess) {
          console.error("Connection closed prematurely while reading body.");
          break;
        }

        const jsonString = decoder.decode(bodyBytes);
        const message = JSON.parse(jsonString);
        for (const callback of this.#callbacks) {
          setTimeout(callback, 0, message);
        }
      }
    });
  }

  async writeBytes(bytes: Uint8Array) {
    const data = this.#prepareBytes(bytes);
    return (await this.#conn).write(data);
  }

  write(obj: Serializable) {
    const jsonString = JSON.stringify(obj);
    const encoder = new TextEncoder();
    const dataBytes = encoder.encode(jsonString);
    return this.writeBytes(dataBytes);
  }

  subscribe(
    // deno-lint-ignore no-explicit-any
    callback: (value: any) => unknown | Promise<unknown>,
  ): () => void {
    this.#callbacks.add(callback);
    return () => this.#callbacks.delete(callback);
  }

  #prepareBytes(bytes: Uint8Array): Uint8Array {
    const lengthBuffer = new ArrayBuffer(4);
    const view = new DataView(lengthBuffer);
    view.setUint32(0, bytes.length, false); // false specifies Big-Endian
    const lengthBytes = new Uint8Array(lengthBuffer);

    const combined = new Uint8Array(lengthBytes.length + bytes.length);
    combined.set(lengthBytes, 0);
    combined.set(bytes, lengthBytes.length);

    return combined;
  }

  async #readExact(conn: Deno.Conn, buffer: Uint8Array): Promise<boolean> {
    let bytesReadTotal = 0;

    while (bytesReadTotal < buffer.length) {
      const chunk = await conn.read(buffer.subarray(bytesReadTotal));

      if (chunk === null) {
        if (bytesReadTotal > 0) {
          throw new Error("Unexpected EOF: Stream closed mid-packet.");
        }
        return false;
      }

      bytesReadTotal += chunk;
    }

    return true;
  }
}

// @ts-expect-error Setting on globalThis
globalThis.conn = new Connection({ 
  hostname: Deno.env.get("HOSTNAME"), 
  port: Deno.env.get("PORT")
});

