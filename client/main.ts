import { h, render, type ComponentType} from "preact";

import { Provider } from './kit/mvvm/provider.ts'
import { Router } from './kit/router/router.ts'
import { DashboardPage } from './pages/dashboard/dashboard.tsx'
import { MarketplacePage } from './pages/marketplace/marketplace.tsx'
import { BotsPage } from './pages/bots/bots.tsx'
import { BotsDetailIdPage } from './pages/bots_detail_id/bots_detail_id.tsx'
import { ProfileService } from './services/profile-service.ts'
import { TriggersPage } from './pages/triggers/triggers.tsx'
import { TriggersDetailIdPage } from "./pages/triggers_detail_id/triggers_detail_id.tsx";
import { TriggersNewPage } from "./pages/triggers_new/triggers_new.tsx";
import { CTraderService } from "./services/ctrader-service.ts";
import { AdminPage } from "./pages/admin/admin.tsx";
import { TriggersService } from "./services/triggers-service.ts";
import { LogService } from "./services/logs-service.ts";
import { BotsNewPage } from "./pages/bots_new/bots_new.tsx";
import { BotsService } from "./services/bots-service.ts";
import { BotsDeployIdPage } from "./pages/bots_deploy_id/bots_deploy_id.tsx";
import { DeploymentsService } from "./services/deployments-service.ts";
import { BotsDeploymentIdPage } from "./pages/bots_deployment_id/bots_deployment_id.tsx";

const app = new Router()

function bootstrap(Component: ComponentType<any>) {
  const profileService = new ProfileService()
  const ctraderService = new CTraderService()
  const triggersService = new TriggersService()
  const botsService = new BotsService()
  const deploymentsService = new DeploymentsService()
  const logsService = new LogService()

  const provider = new Provider()
  provider.set(ProfileService, profileService)
  provider.set(CTraderService, ctraderService)
  provider.set(TriggersService, triggersService)
  provider.set(BotsService, botsService)
  provider.set(DeploymentsService, deploymentsService)
  provider.set(LogService, logsService)
  provider.set(Router, app)

  // @ts-expect-error
  globalThis.provider = provider

  const sub = profileService.profile.subscribe(async profile => {
    if (!profile?.email) return
    sub.unsubscribe()
    
    const msgBuffer = new TextEncoder().encode(profile.email.trim().toLowerCase());
    const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    const hashed = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');

    try {
      // @ts-expect-error
      globalThis.umami?.identify?.({ id: hashed });
    } catch {
      // Do nothing
    }
  })
  profileService.init()

  render(h(Provider.Provider, { value: provider }, [h(Component, {})]), document.body);
}

app.route("/dashboard", () => bootstrap(DashboardPage))
app.route("/marketplace", () => bootstrap(MarketplacePage))
app.route("/bots", () => bootstrap(BotsPage))
app.route("/bots/detail/:id", () => bootstrap(BotsDetailIdPage))
app.route("/bots/deploy/:id", () => bootstrap(BotsDeployIdPage))
app.route("/bots/deployment/:id", () => bootstrap(BotsDeploymentIdPage))
app.route("/bots/new", () => bootstrap(BotsNewPage))
app.route("/triggers", () => bootstrap(TriggersPage))
app.route("/triggers/detail/:id", () => bootstrap(TriggersDetailIdPage))
app.route("/triggers/new", () => bootstrap(TriggersNewPage))

app.route("/admin", () => bootstrap(AdminPage))

app.start()


