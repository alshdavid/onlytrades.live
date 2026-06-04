import { test } from "node:test";
import * as assert from "node:assert";
import { normalizePathname } from "./normalize-pathname.ts";

test("It should provide base route", () => {
  const [baseHref, path, fullPath] = normalizePathname("/", "/");
  assert.equal(baseHref, "/");
  assert.equal(path, "/");
  assert.equal(fullPath, "/");
});

test("It should provide base route with base href", () => {
  const [baseHref, path, fullPath] = normalizePathname("/dist", "/");
  assert.equal(baseHref, "/dist");
  assert.equal(path, "/");
  assert.equal(fullPath, "/dist");
});

test("It should route to path", () => {
  const [baseHref, path, fullPath] = normalizePathname("/", "/foo");
  assert.equal(baseHref, "/");
  assert.equal(path, "/foo");
  assert.equal(fullPath, "/foo");
});

test("it should append path to href", () => {
  const [baseHref, path, fullPath] = normalizePathname("/dist", "/foo");
  assert.equal(baseHref, "/dist");
  assert.equal(path, "/foo");
  assert.equal(fullPath, "/dist/foo");
});

test("it should ignore trailing slashes on base href", () => {
  const [baseHref, path, fullPath] = normalizePathname("/dist/", "/foo");
  assert.equal(baseHref, "/dist");
  assert.equal(path, "/foo");
  assert.equal(fullPath, "/dist/foo");
});

test('it should consider an empty basehref as "/"', () => {
  const [baseHref, path, fullPath] = normalizePathname("", "/");
  assert.equal(baseHref, "/");
  assert.equal(path, "/");
  assert.equal(fullPath, "/");
});

test("It should route to path with empty base href", () => {
  const [baseHref, path, fullPath] = normalizePathname("", "/foo");
  assert.equal(baseHref, "/");
  assert.equal(path, "/foo");
  assert.equal(fullPath, "/foo");
});

test("It should lower case path", () => {
  const [baseHref, path, fullPath] = normalizePathname("/", "/Foo");
  assert.equal(baseHref, "/");
  assert.equal(path, "/foo");
  assert.equal(fullPath, "/foo");
});

test("It should lower case path and basehref", () => {
  const [baseHref, path, fullPath] = normalizePathname("/Dist", "/Foo");
  assert.equal(baseHref, "/dist");
  assert.equal(path, "/foo");
  assert.equal(fullPath, "/dist/foo");
});
