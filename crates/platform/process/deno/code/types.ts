export type SerializablePrimitive = string | number | boolean | null;

export type SerializableObject = { [key: string]: Serializable };

export type SerializableArray = Array<Serializable>;

export type Serializable =
  | SerializablePrimitive
  | SerializableObject
  | SerializableArray;

export interface IConnection {
  writeBytes(bytes: Uint8Array): void
  write(obj: Serializable): void
  subscribe(
    // deno-lint-ignore no-explicit-any
    callback: (value: any) => unknown | Promise<unknown>,
  ): () => void 
}
