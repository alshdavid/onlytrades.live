import { Fragment, h } from "preact";
import { Navbar } from "../../components/navbar/navbar.tsx";
import { ProfileService } from "../../services/profile-service.ts";
import { useSubscribe } from "../../kit/mvvm/use-async.ts";
import { useInject } from "../../kit/mvvm/provider.ts";
import { Spinner } from "../../components/spinner/spinner.tsx";
import { Router } from "../../kit/router/router.ts";
import { DeploymentsService } from "../../services/deployments-service.ts";
import { CTraderService } from "../../services/ctrader-service.ts";
import { useEffect, useState } from "preact/hooks";
import { Log, LogService } from "../../services/logs-service.ts";
import { classNames } from "../../kit/class-names.ts";

export function BotsDeploymentIdPage() {
  const router = useInject(Router);
  const deploymentId = router.req.params.id;

  const profileService = useInject(ProfileService);
  const ctraderService = useInject(CTraderService);
  const deploymentService = useInject(DeploymentsService);
  const logService = useInject(LogService);

  profileService.useInit();
  ctraderService.useInit();

  const [profile, profileError] = useSubscribe(profileService.profile, [
    profileService,
  ]);

  const [ctraderAccounts, ctraderAccountsError] = useSubscribe(
    ctraderService.accounts,
    [ctraderService],
  );

  const [deployment, _deploymentError] = useSubscribe(
    () => deploymentService.getDeployment(deploymentId),
    [deploymentService, deploymentId],
  );

  if (profileError || ctraderAccountsError) {
    window.location.assign("/");
    return null;
  }

  if (!profile || !deployment || !ctraderAccounts) {
    return (
      <Fragment>
        <Navbar />
        <main className="loading">
          <Spinner />
        </main>
      </Fragment>
    );
  }

  const account = ctraderAccounts.find(
    (a) => a.account_id === deployment.account_id,
  );

  async function deleteDeployment() {
    await deploymentService.deleteDeployment(deploymentId);
    window.location.assign("/bots");
  }

  async function pauseDeployment() {
    await deploymentService.pauseDeployment(deploymentId);
    window.location.reload();
  }

  async function resumeDeployment() {
    await deploymentService.resumeDeployment(deploymentId);
    window.location.reload();
  }

  const [logs, setLogs] = useState<Array<Log>>([]);

  useEffect(() => {
    const stream = logService.getLogStream(`deployments:${deploymentId}`);
    const sub = stream.subscribe((l) =>
      setLogs((logs) => {
        const update = [...logs, l];
        update.sort(
          (a, b) =>
            new Date(b.created_at).getTime() - new Date(a.created_at).getTime(),
        );
        return update;
      }),
    );
    return () => sub.unsubscribe();
  }, [logService, deploymentId]);

  return (
    <Fragment>
      <Navbar />
      <main>
        <div className="content-max-width l">
          <section className="content-panel list-block">
            <div className="panel-header">
              <h2>Bot Deployment Details</h2>
              <div className="btns">
                {!deployment.running && deployment.active && (
                  <button className="btn green" onClick={resumeDeployment}>
                    RESTART
                  </button>
                )}
                {deployment.active ? (
                  <button className="btn red" onClick={pauseDeployment}>
                    DEACTIVATE
                  </button>
                ) : (
                  <button className="btn green" onClick={resumeDeployment}>
                    ACTIVATE
                  </button>
                )}

                <button className="btn red solid" onClick={deleteDeployment}>
                  DELETE
                </button>
              </div>
            </div>

            <label>
              <h3>Deployment ID</h3>
              <input type="text" disabled value={deployment.id} />
            </label>

            <label>
              <h3>Bot ID</h3>
              <input type="text" disabled value={deployment.bot_id} />
            </label>

            <label>
              <h3>Name</h3>
              <input type="text" disabled value={deployment.name} />
            </label>

            <label>
              <h3>Account</h3>
              <input
                type="text"
                disabled
                value={`${deployment.account_id} | ${account?.broker_title} : ${account?.account_number}`}
              />
            </label>

            <label>
              <h3>Created At</h3>
              <input type="text" disabled value={deployment.created_at} />
            </label>

            <label>
              <h3>Enabled</h3>
              <input
                type="text"
                disabled
                value={deployment.active ? "Enabled" : "Paused"}
              />
            </label>

            <label>
              <h3>Status</h3>
              <input
                type="text"
                disabled
                value={deployment.running ? "Running" : "Exited"}
              />
            </label>

            <label>
              <h3>Logs</h3>
              <div className="logs">
                {logs.map((log) => (
                  <div
                    className={classNames(
                      "entry",
                      log.log_level === 2 ? "stdout" : undefined,
                      log.log_level === 0 ? "stderr" : undefined,
                    )}
                  >
                    <time datetime={log.created_at}>{log.created_at}</time>
                    <code>{log.message}</code>
                  </div>
                ))}
              </div>
            </label>
          </section>
        </div>
      </main>
    </Fragment>
  );
}
