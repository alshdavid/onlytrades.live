import { Fragment, h } from "preact";
import { Navbar } from "../../components/navbar/navbar.tsx";
import { ProfileService } from "../../services/profile-service.ts";
import { useSubscribe } from "../../kit/mvvm/use-async.ts";
import { useInject } from "../../kit/mvvm/provider.ts";
import { Spinner } from "../../components/spinner/spinner.tsx";
import { TriggersService } from "../../services/triggers-service.ts";

export function TriggersPage() {
  const profileService = useInject(ProfileService);
  const triggersService = useInject(TriggersService);

  profileService.useInit();
  triggersService.useInit();

  const [profile, profileError] = useSubscribe(profileService.profile, [
    profileService,
  ]);
  const [profileLoaded = false] = useSubscribe(profileService.profileLoaded, [
    profileService,
  ]);

  const [triggers = [], triggersError] = useSubscribe(
    triggersService.triggers,
    [triggersService],
  );
  const [triggersLoaded = false] = useSubscribe(
    triggersService.triggersLoaded,
    [triggersService],
  );

  if ((profileLoaded && profileError) || (triggersLoaded && triggersError)) {
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
              <h2>Triggers</h2>
              <a className="btn blue solid" href="/triggers/new">
                CREATE TRIGGER +
              </a>
            </div>

            <table>
              <thead>
                <tr>
                  <th>Title</th>
                  {/* <th className="wrap">Symbol</th> */}
                  <th className="wrap">Platform</th>
                  <th className="wrap">Type</th>
                  <th className="wrap">Status</th>
                  <th className="wrap"></th>
                </tr>
              </thead>
              <tbody>
                {triggers.map((t) => (
                  <tr>
                    <td>{t.name}</td>
                    {/* <td className="wrap">{t.symbol}</td> */}
                    <td className="wrap">{t.platform}</td>
                    <td className="wrap">Webhook</td>
                    <td className="wrap">{t.status}</td>
                    <td className="wrap">
                      <a className="btn blue" href={`/triggers/detail/${t.id}`}>
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
