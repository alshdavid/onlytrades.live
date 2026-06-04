import { Fragment, h } from "preact";
import { Navbar } from "../../components/navbar/navbar.tsx";

export function AdminPage() {
  return (
    <Fragment>
      <Navbar />
      <main>
        <div className="content-max-width l">
          <h1>admin - Coming Soon</h1>
          <br />
          <p>
            admin are algorithms running on the OnlyTrades server that execute
            trades
          </p>
          <br />
          <p>You can write your own Bot or get a Bot from the Marketplace</p>
          <br />
          <p>
            admin have a direct link to the broker API to maximize performance
            and reduce latency
          </p>
        </div>
      </main>
    </Fragment>
  );
}
