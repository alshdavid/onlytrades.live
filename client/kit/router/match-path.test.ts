import { test } from "node:test";
import * as assert from "node:assert";
import { matchPath } from "./match-path.ts";

test("", () => {
  const result = matchPath("/", "/", "/");
  assert.deepEqual(result, {});
});

test("", () => {
  const result = matchPath("/", "/", "/foo");
  assert.equal(result, undefined);
});

test("", () => {
  const result = matchPath("/", "/foo", "/foo");
  assert.deepEqual(result, {});
});

test("", () => {
  const result = matchPath("/", "/**", "/foo");
  assert.deepEqual(result, {});
});

test("", () => {
  const result = matchPath("/", "/:id", "/foo");
  assert.deepEqual(result, {
    id: "foo",
  });
});

test("", () => {
  const result = matchPath("/", "/foo/:id", "/foo/bar");
  assert.deepEqual(result, {
    id: "bar",
  });
});

// Base href
test("", () => {
  const result = matchPath("/dist", "/", "/dist");
  assert.deepEqual(result, {});
});

test("", () => {
  const result = matchPath("/dist", "/", "/dist/foo");
  assert.equal(result, undefined);
});

test("", () => {
  const result = matchPath("/dist", "/foo", "/dist/foo");
  assert.deepEqual(result, {});
});

test("", () => {
  const result = matchPath("/dist", "/**", "/dist/foo");
  assert.deepEqual(result, {});
});

test("", () => {
  const result = matchPath("/dist", "/:id", "/dist/foo");
  assert.deepEqual(result, {
    id: "foo",
  });
});

test("", () => {
  const result = matchPath("/dist", "/foo/:id", "/dist/foo/bar");
  assert.deepEqual(result, {
    id: "bar",
  });
});
