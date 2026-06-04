import { Fragment, h } from "preact";
import { Navbar } from "../../components/navbar/navbar.tsx";
import { ProfileService } from "../../services/profile-service.ts";
import { useSubscribe } from "../../kit/mvvm/use-async.ts";
import { useInject } from "../../kit/mvvm/provider.ts";
import { Spinner } from "../../components/spinner/spinner.tsx";
import { CTraderService } from "../../services/ctrader-service.ts";

const defaultCode = `
export default async function handler(context: Context): Promise<void> {
  while (true) {
    console.log('BOT: hi')
    await new Promise(res => setTimeout(res, 1000))
  }
}
`.trim();

export function BotsNewPage() {
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

  if (profileLoaded && profileError) {
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

  async function onSubmit(e: SubmitEvent) {
    e.preventDefault();
    const formElement = e.currentTarget as HTMLFormElement;
    const formData = new FormData(formElement);
    const { bot_name, bot_type, bot_language, bot_code } = Object.fromEntries(
      formData.entries(),
    );

    if (!bot_name || !bot_type || !bot_language || !bot_code) return;

    const encoder = new TextEncoder();
    const botCodeBytes = encoder.encode(bot_code.toString());
    const botCodeUtf8 = String.fromCodePoint(...botCodeBytes);
    const botCode = btoa(botCodeUtf8);

    const body = {
      name: bot_name,
      kind: bot_type,
      language: bot_language,
      handler: botCode,
    };

    let res = await fetch(`/api/bots`, {
      method: "POST",
      body: JSON.stringify(body),
    });
    if (!res.ok) {
      console.error("failed to submit request");
      return;
    }

    // const { id } = (await res.json()) as { id: string };
    window.location.assign(`/bots`);
  }

  return (
    <Fragment>
      <Navbar />
      <main>
        <div className="content-max-width l">
          <section className="content-panel list-block">
            <div className="panel-header">
              <h2>Create Bot</h2>
            </div>

            <form onSubmit={onSubmit}>
              <label htmlFor="bot_name">
                <p>Bot Name</p>
                <input
                  type="text"
                  name="bot_name"
                  id="bot_name"
                  required
                  placeholder="Enter Name for the Bot"
                />
              </label>

              <label htmlFor="bot_type">
                <p>Bot Type</p>
                <select name="bot_type" id="bot_type">
                  <option value="live">Live - Actively Trade 24/7</option>
                  {/* <option value="candle-close">
                    Candle Close - Run when candle closes
                  </option> */}
                </select>
              </label>

              <label htmlFor="bot_language">
                <p>Bot Language</p>
                <select name="bot_language" id="bot_language">
                  <option value="typescript-v1">TypeScript V1</option>
                </select>
              </label>

              <label htmlFor="bot_code">
                <p>Bot Code</p>
                <textarea
                  defaultValue={defaultCode}
                  name="bot_code"
                  id="bot_code"
                />
              </label>

              <div className="btns">
                <button type="submit" className="btn blue solid">
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
