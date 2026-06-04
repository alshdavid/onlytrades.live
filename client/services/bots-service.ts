import { BehaviorSubject, Observable } from "rxjs";
import { useEffect } from "preact/hooks";
import { base64EncodeString } from "../kit/b64.ts";

type ApiBotsResponse = {
  bots: Array<Bot>;
};

export type Bot = {
  id: string;
  name: string;
  kind: string;
  language: string;
  created_at: string;
};

export type BotDetail = Bot & {
  handler: string | null;
};

export class BotsService {
  #bots: BehaviorSubject<Array<Bot> | undefined>;
  #botsLoaded: BehaviorSubject<boolean>;

  bots: Observable<Array<Bot> | undefined>;
  botsLoaded: Observable<boolean>;

  constructor() {
    this.#botsLoaded = new BehaviorSubject<boolean>(false);
    this.#bots = new BehaviorSubject<Array<Bot> | undefined>(undefined);

    this.bots = this.#bots;
    this.botsLoaded = this.#botsLoaded;
  }

  async init() {
    const response = await fetch("/api/bots");
    this.#botsLoaded.next(true);
    if (!response.ok) {
      this.#bots.error(new Error(`${response.status}`));
      return;
    }

    const data: ApiBotsResponse = await response.json();

    this.#bots.next(data.bots);
  }

  async getBot(id: string): Promise<BotDetail> {
    const response = await fetch(`/api/bots/${id}`);
    if (!response.ok) {
      throw new Error("Unable to get bot");
    }

    const body = (await response.json()) as BotDetail;
    if (body.handler) {
      body.handler = atob(body.handler);
    }

    return body;
  }

  async deleteBot(id: string): Promise<void> {
    const response = await fetch(`/api/bots/${id}`, { method: "DELETE" });
    if (!response.ok) {
      throw new Error("Unable to delete bot");
    }
  }

  async updateBot(id: string, code: string): Promise<void> {
    const response = await fetch(`/api/bots/${id}`, {
      method: "PATCH",
      body: JSON.stringify({
        handler: base64EncodeString(code),
      }),
    });
    if (!response.ok) {
      throw new Error("Unable to update bot");
    }
  }

  useInit = () => {
    useEffect(() => {
      if (this.#botsLoaded.value) {
        return;
      }
      this.init();
    }, [this]);
  };
}
