import { Fragment, h } from "preact";
import { Navbar } from "../../components/navbar/navbar.tsx";
import { ProfileService } from "../../services/profile-service.ts";
import { useSubscribe } from "../../kit/mvvm/use-async.ts";
import { useInject } from "../../kit/mvvm/provider.ts";
import { Spinner } from "../../components/spinner/spinner.tsx";
import { Router } from "../../kit/router/router.ts";
import { BotsService } from "../../services/bots-service.ts";
import { Textarea } from "../../components/textarea/textarea.tsx";
import { useEffect, useState } from "preact/hooks";

export function BotsDetailIdPage() {
  const profileService = useInject(ProfileService);
  const botsService = useInject(BotsService);
  const router = useInject(Router);

  profileService.useInit();

  const [profile, profileError] = useSubscribe(profileService.profile, [
    profileService,
  ]);
  const [bot, _botError] = useSubscribe(
    () => botsService.getBot(router.req.params.id),
    [botsService, router.req.params.id],
  );

  if (profileError) {
    window.location.assign("/");
    return null;
  }

  if (!profile || !bot) {
    return (
      <Fragment>
        <Navbar />
        <main className="loading">
          <Spinner />
        </main>
      </Fragment>
    );
  }

  const [handlerCode, setHandlerCode] = useState("");

  useEffect(() => {
    setHandlerCode(bot.handler || "");
  }, [bot]);

  async function deleteBot() {
    if (!confirm("Are you sure you want to delete this bot?")) return;
    await botsService.deleteBot(router.req.params.id);
    window.location.assign("/bots");
  }

  async function updateBot() {
    if (
      !confirm(
        "Are you sure you want to update this bot? It will update all deployments",
      )
    )
      return;
    await botsService.updateBot(router.req.params.id, handlerCode);
  }

  return (
    <Fragment>
      <Navbar />
      <main>
        <div className="content-max-width l">
          <section className="content-panel list-block">
            <div className="panel-header">
              <h2>Bot Details</h2>
              <div className="btns">
                <button className="btn green" onClick={updateBot}>
                  UPDATE
                </button>
                <a
                  className="btn blue"
                  href={`/bots/deploy/${router.req.params.id}`}
                >
                  DEPLOY
                </a>
                <button className="btn red solid" onClick={deleteBot}>
                  DELETE
                </button>
              </div>
            </div>

            <label>
              <h3>Bot ID</h3>
              <input type="text" disabled value={bot.id} />
            </label>

            <label>
              <h3>Bot Name</h3>
              <input type="text" disabled value={bot.name} />
            </label>

            <label>
              <h3>Bot Created At</h3>
              <input type="text" disabled value={bot.created_at} />
            </label>

            <label>
              <h3>Bot Language</h3>
              <input type="text" disabled value={bot.language} />
            </label>

            <label>
              <h3>Bot Code</h3>
              <Textarea
                onInput={(e) => setHandlerCode(e.currentTarget.value)}
                value={handlerCode}
              />
            </label>
          </section>
        </div>
      </main>
    </Fragment>
  );
}
