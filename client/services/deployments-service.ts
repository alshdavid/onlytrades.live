import { BehaviorSubject, Observable } from "rxjs";
import { useEffect } from "preact/hooks";

type ApiDeploymentsResponse = {
  deployments: Array<Deployment>;
};

export type Deployment = {
  id: string;
  bot_id: string;
  name: string;
  account_id: number;
  environment: string;
  active: boolean;
  created_at: string;
  running: boolean;
  started_at: string;
};

export type DeploymentDetail = Deployment;

export class DeploymentsService {
  #deployments: BehaviorSubject<Array<Deployment> | undefined>;
  #deploymentsLoaded: BehaviorSubject<boolean>;

  deployments: Observable<Array<Deployment> | undefined>;
  deploymentsLoaded: Observable<boolean>;

  constructor() {
    this.#deploymentsLoaded = new BehaviorSubject<boolean>(false);
    this.#deployments = new BehaviorSubject<Array<Deployment> | undefined>(
      undefined,
    );

    this.deployments = this.#deployments;
    this.deploymentsLoaded = this.#deploymentsLoaded;
  }

  async init() {
    const response = await fetch("/api/deployments");
    this.#deploymentsLoaded.next(true);
    if (!response.ok) {
      this.#deployments.error(new Error(`${response.status}`));
      return;
    }

    const data: ApiDeploymentsResponse = await response.json();

    for (const entry of data.deployments) {
      entry.environment = atob(entry.environment);
    }

    this.#deployments.next(data.deployments);
  }

  async getDeployment(id: string): Promise<DeploymentDetail> {
    const response = await fetch(`/api/deployments/${id}`);
    if (!response.ok) {
      throw new Error("Unable to get deployment");
    }

    const body = (await response.json()) as DeploymentDetail;
    return body;
  }

  async deleteDeployment(id: string): Promise<void> {
    const response = await fetch(`/api/deployments/${id}`, {
      method: "DELETE",
    });
    if (!response.ok) {
      throw new Error("Unable to delete deployment");
    }
  }

  async pauseDeployment(id: string): Promise<void> {
    const response = await fetch(`/api/deployments/${id}`, {
      method: "PATCH",
      body: JSON.stringify({
        active: false,
      }),
    });
    if (!response.ok) {
      throw new Error("Unable to pause deployment");
    }
  }

  async resumeDeployment(id: string): Promise<void> {
    const response = await fetch(`/api/deployments/${id}`, {
      method: "PATCH",
      body: JSON.stringify({
        active: true,
      }),
    });
    if (!response.ok) {
      throw new Error("Unable to pause deployment");
    }
  }

  useInit = () => {
    useEffect(() => {
      if (this.#deploymentsLoaded.value) {
        return;
      }
      this.init();
    }, [this]);
  };
}
