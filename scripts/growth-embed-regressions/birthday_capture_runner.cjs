// The script is the actual Rust-rendered listener. DOM state is local; all HTTP
// goes to this ephemeral loopback recorder, never a provider or live account.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const http = require('node:http');
const vm = require('node:vm');
const input = JSON.parse(fs.readFileSync(0, 'utf8'));
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
async function until(check) {
  const deadline = Date.now() + 3000;
  while (!check()) {
    if (Date.now() >= deadline) throw new Error('Listener did not reach a visible terminal state');
    await sleep(1);
  }
}
async function exercise(spec) {
  const requests = [];
  const server = http.createServer(async (request, response) => {
    let raw = '';
    for await (const chunk of request) raw += chunk;
    requests.push({method: request.method, path: request.url, body: JSON.parse(raw)});
    if (spec.drop) { request.socket.destroy(); return; }
    response.writeHead(spec.status, {'content-type': 'application/json'});
    response.end(spec.raw ?? JSON.stringify(spec.body));
  });
  await new Promise(resolve => server.listen(0, '127.0.0.1', resolve));
  try {
    const origin = `http://127.0.0.1:${server.address().port}`;
    const nodes = Object.fromEntries(Object.entries(input.ids).map(([id, value]) => [id, {
      textContent: value.text.trim(), value: '', disabled: false,
      style: {display: /display:\s*none/.test(value.style) ? 'none' : ''},
      setAttribute(name, value) { this[name] = value; },
    }]));
    nodes.name.value = 'Synthetic fixture';
    nodes.email.value = 'fixture@example.test';
    nodes.birthday.value = '2000-01-01';
    const group = {style: {display: ''}};
    let click;
    nodes['join-btn'].addEventListener = (event, handler) => { assert.equal(event, 'click'); click = handler; };
    const alerts = [];
    const context = vm.createContext({
      document: {
        getElementById(id) { assert.ok(nodes[id], `Rendered element is missing: ${id}`); return nodes[id]; },
        querySelector(selector) {
          if (selector === '.input-group') return group;
          if (selector === '.widget-container') return {getAttribute(name) {assert.equal(name, 'data-tenant'); return input.tenant;}};
          throw new Error(`Unexpected selector ${selector}`);
        },
      },
      fetch(url, options) {
        assert.equal(url, '/api/v1/growth/birthday-club/capture');
        return fetch(new URL(url, origin), options);
      },
      alert(message) { alerts.push(message); },
      console: {error() {}},
    });
    new vm.Script(input.script).runInContext(context, {timeout: 1000});
    assert.equal(requests.length, 0);
    assert.equal(typeof click, 'function');
    const pending = click.call(nodes['join-btn']);
    // Duplicate listener invocations while pending cannot create another POST.
    if (spec.duplicate) click.call(nodes['join-btn']);
    if (pending?.then) await pending;
    await until(() => group.style.display === 'none' || !nodes['join-btn'].disabled
      || /could not be confirmed|not accepted/i.test(nodes['capture-status']?.textContent ?? ''));
    assert.equal(requests.length, 1, 'A pending submission must remain single-flight');
    assert.deepEqual(requests[0], {method: 'POST', path: '/api/v1/growth/birthday-club/capture', body: {
      tenant_id: input.tenant, name: 'Synthetic fixture', email: 'fixture@example.test', birthday: '2000-01-01',
    }});
    if (spec.accepted) {
      assert.equal(group.style.display, 'none');
      assert.equal(nodes['success-message'].style.display, 'block');
      assert.match(nodes['success-message'].textContent, /request (was )?accepted/i);
      assert.doesNotMatch(nodes['success-message'].textContent, /send you|joined successfully|gift/i);
    } else {
      assert.notEqual(group.style.display, 'none', 'Unconfirmed submission must preserve the form');
      assert.equal(nodes['success-message'].style.display, 'none', 'Rejected/unknown capture must not show success');
      assert.match(nodes['capture-status'].textContent, spec.retryable ? /not accepted/i : /could not be confirmed/i);
      assert.equal(nodes['join-btn'].disabled, !spec.retryable);
      assert.equal(nodes.name.value, 'Synthetic fixture');
      assert.equal(nodes.email.value, 'fixture@example.test');
      assert.equal(nodes.birthday.value, '2000-01-01');
    }
    assert.deepEqual(alerts, [], 'Do not fabricate a membership or delivery confirmation');
    console.log(`birthday HTTP case passed: ${spec.name}`);
  } finally {
    server.closeAllConnections();
    await new Promise(resolve => server.close(resolve));
  }
}
(async () => {
  for (const spec of [
    {name: 'server-error', status: 500, body: {error: 'fixture failure'}},
    {name: 'client-rejection', status: 400, body: {error: 'fixture rejected'}, retryable: true},
    {name: 'negative-acknowledgement', status: 200, body: {success: false}},
    {name: 'contradictory-acknowledgement', status: 200, body: {success: true, error: 'not accepted'}},
    {name: 'missing-acknowledgement', status: 200, body: {}},
    {name: 'malformed-body', status: 200, raw: 'not json'},
    {name: 'accepted-not-completed-status', status: 202, body: {success: true}},
    {name: 'connection-loss', drop: true},
    {name: 'acknowledged-request', status: 200, body: {success: true}, accepted: true},
    {name: 'duplicate-pending-click', status: 200, body: {success: true}, accepted: true, duplicate: true},
  ]) await exercise(spec);
})().catch(error => { console.error(error); process.exitCode = 1; });
