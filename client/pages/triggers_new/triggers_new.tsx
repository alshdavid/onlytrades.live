import { Fragment, h } from "preact";
import { Navbar } from "../../components/navbar/navbar.tsx";
import {
  formatProfileBalance,
  ProfileService,
} from "../../services/profile-service.ts";
import { useSubscribe } from "../../kit/mvvm/use-async.ts";
import { useInject } from "../../kit/mvvm/provider.ts";
import { Spinner } from "../../components/spinner/spinner.tsx";
import { useState } from "preact/hooks";
import { CTraderService } from "../../services/ctrader-service.ts";

export function TriggersNewPage() {
  const profileService = useInject(ProfileService);
  const ctraderService = useInject(CTraderService);

  profileService.useInit();
  ctraderService.useInit();

  const [profile, profileError] = useSubscribe(profileService.profile, [
    profileService,
  ]);
  const [profileLoaded = false] = useSubscribe(profileService.profileLoaded, [
    profileService,
  ]);

  const [ctraderAccounts = [], ctraderAccountsError] = useSubscribe(
    ctraderService.accounts,
    [ctraderService],
  );
  const [accountsLoaded = false] = useSubscribe(ctraderService.accountsLoaded, [
    ctraderService,
  ]);

  const [showDemoOnly, setShowDemoOnly] = useState<boolean>(false);

  if (
    (profileLoaded && profileError) ||
    (accountsLoaded && ctraderAccountsError)
  ) {
    window.location.assign("/");
    return null;
  }

  if (!profile) {
    return (
      <Fragment>
        <Navbar />
        <main className="loading">
          <Spinner />
        </main>
      </Fragment>
    );
  }

  const pageReady = profileLoaded && accountsLoaded;

  const ctraderAccountsFiltered = showDemoOnly
    ? ctraderAccounts.filter((a) => !a.live)
    : ctraderAccounts;

  async function onSubmit(e: SubmitEvent) {
    e.preventDefault();
    const formElement = e.currentTarget as HTMLFormElement;
    const formData = new FormData(formElement);
    const formValues = Object.fromEntries(formData.entries());

    const payload = {
      account_id: parseInt(formValues.account_id.toString(), 10),
      name: formValues.name.toString(),
      platform: formValues.platform.toString(),
    };

    let res = await fetch(`/api/triggers`, {
      method: "POST",
      body: JSON.stringify(payload),
    });
    if (!res.ok) {
      console.error("failed to submit request");
      return;
    }

    const { id } = (await res.json()) as { id: string };
    window.location.assign(`/triggers/detail/${id}`);
  }

  return (
    <Fragment>
      <Navbar />
      <main>
        <div className="content-max-width l">
          <section className="content-panel list-block">
            <div className="panel-header">
              <h2>Create Trigger</h2>
            </div>

            <form onSubmit={onSubmit}>
              <label htmlFor="trigger_name">
                <p>Trigger Name</p>
                <input
                  type="text"
                  name="name"
                  id="trigger_name"
                  required
                  placeholder="Enter Name for the Trigger"
                />
              </label>

              <label htmlFor="trigger_platform">
                <p>Platform</p>
                <select name="platform" id="trigger_platform">
                  <option value="trading-view">TradingView</option>
                  {/* <option value="generic">Generic</option> */}
                </select>
              </label>

              <label htmlFor="trigger_account_id">
                <p>Account</p>
                <label className="checkbox" htmlFor="trigger_only_demo">
                  <input
                    onClick={() => setShowDemoOnly(!showDemoOnly)}
                    type="checkbox"
                    name="trigger_only_demo"
                    id="trigger_only_demo"
                  />
                  <p>Only Show Demo Accounts</p>
                </label>
                <select name="account_id" id="trigger_account_id">
                  {!accountsLoaded ? (
                    <option>Loading...</option>
                  ) : (
                    ctraderAccountsFiltered.map((a) => (
                      <option value={a.account_id}>
                        {a.live ? "" : "DEMO - "}
                        {a.broker_title} {a.account_id} $
                        {formatProfileBalance(a.balance, a.money_digits)}
                      </option>
                    ))
                  )}
                </select>
              </label>

              <div className="btns">
                <button
                  disabled={!pageReady}
                  type="submit"
                  className="btn blue solid"
                >
                  CREATE
                </button>
                <a href="/triggers" className="btn red solid">
                  CANCEL
                </a>
              </div>
            </form>
          </section>
        </div>
      </main>
    </Fragment>
  );
}
