import { Fragment, h } from "preact";
import { Navbar } from "../../components/navbar/navbar.tsx";
import { ProfileService } from "../../services/profile-service.ts";
import { useSubscribe } from "../../kit/mvvm/use-async.ts";
import { useInject } from "../../kit/mvvm/provider.ts";
import { Spinner } from "../../components/spinner/spinner.tsx";

export function MarketplacePage() {
  const profileService = useInject(ProfileService);
  const [profile, profileError] = useSubscribe(profileService.profile, [
    profileService,
  ]);

  profileService.useInit();

  if (profileError) {
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
          <h1>Marketplace - Coming Soon</h1>
          <br />
          <p>
            Bots are algorithms running on the OnlyTrades server that execute
            trades
          </p>
          <br />
          <p>
            You can get a Bot from the Marketplace or publish a Bot to the
            Marketplace and earn income from it
          </p>
          <br />
          <p>
            Bots have a direct link to the broker API to maximize performance
            and reduce latency
          </p>
        </div>
      </main>
    </Fragment>
  );
}
