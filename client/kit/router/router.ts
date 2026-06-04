import { matchPath } from "./match-path.ts";
import { normalizePathname } from "./normalize-pathname.ts";

export const AUTO_BASE_HREF = Symbol("auto");

export type HandlerFunc = (req: Req) => any | Promise<any>;
export type Req = {
  routePattern: string;
  path: string;
  url: URL;
  params: Record<string, string>;
};

export type RouterOptions = {
  baseHref?: string | typeof AUTO_BASE_HREF;
};

export class Router extends EventTarget {
  #routes: Map<string, HandlerFunc>;
  #baseHref: string;
  #req: Req | undefined;

  constructor({ baseHref }: RouterOptions = {}) {
    super();
    this.#routes = new Map();
    if (baseHref === AUTO_BASE_HREF) {
      this.#baseHref =
        globalThis.document.querySelector("base[href]")?.getAttribute("href") ||
        "/";
    } else {
      this.#baseHref = baseHref || "/";
    }
  }

  get req(): Req {
    if (!this.#req) {
      throw new Error("Request hasn't started yet");
    }
    return this.#req;
  }

  routes(): Readonly<Record<string, HandlerFunc>> {
    const obj: Record<string, HandlerFunc> = {};
    for (const [k, v] of this.#routes.entries()) {
      obj[k] = v;
    }
    return obj;
  }

  start() {
    this.#digest();
  }

  // route(regex: RegExp, handler: HandlerFunc): Router
  // route(paths: string[], handler: HandlerFunc): Router
  // route(path: string, handler: HandlerFunc<T>): Router
  route(path: string | Array<string>, handler: HandlerFunc): Router {
    if (typeof path === "string") {
      path = [path];
    }
    for (const p of path) {
      const [, normalizedPath] = normalizePathname(this.#baseHref, p);
      this.#routes.set(normalizedPath, handler);
    }
    return this;
  }

  #digest() {
    const [, normalizedPath] = normalizePathname(
      this.#baseHref,
      globalThis.location.pathname,
    );
    const [handler, params, pattern] = this.#matchRoute(normalizedPath);
    if (!handler || !pattern) {
      return;
    }
    const req: Req = {
      routePattern: pattern,
      path: normalizedPath,
      url: new URL(globalThis.location.href),
      params: params || {},
    };
    this.#req = req;
    handler(req);
    this.dispatchEvent(new Event("change"));
  }

  #matchRoute(
    pathname: string,
  ): [HandlerFunc?, Record<string, string>?, string?] {
    const route = this.#routes.get(pathname);

    if (route) {
      return [route, {}, pathname];
    }
    for (const [pattern, handler] of this.#routes.entries()) {
      const result = matchPath(this.#baseHref, pattern, pathname);
      if (result) {
        return [handler, result, pattern];
      }
    }
    return [];
  }
}
