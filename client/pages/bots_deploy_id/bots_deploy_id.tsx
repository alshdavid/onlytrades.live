import { Fragment, h, TargetedInputEvent } from "preact";
import { Navbar } from "../../components/navbar/navbar.tsx";
import {
  formatProfileBalance,
  ProfileService,
} from "../../services/profile-service.ts";
import { useSubscribe } from "../../kit/mvvm/use-async.ts";
import { useInject } from "../../kit/mvvm/provider.ts";
import { Spinner } from "../../components/spinner/spinner.tsx";
import { Router } from "../../kit/router/router.ts";
import { BotsService } from "../../services/bots-service.ts";
import { Textarea } from "../../components/textarea/textarea.tsx";
import { CTraderService } from "../../services/ctrader-service.ts";
import { base64EncodeString } from "../../kit/b64.ts";
import { useState } from "preact/hooks";
import { classNames } from "../../kit/class-names.ts";

export function BotsDeployIdPage() {
  const profileService = useInject(ProfileService);
  const botsService = useInject(BotsService);
  const ctraderService = useInject(CTraderService);
  const router = useInject(Router);

  profileService.useInit();
  ctraderService.useInit();

  const [profile, profileError] = useSubscribe(profileService.profile, [
    profileService,
  ]);
  const [bot, _botError] = useSubscribe(
    () => botsService.getBot(router.req.params.id),
    [botsService, router.req.params.id],
  );

  const [ctraderAccounts = [], ctraderAccountsError] = useSubscribe(
    ctraderService.accounts,
    [ctraderService],
  );
  const [accountsLoaded = false] = useSubscribe(ctraderService.accountsLoaded, [
    ctraderService,
  ]);

  if (profileError || (accountsLoaded && ctraderAccountsError)) {
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

  async function onSubmit(e: SubmitEvent) {
    e.preventDefault();
    const formElement = e.currentTarget as HTMLFormElement;
    const formData = new FormData(formElement);
    const formValues = Object.fromEntries(formData.entries());

    const payload = {
      bot_id: router.req.params.id,
      name: formValues.deployment_name.toString(),
      account_id: parseInt(formValues.account_id.toString(), 10),
      environment: base64EncodeString(
        JSON.stringify(formValues.deployment_environment),
      ),
    };

    let res = await fetch(`/api/deployments`, {
      method: "POST",
      body: JSON.stringify(payload),
    });
    if (!res.ok) {
      console.error("failed to submit request");
      return;
    }

    // const { id } = (await res.json()) as { id: string };
    window.location.assign(`/bots`);
  }

  const [envValid, setEnvValid] = useState(true);

  function validateEnv(e: TargetedInputEvent<HTMLTextAreaElement>) {
    try {
      JSON.parse(e.currentTarget.value);
      setEnvValid(true);
    } catch (error) {
      setEnvValid(false);
    }
  }

  const canSubmit = envValid && accountsLoaded;

  return (
    <Fragment>
      <Navbar />
      <main>
        <div className="content-max-width l">
          <section className="content-panel list-block">
            <div className="panel-header">
              <h2>Create Bot Deployment</h2>
            </div>

            <form onSubmit={onSubmit}>
              <label htmlFor="deployment_name">
                <h3>Deployment Name</h3>
                <input
                  id="deployment_name"
                  name="deployment_name"
                  type="text"
                  required
                />
              </label>

              <label htmlFor="deployment_account_id">
                <p>Account</p>
                <select name="account_id" id="deployment_account_id">
                  {!accountsLoaded ? (
                    <option>Loading...</option>
                  ) : (
                    ctraderAccounts.map((a) => (
                      <option value={a.account_id}>
                        {a.live ? "" : "DEMO - "}
                        {a.broker_title} {a.account_id} $
                        {formatProfileBalance(a.balance, a.money_digits)}
                      </option>
                    ))
                  )}
                </select>
              </label>

              <label htmlFor="deployment_environment">
                <h3>Environment</h3>
                <p>
                  JSON Object passed into the handler function under:{" "}
                  <code>Context.env</code>
                </p>
                <Textarea
                  className={classNames({ invalid: !envValid })}
                  name="deployment_environment"
                  id="deployment_environment"
                  onInput={validateEnv}
                  autoResize={true}
                  defaultValue={"{}"}
                />
              </label>

              <label>
                <h3>Handler</h3>
                <Textarea
                  disabled={true}
                  autoResize={true}
                  value={bot.handler || ""}
                />
              </label>

              <div className="btns">
                <button
                  disabled={!canSubmit}
                  type="submit"
                  className="btn blue solid"
                >
                  CREATE
                </button>
                <a href="/deployments" className="btn red solid">
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
