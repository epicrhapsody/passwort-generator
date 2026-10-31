import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { runInNewContext } from 'node:vm';

const source = readFileSync(new URL('../src/main.js', import.meta.url), 'utf8')
  .replace(/^import .*;\r?\n/gm, '');
const tick = () => new Promise(resolve => setImmediate(resolve));
function setup(invoke, writeText = async () => {}, desktop = true) {
  const elements = new Map();
  const get = id => {
    if (!elements.has(id)) elements.set(id, {
      value: id === 'length' ? '20' : '', checked: id !== 'exclude', disabled: false,
      textContent: '', handlers: {}, addEventListener(event, fn) { this.handlers[event] = fn; }
    });
    return elements.get(id);
  };
  runInNewContext(source, { document: { getElementById: get }, invoke, writeText, isTauri: () => desktop });
  return get;
}
test('successful generation and explicit clipboard copy', async () => {
  let copied;
  const get = setup(async () => 'abc-123', async text => { copied = text; });
  await tick();
  assert.equal(get('password').value, 'abc-123');
  assert.equal(copied, undefined);
  await get('copy').handlers.click();
  assert.equal(copied, 'abc-123');
});
test('option changes discard an in-flight password', async () => {
  let complete;
  const get = setup(() => new Promise(resolve => { complete = resolve; }));
  get('length').value = '30';
  get('length').handlers.input();
  complete('stale-password');
  await tick();
  assert.equal(get('password').value, '');
  assert.equal(get('copy').disabled, true);
});
test('RNG/backend failure leaves no password to copy', async () => {
  const get = setup(async () => { throw 'Systemzufall fehlt'; });
  await tick();
  assert.equal(get('password').value, '');
  assert.equal(get('copy').disabled, true);
  assert.match(get('status').textContent, /Systemzufall/);
  assert.equal(get('generate').disabled, false);
});
test('clipboard failure is recoverable', async () => {
  const get = setup(async () => 'abc', async () => { throw Error('clipboard unavailable'); });
  await tick();
  await get('copy').handlers.click();
  assert.match(get('status').textContent, /fehlgeschlagen/);
  assert.equal(get('copy').disabled, false);
});
test('browser preview does not silently substitute a generator', async () => {
  let called = false;
  const get = setup(async () => { called = true; }, undefined, false);
  await tick();
  assert.equal(called, false);
  assert.equal(get('password').value, '');
  assert.match(get('status').textContent, /Desktop-App/);
});
