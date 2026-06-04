import { BehaviorSubject, Observable } from "rxjs";
import { useEffect } from "preact/hooks";

type ApiTriggersResponse = {
  triggers: Array<Trigger>;
};

export type Trigger = {
  id: string;
  name: string;
  platform: string;
  status: TriggerStatus;
  created_at: string;
  account_id: string;
};

export type TriggerPlatform =
  (typeof TriggerPlatform)[keyof typeof TriggerPlatform];
export const TriggerPlatform = Object.freeze({
  TradingView: "trading-view",
  Webhook: "webhook",
} as const);

export type TriggerStatus = (typeof TriggerStatus)[keyof typeof TriggerStatus];
export const TriggerStatus = Object.freeze({
  Active: "active",
  Paused: "paused",
} as const);

export class TriggersService {
  #triggers: BehaviorSubject<Array<Trigger> | undefined>;
  #triggersLoaded: BehaviorSubject<boolean>;

  triggers: Observable<Array<Trigger> | undefined>;
  triggersLoaded: Observable<boolean>;

  constructor() {
    this.#triggersLoaded = new BehaviorSubject<boolean>(false);
    this.#triggers = new BehaviorSubject<Array<Trigger> | undefined>(undefined);

    this.triggers = this.#triggers;
    this.triggersLoaded = this.#triggersLoaded;
  }

  async init() {
    const response = await fetch("/api/triggers");
    this.#triggersLoaded.next(true);
    if (!response.ok) {
      this.#triggers.error(new Error(`${response.status}`));
      return;
    }

    const data: ApiTriggersResponse = await response.json();

    this.#triggers.next(data.triggers);
  }

  async getTrigger(id: string): Promise<Trigger> {
    const response = await fetch(`/api/triggers/${id}`);
    if (!response.ok) {
      throw new Error("Unable to get trigger");
    }
    return response.json();
  }

  async deleteTrigger(id: string): Promise<void> {
    const response = await fetch(`/api/triggers/${id}`, {
      method: "DELETE",
    });
    if (!response.ok) {
      throw new Error("Unable to delete deployment");
    }
  }

  useInit = () => {
    useEffect(() => {
      if (this.#triggersLoaded.value) {
        return;
      }
      this.init();
    }, [this]);
  };
}
