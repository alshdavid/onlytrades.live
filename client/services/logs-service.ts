import { Observable } from "rxjs";

type ApiLogsIdResponse = {
  logs: Array<Log>;
};

export type Log = {
  log_level: number;
  message: string;
  created_at: string;
};

export class LogService {
  async getLogs(audience: string): Promise<Array<Log>> {
    const response = await fetch(`/api/logs/${audience}`);
    if (!response.ok) {
      throw new Error("Unable to get logs");
    }
    const body: ApiLogsIdResponse = await response.json();
    return body.logs;
  }

  getLogStream(audience: string): Observable<Log> {
    return new Observable<Log>((o) => {
      const eventSource = new EventSource(`/api/logs/stream/${audience}`);
      eventSource.onmessage = (event) => {
        try {
          const log: Log = JSON.parse(event.data);
          o.next(log);
        } catch (err) {
          o.error(new Error("Failed to parse log data", { cause: err }));
        }
      };

      eventSource.onerror = (err) => {
        o.error(err);
      };

      return () => {
        eventSource.close();
      };
    });
  }
}
