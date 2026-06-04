import { BehaviorSubject, Observable } from "rxjs";
import { useEffect } from "preact/hooks";

export type Profile = {
  id: string;
  email: string;
  identities: Array<ProfileIdentity>;
  permissions: Set<ProfilePermission>;
};

export type ProfileIdentity = {
  sub: string;
  provider: string;
  last_login: string;
};

export type ProfilePermission =
  (typeof ProfilePermission)[keyof typeof ProfilePermission];
export const ProfilePermission = Object.freeze({
  Admin: "admin",
} as const);

export class ProfileService {
  #profile: BehaviorSubject<Profile | undefined>;
  #profileLoaded: BehaviorSubject<boolean>;

  profile: Observable<Profile | undefined>;
  profileLoaded: Observable<boolean>;

  constructor() {
    this.#profileLoaded = new BehaviorSubject<boolean>(false);
    this.#profile = new BehaviorSubject<Profile | undefined>(undefined);
    this.profile = this.#profile;
    this.profileLoaded = this.#profileLoaded;
  }

  async init() {
    const response = await fetch("/api/auth/me");
    this.#profileLoaded.next(true);

    if (!response.ok) {
      this.#profile.error(new Error(`${response.status}`));
      return;
    }

    const data: Profile = await response.json();

    this.#profile.next({
      ...data,
      permissions: new Set(data.permissions),
    });
  }

  useInit = () => {
    useEffect(() => {
      if (this.#profileLoaded.value) {
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
