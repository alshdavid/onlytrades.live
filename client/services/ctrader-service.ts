import { BehaviorSubject, Observable } from "rxjs";
import { useEffect } from "preact/hooks";

type ApiCtraderAccountsGetResponse = {
  ctrader_accounts: Array<CtraderAccount> | undefined;
};

export type CtraderAccount = {
  account_id: number;
  account_number: number;
  live: boolean;
  broker_name: string;
  broker_title: string;
  deposit_currency: string;
  trader_account_type: string;
  leverage: number;
  leverage_in_cents: number;
  balance: number;
  deleted: boolean;
  account_status: string;
  swap_free: boolean;
  money_digits: number;
};

export class CTraderService {
  #accounts: BehaviorSubject<Array<CtraderAccount> | undefined>;
  #accountsLoaded: BehaviorSubject<boolean>;
  accounts: Observable<Array<CtraderAccount> | undefined>;
  accountsLoaded: Observable<boolean>;

  constructor() {
    this.#accountsLoaded = new BehaviorSubject<boolean>(false);
    this.#accounts = new BehaviorSubject<Array<CtraderAccount> | undefined>(
      undefined,
    );
    this.accounts = this.#accounts;
    this.accountsLoaded = this.#accountsLoaded;
  }

  async init() {
    const response = await fetch("/api/ctrader/accounts");
    this.#accountsLoaded.next(true);

    if (!response.ok) {
      this.#accounts.error(new Error(`${response.status}`));
      return;
    }

    const data: ApiCtraderAccountsGetResponse = await response.json();

    if (!data.ctrader_accounts) {
      this.#accounts.next([]);
      return;
    }

    data.ctrader_accounts = [
      ...data.ctrader_accounts
        .filter((a) => a.live)
        .sort((a, b) => a.broker_name.localeCompare(b.broker_name)),
      ...data.ctrader_accounts
        .filter((a) => !a.live)
        .sort((a, b) => a.broker_name.localeCompare(b.broker_name)),
    ];

    this.#accounts.next(data.ctrader_accounts);
  }

  useInit = () => {
    useEffect(() => {
      if (this.#accountsLoaded.value) {
        return;
      }
      this.init();
    }, [this]);
  };
}

export function formatProfileBalance(
  balance: number,
  money_digits: number,
): string {
  const trueAmount = balance / Math.pow(10, money_digits);

  return new Intl.NumberFormat("en-US", {
    minimumFractionDigits: money_digits,
    maximumFractionDigits: money_digits,
  }).format(trueAmount);
}
