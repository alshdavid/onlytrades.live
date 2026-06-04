import { Fragment, h } from "preact";
import { Navbar } from "../../components/navbar/navbar.tsx";
import { ProfileService } from "../../services/profile-service.ts";
import { useSubscribe } from "../../kit/mvvm/use-async.ts";
import { useInject } from "../../kit/mvvm/provider.ts";
import { Spinner } from "../../components/spinner/spinner.tsx";
import { BotsService } from "../../services/bots-service.ts";
import { DeploymentsService } from "../../services/deployments-service.ts";

const deployedBots: any[] = [
  // {
  //   id: "12345",
  //   name: "US500 1hr",
  //   bot_name: "EMA 9/20",
  //   type: "live", // "live" | "candle-close"
  //   account: "412239994",
  //   status: "active",
  // }
];

export function BotsPage() {
  const profileService = useInject(ProfileService);
  const [profile, profileError] = useSubscribe(profileService.profile, [
    profileService,
  ]);

  const botsService = useInject(BotsService);
  const [bots = [], botsError] = useSubscribe(botsService.bots, [botsService]);

  const deploymentsService = useInject(DeploymentsService);
  const [deployments = [], deploymentsError] = useSubscribe(
    deploymentsService.deployments,
    [botsService],
  );

  profileService.useInit();
  botsService.useInit();
  deploymentsService.useInit();

  if (profileError || botsError || deploymentsError) {
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

  return (
    <Fragment>
      <Navbar />
      <main>
        <div className="content-max-width l">
          <section className="content-panel list-block">
            <div className="panel-header">
              <h2>Bots</h2>
              <a className="btn blue solid" href="/bots/new">
                CREATE BOT +
              </a>
            </div>

            <table>
              <thead>
                <tr>
                  <th>Name</th>
                  <th className="wrap">Language</th>
                  <th className="wrap">Type</th>
                  <th className="wrap"></th>
                </tr>
              </thead>
              <tbody>
                {bots.map((t) => (
                  <tr>
                    <td>{t.name}</td>
                    {/* <td className="wrap">{t.symbol}</td> */}
                    <td className="wrap">{t.language}</td>
                    <td className="wrap">{t.kind}</td>
                    <td className="wrap">
                      <a className="btn blue" href={`/bots/deploy/${t.id}`}>
                        DEPLOY
                      </a>
                      <a className="btn blue" href={`/bots/detail/${t.id}`}>
                        CONFIGURE
                      </a>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </section>

          <section className="content-panel list-block">
            <div className="panel-header">
              <h2>Deployed Bots</h2>
            </div>

            <table>
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Bot Name</th>
                  <th className="wrap">Account</th>
                  <th className="wrap">Type</th>
                  <th className="wrap">Status</th>
                  <th className="wrap"></th>
                </tr>
              </thead>
              <tbody>
                {deployments.map((t) => (
                  <tr>
                    <td>{t.name}</td>
                    <td>{bots.find((b) => b.id === t.bot_id)?.name}</td>
                    {/* <td className="wrap">{t.symbol}</td> */}
                    <td className="wrap">{t.account_id}</td>
                    <td className="wrap">
                      {bots.find((b) => b.id === t.bot_id)?.kind}
                    </td>
                    <td className="wrap">{t.active ? "Active" : "Paused"}</td>
                    <td className="wrap">
                      <a className="btn blue" href={`/bots/deployment/${t.id}`}>
                        CONFIGURE
                      </a>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </section>
        </div>
      </main>
    </Fragment>
  );
}
