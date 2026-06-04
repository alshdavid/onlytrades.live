import { type Observable } from "rxjs";
import { useEffect, useState } from "preact/hooks";

export type Result<T, E = Error> =
  // Done
  | [T, undefined]
  // Error
  | [undefined, E]
  // Pending
  | [undefined, undefined];

export function useSubscribe<T, E = Error>(
  x: Observable<T> | Promise<T> | (() => Observable<T>) | (() => Promise<T>),
  deps: Array<any> = [],
): Result<T, E> {
  const [state, setState] = useState<Result<T, E>>([undefined, undefined]);

  useEffect(() => {
    let target = (() => {
      if (typeof x === "function") {
        return x();
      }
      return x;
    })();

    if (target instanceof Promise) {
      target
        .then((v) => setState([v, undefined]))
        .catch((e) => setState([undefined, e]));
      return;
    }
    const subscription = target.subscribe({
      next: (val) => {
        setState([val, undefined]);
      },
      error: (err: E) => {
        setState([undefined, err]);
      },
    });

    return () => subscription.unsubscribe();
  }, [...deps]);

  return state;
}
