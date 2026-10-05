import test from 'node:test';
import assert from 'node:assert/strict';
import { EventEmitter } from 'node:events';
import { createServer } from 'node:net';
import { reserveServicePorts } from './native-port-reservations.mjs';

const roles = ['api', 'grpc', 'web'];
async function bind(port = 0, host = '127.0.0.1') {
  const server = createServer(connection => connection.destroy());
  await new Promise((resolve, reject) => {
    server.once('error', reject); server.listen(port, host, resolve);
  });
  return server;
}
const close = server => new Promise((resolve, reject) => server.close(error => error ? reject(error) : resolve()));

function recyclingKernel({ failAt, failCloseAt, onListen } = {}) {
  const active = new Set(), servers = [];
  const allocationError = new Error('Injected bind failure');
  return { active, servers, allocationError, createServer: () => {
    const index = servers.length;
    const socket = new EventEmitter();
    socket.port = undefined; socket.closeCalls = 0;
    socket.listen = (_options, ready) => {
      if (index === failAt) { queueMicrotask(() => socket.emit('error', allocationError)); return; }
      socket.port = 34539;
      while (active.has(socket.port)) socket.port++;
      active.add(socket.port);
      onListen?.(index);
      queueMicrotask(ready);
    };
    socket.address = () => ({ port: socket.port });
    socket.close = done => {
      socket.closeCalls++; active.delete(socket.port);
      queueMicrotask(() => done(index === failCloseAt ? new Error('Injected close failure') : undefined));
    };
    servers.push(socket); return socket;
  } };
}

test('simultaneous reservations remain distinct when the kernel immediately recycles released ports', async () => {
  const kernel = recyclingKernel();
  const reservation = await reserveServicePorts(roles, kernel);
  assert.deepEqual(Object.values(reservation.ports), [34539, 34540, 34541]);
  assert.equal(kernel.active.size, 3);
  await reservation.close();
  assert.equal(kernel.active.size, 0);
});

test('real bound reservations protect provider startup and hand backend/web roles over separately', async () => {
  const reservation = await reserveServicePorts(roles);
  const children = [];
  try {
    assert.equal(new Set(Object.values(reservation.ports)).size, 3);
    for (const port of Object.values(reservation.ports)) await assert.rejects(bind(port), { code: 'EADDRINUSE' });
    const provider = await bind(); children.push(provider);
    assert.ok(!Object.values(reservation.ports).includes(provider.address().port));
    await reservation.release('api', 'grpc');
    // These are the backend's actual wildcard bind semantics.
    children.push(await bind(reservation.ports.api, '0.0.0.0'));
    children.push(await bind(reservation.ports.grpc, '0.0.0.0'));
    await assert.rejects(bind(reservation.ports.web), { code: 'EADDRINUSE' });
    await reservation.release('web');
    children.push(await bind(reservation.ports.web));
    // Closing old reservations must not close the new service's sockets.
    await reservation.close();
    for (const child of children) assert.equal(child.listening, true);
  } finally {
    await reservation.close(); await Promise.all(children.map(close));
  }
});

test('partial allocation failure closes every owned reservation and preserves the bind error', async () => {
  const kernel = recyclingKernel({ failAt: 2 });
  await assert.rejects(reserveServicePorts(roles, kernel), error => error === kernel.allocationError);
  assert.equal(kernel.active.size, 0);
  assert.deepEqual(kernel.servers.map(socket => socket.closeCalls), [1, 1, 1]);
});

test('cleanup attempts every reservation even when one close fails', async () => {
  const kernel = recyclingKernel({ failAt: 2, failCloseAt: 0 });
  await assert.rejects(reserveServicePorts(roles, kernel), error => error instanceof AggregateError
    && error.cause === kernel.allocationError && error.errors[0] === kernel.allocationError);
  assert.equal(kernel.active.size, 0);
  assert.deepEqual(kernel.servers.map(socket => socket.closeCalls), [1, 1, 1]);
});

test('concurrent and repeated release closes each owned socket once', async () => {
  const kernel = recyclingKernel();
  const reservation = await reserveServicePorts(roles, kernel);
  await Promise.all([reservation.release('api'), reservation.release('api', 'grpc'), reservation.close()]);
  await reservation.close();
  assert.deepEqual(kernel.servers.map(socket => socket.closeCalls), [1, 1, 1]);
});

test('cancellation during allocation releases partial reservations and preserves cancellation', async () => {
  const controller = new AbortController(), reason = new Error('Cancelled allocation');
  const kernel = recyclingKernel({ onListen: index => { if (index === 1) controller.abort(reason); } });
  await assert.rejects(reserveServicePorts(roles, { ...kernel, signal: controller.signal }), error => error === reason);
  assert.equal(kernel.active.size, 0);
  assert.deepEqual(kernel.servers.map(socket => socket.closeCalls), [1, 1]);
});

test('cancellation releases real held sockets before any child starts', async () => {
  const controller = new AbortController();
  const reservation = await reserveServicePorts(roles, { signal: controller.signal });
  controller.abort(); await reservation.close();
  const rebound = [];
  try { for (const port of Object.values(reservation.ports)) rebound.push(await bind(port)); }
  finally { await Promise.all(rebound.map(close)); }
});

test('cancellation while real listen is still pending cannot create a late listener', async () => {
  const controller = new AbortController(), reason = new Error('Cancelled before listening');
  const servers = [], listening = [];
  const allocation = reserveServicePorts(roles, { signal: controller.signal, createServer: handler => {
    const server = createServer(handler); servers.push(server);
    server.on('listening', () => listening.push(server.address()));
    return server;
  } });
  assert.equal(servers.length, 1);
  assert.equal(servers[0].listening, false, 'loopback lookup/listen has not settled yet');
  controller.abort(reason);
  await assert.rejects(allocation, error => error === reason);
  // Drain the pending lookup/listen continuation after cancellation cleanup.
  await new Promise(setImmediate);
  assert.deepEqual(listening, []);
  assert.equal(servers[0].listening, false);
  assert.equal(servers[0].listenerCount('error'), 0);
});

test('synchronous listen failure removes allocation listeners and closes earlier sockets', async () => {
  const kernel = recyclingKernel(), controller = new AbortController();
  const failed = new Error('Synchronous listen failure');
  const factory = () => {
    const socket = kernel.createServer();
    if (kernel.servers.length === 2) socket.listen = () => { throw failed; };
    return socket;
  };
  await assert.rejects(reserveServicePorts(roles, { createServer: factory, signal: controller.signal }), error => error === failed);
  assert.equal(kernel.active.size, 0);
  assert.deepEqual(kernel.servers.map(socket => socket.closeCalls), [1, 1]);
  assert.deepEqual(kernel.servers.map(socket => socket.listenerCount('error')), [0, 0]);
  controller.abort();
  assert.deepEqual(kernel.servers.map(socket => socket.closeCalls), [1, 1]);
});

test('invalid roles fail before allocation and unknown release roles preserve held sockets', async () => {
  const kernel = recyclingKernel();
  for (const names of [[], ['api', 'api'], [''], [null]]) await assert.rejects(reserveServicePorts(names, kernel));
  assert.equal(kernel.servers.length, 0);
  const reservation = await reserveServicePorts(roles, kernel);
  await assert.rejects(reservation.release('api', 'unknown'), /Unknown service port role/);
  assert.equal(kernel.active.size, 3);
  await reservation.close();
});
