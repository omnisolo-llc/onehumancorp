import assert from 'node:assert/strict';
import net from 'node:net';
import { readdir, readFile } from 'node:fs/promises';
import { test } from 'node:test';
import { createRequire } from 'node:module';
import { getGlobalDispatcher } from 'undici';
import { JSDOM, VirtualConsole } from './test-support/offline-dom.mjs';

test('DOM fixtures block transports before inline scripts and retain explicit response fixtures', async t => {
  const connections = [];
  t.mock.method(net.Socket.prototype, 'connect', function (...args) {
    connections.push(args);
    throw new Error('Unexpected socket connection from a DOM-only fixture');
  });
  const dom = new JSDOM(`<script>
    window.socket = new WebSocket('wss://app.example.test/feed');
    window.xhrBlocked = false;
    try { new XMLHttpRequest(); } catch { window.xhrBlocked = true; }
    window.eventsBlocked = false;
    try { new EventSource('https://app.example.test/events'); } catch { window.eventsBlocked = true; }
    window.beaconAccepted = navigator.sendBeacon('https://app.example.test/beacon', 'fixture-only');
    window.reply = fetch('/recorded').then(reply => reply.json());
  </script>`, {
    url: 'https://app.example.test/dashboard.html', runScripts: 'dangerously',
    beforeParse(window) { window.fetch = async () => Response.json({ recorded: true }); },
  });
  try {
    assert.equal(dom.window.socket.readyState, dom.window.WebSocket.CLOSED);
    assert.throws(() => dom.window.socket.send('fixture-only'), /offline DOM fixture/);
    assert.equal(dom.window.xhrBlocked, true);
    assert.equal(dom.window.eventsBlocked, true);
    assert.equal(dom.window.beaconAccepted, false);
    assert.deepEqual(await dom.window.reply, { recorded: true });
    assert.deepEqual(connections, []);
  } finally { dom.window.close(); }
});

test('external resources and child frames never receive live transport implementations', async t => {
  let connections = 0;
  t.mock.method(net.Socket.prototype, 'connect', function () { connections++; throw new Error('Unexpected network'); });
  const dom = new JSDOM(`<script src="https://app.example.test/script.js"></script>
    <link rel="stylesheet" href="https://app.example.test/theme.css">
    <iframe src="https://app.example.test/frame"></iframe>`, {
    url: 'https://app.example.test/', runScripts: 'dangerously', resources: 'usable',
    virtualConsole: new VirtualConsole(),
  });
  try {
    await new Promise(resolve => setImmediate(resolve));
    await assert.rejects(dom.window.fetch('https://app.example.test/api'), /offline DOM fixture/);
    const frame = dom.window.document.querySelector('iframe').contentWindow;
    const socket = new frame.WebSocket('wss://app.example.test/frame-socket');
    assert.equal(socket.readyState, frame.WebSocket.CLOSED);
    assert.throws(() => new frame.XMLHttpRequest(), /offline DOM fixture/);
    assert.equal(connections, 0);
  } finally { dom.window.close(); }
});

test('network-loading constructors cannot be used through the offline fixture helper', async () => {
  await assert.rejects(JSDOM.fromURL('https://app.example.test/'), /offline DOM fixture/);
});

test('child-realm transports are denied without changing the process dispatcher', async t => {
  let connections = 0;
  t.mock.method(net.Socket.prototype, 'connect', function () { connections++; throw new Error('Unexpected network'); });
  const previous = getGlobalDispatcher();
  const dom = new JSDOM('<iframe></iframe>', { url: 'https://app.example.test/', runScripts: 'dangerously' });
  try {
    assert.equal(getGlobalDispatcher(), previous);
    // Access through frames must not expose a fresh unguarded transport.
    const child = dom.window.frames[0];
    const socket = new child.WebSocket('wss://app.example.test/native-frame');
    await new Promise(resolve => setImmediate(resolve));
    assert.equal(socket.readyState, child.WebSocket.CLOSED);
    assert.throws(() => socket.send('fixture-only'), /offline DOM fixture/);
    assert.equal(connections, 0);
    assert.equal(getGlobalDispatcher(), previous);
  } finally { dom.window.close(); }
  assert.throws(() => new JSDOM('', { beforeParse() { throw new Error('fixture setup failed'); } }), /fixture setup failed/);
  assert.equal(getGlobalDispatcher(), previous);
});

test('all root DOM fixtures import the offline constructor', async () => {
  for (const file of await readdir(new URL('.', import.meta.url))) {
    if (!file.endsWith('.test.mjs')) continue;
    const source = await readFile(new URL(file, import.meta.url), 'utf8');
    assert.doesNotMatch(source, /from\s+['"]jsdom['"]|require\(['"]jsdom['"]\)/, `${file} must use the offline fixture helper`);
  }
});

test('offline dispatcher is a direct locked dependency shared with jsdom', async () => {
  const manifest = JSON.parse(await readFile(new URL('../package.json', import.meta.url), 'utf8'));
  const lock = JSON.parse(await readFile(new URL('../package-lock.json', import.meta.url), 'utf8'));
  assert.equal(manifest.devDependencies.undici, '7.29.1');
  assert.equal(lock.packages[''].devDependencies.undici, manifest.devDependencies.undici);
  assert.equal(lock.packages['node_modules/undici'].version, manifest.devDependencies.undici);
  const require = createRequire(import.meta.url);
  const jsdomRequire = createRequire(require.resolve('jsdom'));
  assert.equal(jsdomRequire.resolve('undici'), require.resolve('undici'));
  assert.equal(jsdomRequire('undici').getGlobalDispatcher, getGlobalDispatcher);
});
