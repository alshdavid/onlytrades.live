import { Fragment, h } from "preact";
import { Navbar } from "../../components/navbar/navbar.tsx";
import {
  formatProfileBalance,
  ProfileService,
} from "../../services/profile-service.ts";
import { useSubscribe } from "../../kit/mvvm/use-async.ts";
import { useInject } from "../../kit/mvvm/provider.ts";
import { Spinner } from "../../components/spinner/spinner.tsx";
import { CTraderService } from "../../services/ctrader-service.ts";

export function DashboardPage() {
  const profileService = useInject(ProfileService);
  const ctraderService = useInject(CTraderService);

  profileService.useInit();
  ctraderService.useInit();

  const [profile, profileError] = useSubscribe(profileService.profile, [
    profileService,
  ]);
  const [ctraderAccounts, ctraderAccountsError] = useSubscribe(
    ctraderService.accounts,
    [ctraderService],
  );

  if (profileError || ctraderAccountsError) {
    window.location.assign("/");
    return null;
  }

  async function deleteCtraderAccount(id: string | number) {
    const res = await fetch(`/api/ctrader/account/${id}`, { method: "DELETE" });
    if (!res.ok) {
      return;
    }
    window.location.reload();
  }

  if (!profile || !ctraderAccounts) {
    return (
      <Fragment>
        <Navbar />
        <main className="loading">
          <Spinner />
        </main>
      </Fragment>
    );
  }

  return (
    <Fragment>
      <Navbar />
      <main>
        <div className="content-max-width l">
          <section className="content-panel list-block">
            <div className="panel-header">
              <h2>Account</h2>
            </div>

            <div className="label">
              <h3>Email</h3>
              <input type="text" disabled={true} value={profile.email} />
            </div>

            {/* <div className="label account-balance">
              <h3>Account Balance</h3>
              <input type="text" disabled={true} value={"$10.00"} />
              <button className="btn blue solid">Add Balance</button>
              <button className="btn blue">Withdraw</button>
            </div> */}

            <div className="label">
              <h3>cTrader Connection</h3>
              <a className="ctrader-button" href="/api/ctrader/connect">
                <img
                  height="20px"
                  src="/assets/ctrader.b569153a9afbb3a43bb6.svg"
                  alt="ctrader-api logo"
                />
                <span>Connect</span>
              </a>
              <p>
                <i>Note: To add new accounts, you must connect again</i>
              </p>

              <table className="ctrader-accounts-table">
                <thead>
                  <tr>
                    <th>Account ID</th>
                    <th className="wrap">Broker</th>
                    <th className="wrap">Currency</th>
                    <th className="wrap">Balance</th>
                    <th className="wrap">Account Type</th>
                    <th className="wrap"></th>
                  </tr>
                </thead>
                <tbody>
                  {ctraderAccounts
                    .filter((a) => a.live)
                    .map((ctraderAccount) => (
                      <tr>
                        <td className="wrap">{ctraderAccount.account_id}</td>
                        <td>{ctraderAccount.broker_title}</td>
                        <td className="wrap">
                          {ctraderAccount.deposit_currency}
                        </td>
                        <td className="wrap">
                          {formatProfileBalance(
                            ctraderAccount.balance,
                            ctraderAccount.money_digits,
                          )}
                        </td>
                        <td className="wrap">
                          {ctraderAccount.live
                            ? "Live Account"
                            : "Demo Account"}
                        </td>
                        <td className="align-right">
                          <button
                            className="btn red solid"
                            onClick={() =>
                              deleteCtraderAccount(ctraderAccount.account_id)
                            }
                          >
                            REMOVE
                          </button>
                        </td>
                      </tr>
                    ))}
                  {ctraderAccounts.filter((a) => a.live).length ? null : (
                    <tr>
                      <td className="wrap">No accounts</td>
                      <td></td>
                      <td className="wrap"></td>
                      <td className="wrap"></td>
                      <td className="wrap"></td>
                      <td className="align-right"></td>
                    </tr>
                  )}
                </tbody>
              </table>

              <table className="ctrader-accounts-table">
                <thead>
                  <tr>
                    <th>Account ID</th>
                    <th className="wrap">Broker</th>
                    <th className="wrap">Currency</th>
                    <th className="wrap">Balance</th>
                    <th className="wrap">Account Type</th>
                    <th className="wrap"></th>
                  </tr>
                </thead>
                <tbody>
                  {ctraderAccounts
                    .filter((a) => !a.live)
                    .map((ctraderAccount) => (
                      <tr>
                        <td className="wrap">{ctraderAccount.account_id}</td>
                        <td>{ctraderAccount.broker_title}</td>
                        <td className="wrap">
                          {ctraderAccount.deposit_currency}
                        </td>
                        <td className="wrap">
                          {formatProfileBalance(
                            ctraderAccount.balance,
                            ctraderAccount.money_digits,
                          )}
                        </td>
                        <td className="wrap">
                          {ctraderAccount.live
                            ? "Live Account"
                            : "Demo Account"}
                        </td>
                        <td className="align-right">
                          <button
                            className="btn red solid"
                            onClick={() =>
                              deleteCtraderAccount(ctraderAccount.account_id)
                            }
                          >
                            REMOVE
                          </button>
                        </td>
                      </tr>
                    ))}
                  {ctraderAccounts.filter((a) => !a.live).length ? null : (
                    <tr>
                      <td className="wrap">No accounts</td>
                      <td></td>
                      <td className="wrap"></td>
                      <td className="wrap"></td>
                      <td className="wrap"></td>
                      <td className="align-right"></td>
                    </tr>
                  )}
                </tbody>
              </table>
            </div>
          </section>
        </div>
      </main>
    </Fragment>
  );
}
