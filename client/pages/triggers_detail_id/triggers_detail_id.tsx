import { Fragment, h } from "preact";
import { Navbar } from "../../components/navbar/navbar.tsx";
import { ProfileService } from "../../services/profile-service.ts";
import { useSubscribe } from "../../kit/mvvm/use-async.ts";
import { useInject } from "../../kit/mvvm/provider.ts";
import { Spinner } from "../../components/spinner/spinner.tsx";
import { Router } from "../../kit/router/router.ts";
import { TriggersService } from "../../services/triggers-service.ts";
import { LogService } from "../../services/logs-service.ts";

export function TriggersDetailIdPage() {
  const profileService = useInject(ProfileService);
  const triggerService = useInject(TriggersService);
  const logService = useInject(LogService);
  const router = useInject(Router);

  profileService.useInit();

  const [profile, profileError] = useSubscribe(profileService.profile, [
    profileService,
  ]);
  const [trigger, triggerError] = useSubscribe(
    () => triggerService.getTrigger(router.req.params.id),
    [triggerService, router.req.params.id],
  );
  const [logs, logsError] = useSubscribe(
    () => logService.getLogs(`trigger:${router.req.params.id}`),
    [logService, router.req.params.id],
  );

  const tradingViewSnippet = JSON.stringify(
    {
      trigger_id: trigger?.id,
      order_id: "{{strategy.order.id}}",
      action: "{{strategy.order.action}}",
      ticker: "{{ticker}}",
      position_size: "100",
      timestamp: "{{time}}",
    },
    null,
    2,
  );

  if (profileError) {
    window.location.assign("/");
    return null;
  }

  if (!profile || !trigger) {
    return (
      <Fragment>
        <Navbar />
        <main className="loading">
          <Spinner />
        </main>
      </Fragment>
    );
  }

  async function deleteTrigger() {
    await triggerService.deleteTrigger(router.req.params.id);
    window.location.assign("/triggers");
  }

  return (
    <Fragment>
      <Navbar />
      <main>
        <div className="content-max-width l">
          <section className="content-panel list-block">
            <div className="panel-header">
              <h2>Trigger Details</h2>
              <div className="btns">
                <button className="btn red solid" onClick={deleteTrigger}>
                  DELETE
                </button>
                {/* {trigger.status === "active" ? (
                  <button className="btn red">PAUSE</button>
                ) : (
                  <button className="btn blue">ACTIVATE</button>
                )} */}
              </div>
            </div>

            <label>
              <h3>Trigger Name</h3>
              <input type="text" disabled value={trigger.name} />
            </label>

            <label>
              <h3>Trigger ID</h3>
              <input type="text" disabled value={trigger.id} />
            </label>

            <label>
              <h3>Trigger Created At</h3>
              <input type="text" disabled value={trigger.created_at} />
            </label>

            <label>
              <h3>Trigger Status</h3>
              <input type="text" disabled value={trigger.status} />
            </label>

            <label>
              <h3>Trigger Account</h3>
              <input type="text" disabled value={trigger.account_id} />
            </label>

            <label>
              <h3>Trigger Platform</h3>
              <input type="text" disabled value={trigger.platform} />
            </label>

            <label>
              <h3>Webhook URL</h3>
              <input
                type="text"
                disabled
                value={`https://onlytrades.live/webhooks`}
              />
            </label>

            <label>
              <h3>Message Body</h3>
              <textarea>{tradingViewSnippet}</textarea>
            </label>

            <br />
            <hr />
            <br />
            <h3>Logs</h3>

            <div className="input">
              <pre>
                <code>
                  {!logs?.length
                    ? ""
                    : logs
                        .map(
                          (log) =>
                            `[${log.log_level}] [${log.created_at}] ${log.message}`,
                        )
                        .join("\n")}
                </code>
              </pre>
            </div>
          </section>
        </div>
      </main>
    </Fragment>
  );
}
