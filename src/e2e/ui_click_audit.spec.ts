import { test, expect } from './fixtures';
import { hasMeaningfulClickEffect, hasFragmentTarget, observeClickEffects, replaceAuditDocument, resolveAuditTarget, tagClickTargets } from './support/ui_click_audit';

import { createServer } from 'node:http';
import { runDynamicClickInventory } from '../../scripts/ui-audit-inventory.cjs';
import { createAuditNavigation } from './support/ui_audit_navigation';

// These verify the crawler's observation boundary using real browser behavior,
// not mocked requests or substituted product responses.
test.describe('click audit oracle', () => {
  test('rejects both dead controls and browser-dialog-only fake success', async ({ page }) => {
    await page.setContent('<button>Dead control</button><button onclick="alert(\'Saved!\')">Fake save</button>');
    for (const label of ['Dead control', 'Fake save']) {
      const effect = await observeClickEffects(page, (await page.getByRole('button', { name: label }).elementHandle())!);
      expect(hasMeaningfulClickEffect(effect)).toBe(false);
      expect(effect.dialogSeen).toBe(label === 'Fake save');
    }
  });

  test('rejects a dead button even when trusted hover opens a tooltip', async ({ page }) => {
    await page.setContent(`<button onmouseenter="const t=document.createElement('div');t.setAttribute('role','tooltip');t.textContent='Hover help';document.body.append(t)">Dead control</button>`);
    const effect = await observeClickEffects(page, (await page.getByRole('button', { name: 'Dead control' }).elementHandle())!);
    expect(await page.getByRole('tooltip').count()).toBeGreaterThan(0);
    expect(effect.changed).toBe(false);
    expect(hasMeaningfulClickEffect(effect)).toBe(false);
  });

  test('observes a browser download', async ({ page }) => {
    await page.setContent(`<button onclick="const a=document.createElement('a');a.href='data:text/plain,audit-download';a.download='audit.txt';a.click()">Download</button>`);
    const effect = await observeClickEffects(page, (await page.getByRole('button', { name: 'Download' }).elementHandle())!);
    expect(effect.downloadSeen).toBe(true);
    expect(hasMeaningfulClickEffect(effect)).toBe(true);
  });

  test('observes a native file chooser without uploading anything', async ({ page }) => {
    await page.setContent('<input type="file" id="file" hidden><button onclick="document.getElementById(\'file\').click()">Upload</button>');
    const effect = await observeClickEffects(page, (await page.getByRole('button', { name: 'Upload' }).elementHandle())!);
    expect(effect.fileChooserSeen).toBe(true);
    expect(hasMeaningfulClickEffect(effect)).toBe(true);
    expect(await page.locator('#file').inputValue()).toBe('');
  });

  test('observes an actual popup document', async ({ page }) => {
    await page.setContent(`<button onclick="const p=window.open('about:blank');p.document.body.textContent='Quote request';">Request Quote</button>`);
    const effect = await observeClickEffects(page, (await page.getByRole('button', { name: 'Request Quote' }).elementHandle())!);
    expect(effect.popupSeen).toBe(true);
    expect(hasMeaningfulClickEffect(effect)).toBe(true);
  });

  test('observes browser required-field feedback without inventing submission', async ({ page }) => {
    await page.setContent('<form><input type="email" required><button>Submit</button></form>');
    const effect = await observeClickEffects(page, (await page.getByRole('button', { name: 'Submit' }).elementHandle())!);
    expect(effect.validationSeen).toBe(true);
    expect(effect.requestSeen).toBe(false);
    expect(hasMeaningfulClickEffect(effect)).toBe(true);
  });
});


test('rejects dead pressed toggles and selected-looking CSS without an exemption', async ({ page }) => {
  await page.setContent('<button aria-pressed="true">Dead pressed toggle</button><button class="selected bg-blue-50">Light</button>');
  for(const name of ['Dead pressed toggle','Light']) {
    const effect=await observeClickEffects(page,(await page.getByRole('button',{name}).elementHandle())!);
    expect(hasMeaningfulClickEffect(effect)).toBe(false);
  }
});

test('proves exclusive choice deselection and the original real off-to-on click', async ({ page }) => {
  await page.setContent(`<div><button id="first" aria-pressed="true" onclick="this.setAttribute('aria-pressed','true');document.getElementById('second').setAttribute('aria-pressed','false');">First</button><button id="second" aria-pressed="false" onclick="this.setAttribute('aria-pressed','true');document.getElementById('first').setAttribute('aria-pressed','false');">Second</button></div>`);
  const effect=await observeClickEffects(page,(await page.locator('#first').elementHandle())!);
  expect(effect.selectionRestored).toBe(true);
  expect(hasMeaningfulClickEffect(effect)).toBe(true);
  await expect(page.locator('#first')).toHaveAttribute('aria-pressed','true');
  await expect(page.locator('#second')).toHaveAttribute('aria-pressed','false');
});

test('does not count an alternate choice effect when the original selected button is dead', async ({ page }) => {
  await page.setContent(`<div><button id="first" aria-pressed="true">Dead selected choice</button><button id="second" aria-pressed="false" onclick="this.setAttribute('aria-pressed','true');document.getElementById('first').setAttribute('aria-pressed','false');">Working alternative</button></div>`);
  const effect=await observeClickEffects(page,(await page.locator('#first').elementHandle())!);
  expect(effect.selectionRestored).toBe(false);
  expect(hasMeaningfulClickEffect(effect)).toBe(false);
});

test('independent toggles must still change themselves and cannot borrow a neighboring effect', async ({ page }) => {
  await page.setContent(`<div><button id="first" aria-pressed="true">Dead independent toggle</button><button aria-pressed="false" onclick="this.setAttribute('aria-pressed','true')">Another independent toggle</button></div>`);
  expect(hasMeaningfulClickEffect(await observeClickEffects(page,(await page.locator('#first').elementHandle())!))).toBe(false);
  await page.locator('#first').evaluate((button)=>{button.addEventListener('click',()=>button.setAttribute('aria-pressed','false'));});
  expect(hasMeaningfulClickEffect(await observeClickEffects(page,(await page.locator('#first').elementHandle())!))).toBe(true);
});

test('fails explicitly when preparation replaces the target rather than counting the remount', async ({ page }) => {
  await page.setContent(`<div><button id="first" aria-pressed="true">Original</button><button aria-pressed="false" onclick="document.getElementById('first').outerHTML='<button id=first aria-pressed=false>Replacement</button>'">Replace original</button></div>`);
  await expect(observeClickEffects(page,(await page.locator('#first').elementHandle())!)).rejects.toThrow('detached during alternate-choice preparation');
});

test('does not edit unrelated fields while checking a real click', async ({ page }) => {
  await page.setContent('<input type="email" required><textarea></textarea><input type="number" min="5"><input type="checkbox"><input type="file"><button onclick="this.textContent=\'Clicked\'">Click once</button>');
  const effect = await observeClickEffects(page, (await page.getByRole('button', { name: 'Click once' }).elementHandle())!);
  expect(hasMeaningfulClickEffect(effect)).toBe(true);
  expect(await page.locator('input[type=email]').inputValue()).toBe('');
  expect(await page.locator('textarea').inputValue()).toBe('');
  expect(await page.locator('input[type=number]').inputValue()).toBe('');
  expect(await page.locator('input[type=checkbox]').isChecked()).toBe(false);
  expect(await page.locator('input[type=file]').inputValue()).toBe('');
});

test('observes fulfilled clipboard copying from a real trusted click', async ({ page, context }) => {
  const server=createServer((_request,response)=>{
    response.writeHead(200,{'content-type':'text/html'});
    response.end(`<button onclick="navigator.clipboard.writeText('isolated-audit-link').then(()=>document.querySelector('output').textContent='Copied')">Copy</button><output></output>`);
  });
  await new Promise<void>((resolve,reject)=>{server.once('error',reject);server.listen(0,'127.0.0.1',resolve);});
  try {
    const address=server.address();
    if(!address || typeof address==='string') throw new Error('Missing local fixture address');
    const origin=`http://127.0.0.1:${address.port}`;
    await context.grantPermissions(['clipboard-read','clipboard-write'],{origin});
    await page.goto(origin);
    const effect=await observeClickEffects(page,(await page.getByRole('button',{name:'Copy'}).elementHandle())!);
    expect(hasMeaningfulClickEffect(effect)).toBe(true);
    expect(await page.evaluate(()=>navigator.clipboard.readText())).toBe('isolated-audit-link');
    expect(await page.locator('output').textContent()).toBe('Copied');
  } finally {
    await new Promise<void>((resolve,reject)=>server.close(error=>error?reject(error):resolve()));
  }
});

test('retiring a clicked document prevents its delayed navigation from replacing the next audit', async ({ page }) => {
  let isolated=await page.context().newPage();
  try {
    await isolated.setContent(`<button onclick="setTimeout(()=>location.href='about:blank#late-checkout',150)">Checkout-like navigation</button>`);
    await isolated.getByRole('button').click();
    const old=isolated;
    isolated=await replaceAuditDocument(isolated);
    await isolated.setContent('<h1>Next audit document</h1>');
    await isolated.waitForTimeout(250);
    expect(isolated).toBe(old);
    expect(old.isClosed()).toBe(false);
    expect(isolated.url()).not.toContain('late-checkout');
    await expect(isolated.getByRole('heading',{name:'Next audit document'})).toBeVisible();
  } finally { await isolated.close(); }
});


test('observes an actual confirmation decision while cancellation performs no mutation', async ({ page }) => {
  await page.setContent(`<button onclick="if(confirm('Delete this isolated fixture?'))document.body.dataset.deleted='true'">Ask before delete</button>`);
  const effect=await observeClickEffects(page,(await page.getByRole('button').elementHandle())!);
  expect(effect.decisionSeen).toBe(true);
  expect(hasMeaningfulClickEffect(effect)).toBe(true);
  expect(effect.requestSeen).toBe(false);
  expect(await page.locator('body').getAttribute('data-deleted')).toBeNull();
});

test('observes an actual input prompt without entering data or claiming completion', async ({ page }) => {
  await page.setContent(`<button onclick="const value=prompt('Enter a fixture label');if(value!==null)document.body.dataset.value=value">Ask for input</button>`);
  const effect=await observeClickEffects(page,(await page.getByRole('button').elementHandle())!);
  expect(effect.decisionSeen).toBe(true);
  expect(hasMeaningfulClickEffect(effect)).toBe(true);
  expect(effect.requestSeen).toBe(false);
  expect(await page.locator('body').getAttribute('data-value')).toBeNull();
});


test('committed blank retirement cancels delayed fetch even after an immediate DOM change', async ({ page }) => {
  let lateRequests = 0;
  const server = createServer((request, response) => {
    if (request.url === '/late') {
      lateRequests += 1;
      response.end('observed');
      return;
    }
    response.writeHead(200, { 'content-type': 'text/html' });
    response.end(`<button onclick="document.querySelector('output').textContent='Pending';setTimeout(()=>fetch('/late'),500)">Delayed action</button><output></output>`);
  });
  await new Promise<void>((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
  const isolated = await page.context().newPage();
  try {
    const address = server.address();
    if (!address || typeof address === 'string') throw new Error('Missing local fixture address');
    const origin = `http://127.0.0.1:${address.port}`;
    await isolated.goto(origin);
    await isolated.getByRole('button').click();
    await expect.poll(() => lateRequests).toBe(1);
    await isolated.goto(origin);
    await isolated.getByRole('button').click();
    await expect(isolated.locator('output')).toHaveText('Pending');
    expect(await replaceAuditDocument(isolated)).toBe(isolated);
    await isolated.setContent('<h1>Next isolated document</h1>');
    await isolated.waitForTimeout(650);
    expect(lateRequests).toBe(1);
    await expect(isolated.getByRole('heading', { name: 'Next isolated document' })).toBeVisible();
  } finally {
    await isolated.close();
    await new Promise<void>((resolve, reject) => server.close(error => error ? reject(error) : resolve()));
  }
});


test('retags a remounted control before performing exactly one real click', async ({ page }) => {
  const markup = `<button id="target" onclick="document.querySelector('output').textContent=String(Number(document.querySelector('output').textContent)+1)">Save fixture</button><output>0</output>`;
  await page.setContent(markup);
  let scans = 0;
  const retag = async () => {
    const targets = await page.locator('#target').evaluateAll((elements) => elements.map((element, index) => {
      element.setAttribute('data-ui-audit-click-index', String(index));
      element.setAttribute('data-ui-audit-click-key', 'same-fixture-control');
      return { key: 'same-fixture-control', index, label: 'Save fixture' };
    }));
    if (scans++ === 0) await page.setContent(markup);
    return targets;
  };
  const target = await resolveAuditTarget(page, 'same-fixture-control', retag);
  await expect(page.locator('output')).toHaveText('0');
  const effect = await observeClickEffects(page, target);
  expect(hasMeaningfulClickEffect(effect)).toBe(true);
  await expect(page.locator('output')).toHaveText('1');
});


test('does not count removing audit markers as visible click feedback', async ({ page }) => {
  await page.setContent(`<button data-ui-audit-click-index="1" data-ui-audit-click-key="fixture" onclick="this.removeAttribute('data-ui-audit-click-index');this.removeAttribute('data-ui-audit-click-key')">Marker-only action</button>`);
  const effect = await observeClickEffects(page, (await page.getByRole('button').elementHandle())!);
  expect(hasMeaningfulClickEffect(effect)).toBe(false);
});


test('rejects a same-URL document replacement before any click', async ({ page }) => {
  const markup = `<button data-ui-audit-click-index="0" data-ui-audit-click-key="same-key" onclick="document.body.dataset.clicked='yes'">Same control</button>`;
  await page.goto(`data:text/html,${encodeURIComponent(markup)}`);
  const url = page.url();
  let replaced = false;
  const retag = async () => {
    if (!replaced) { replaced = true; await page.reload(); }
    return [{ key: 'same-key', index: 0, label: 'Same control' }];
  };
  await expect(resolveAuditTarget(page, 'same-key', retag)).rejects.toThrow('document changed');
  expect(page.url()).toBe(url);
  expect(await page.locator('body').getAttribute('data-clicked')).toBeNull();
});


test('reuses a verified audit session and renews only read navigation after real logout or401', async ({ page }) => {
  let logins = 0;
  let session = 0;
  let logoutEffects = 0;
  let unexpectedProbes = 0;
  const server = createServer((request, response) => {
    if (request.url === '/fixture-login' && request.method === 'POST') {
      session = ++logins;
      response.writeHead(200, { 'set-cookie': `audit_nav_session=${session}; Path=/; SameSite=Lax` });
      response.end('authenticated');
    } else if (request.url === '/api/v1/auth/logout' && request.method === 'POST') {
      logoutEffects += 1;
      session = 0;
      response.writeHead(200, { 'set-cookie': 'audit_nav_session=; Path=/; Max-Age=0' });
      response.end('logged out');
    } else if (request.url?.startsWith('/api/v1/agent-feed')) {
      unexpectedProbes += 1;
      response.writeHead(500);
      response.end('A separate API probe is not navigation evidence');
    } else if (request.url === '/denied') {
      response.writeHead(401);
      response.end('unavailable');
    } else if (request.url === '/login') {
      response.writeHead(200, { 'content-type': 'text/html' });
      response.end('<h1>Login required</h1>');
    } else if (!session || !request.headers.cookie?.split(';').some(value => value.trim() === `audit_nav_session=${session}`)) {
      response.writeHead(302, { location: '/login' });
      response.end();
    } else {
      response.writeHead(200, { 'content-type': 'text/html' });
      response.end(`<h1>Private audit fixture</h1><button onclick="fetch('/api/v1/auth/logout',{method:'POST'}).then(()=>document.querySelector('output').textContent='Logged out')">Log out</button><output></output>`);
    }
  });
  await new Promise<void>((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
  try {
    const address = server.address();
    if (!address || typeof address === 'string') throw new Error('Missing local fixture address');
    const origin = `http://127.0.0.1:${address.port}`;
    const navigate = createAuditNavigation(origin, async (target) => {
      const result = await target.request.post(`${origin}/fixture-login`);
      expect(result.ok()).toBe(true);
    });
    await navigate(page, '/private');
    await replaceAuditDocument(page);
    await navigate(page, '/private');
    expect(logins).toBe(1);
    await page.getByRole('button', { name: 'Log out' }).click();
    await expect(page.locator('output')).toHaveText('Logged out');
    await replaceAuditDocument(page);
    await navigate(page, '/private');
    expect(logins).toBe(2);
    expect(logoutEffects).toBe(1);
    session = 0;
    await replaceAuditDocument(page);
    await navigate(page, '/private');
    expect(logins).toBe(3);
    await expect(page.getByRole('heading', { name: 'Private audit fixture' })).toBeVisible();
    await expect(navigate(page, '/denied')).rejects.toThrow('authentication remained unavailable');
    expect(logins).toBe(4);
    expect(logoutEffects).toBe(1);
    expect(unexpectedProbes).toBe(0);
  } finally {
    await replaceAuditDocument(page);
    await new Promise<void>((resolve, reject) => server.close(error => error ? reject(error) : resolve()));
  }
});


test('accepts a real in-document destination while rejecting placeholder and missing fragments', async ({ page }) => {
  const markup = '<a href="#cost-breakdown-section">View Detailed Costs</a><div style="height:1200px"></div><section id="cost-breakdown-section"><h2>Cost Breakdown</h2></section>';
  const server = createServer((_request, response) => {
    response.writeHead(200, { 'content-type': 'text/html' });
    response.end(markup);
  });
  await new Promise<void>((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
  try {
    const address = server.address();
    if (!address || typeof address === 'string') throw new Error('Missing fixture address');
    await page.goto(`http://127.0.0.1:${address.port}/fragment`);
    expect(await page.evaluate(hasFragmentTarget, '#cost-breakdown-section')).toBe(true);
    for (const fragment of ['#', '#missing', '#%E0%A4']) {
      expect(await page.evaluate(hasFragmentTarget, fragment)).toBe(false);
    }
    await page.getByRole('link', { name: 'View Detailed Costs' }).click();
    await expect(page).toHaveURL(/#cost-breakdown-section$/);
    expect(await page.evaluate(() => location.hash)).toBe('#cost-breakdown-section');
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' })).toBeInViewport();
  } finally { await new Promise<void>((resolve, reject) => server.close(error => error ? reject(error) : resolve())); }

});


test('does not trigger delayed autosave DOM or network effects before an inert submit', async ({ page }) => {
  let autosaves = 0;
  const server = createServer((request, response) => {
    if (request.url === '/autosave') { autosaves += 1; response.writeHead(204); response.end(); return; }
    response.writeHead(200, { 'content-type': 'text/html' });
    response.end(`<form onsubmit="event.preventDefault()"><input type="email" oninput="setTimeout(()=>{document.querySelector('output').textContent='Autosaved';fetch('/autosave',{method:'POST'})},100)"><button>Dead submit</button><output></output></form>`);
  });
  await new Promise<void>((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
  try {
    const address = server.address();
    if (!address || typeof address === 'string') throw new Error('Missing fixture address');
    await page.goto(`http://127.0.0.1:${address.port}/`);
    const effect = await observeClickEffects(page, (await page.getByRole('button', { name: 'Dead submit' }).elementHandle())!);
    expect(hasMeaningfulClickEffect(effect)).toBe(false);
    expect(effect.requestSeen).toBe(false);
    expect(await page.locator('input').inputValue()).toBe('');
    await expect(page.locator('output')).toBeEmpty();
    expect(autosaves).toBe(0);
  } finally { await new Promise<void>((resolve, reject) => server.close(error => error ? reject(error) : resolve())); }
});

test('waits for a real busy shell replacement before discovering its control', async ({ page }) => {
  const server = createServer((request, response) => {
    if (request.url !== '/fixture-busy') { response.writeHead(404).end(); return; }
    response.writeHead(200, { 'content-type': 'text/html' });
    response.end(`<div aria-busy="true"><button>Loading control</button></div><script>setTimeout(()=>{document.querySelector('div').outerHTML='<button onclick="this.textContent=String(Number(this.textContent)+1)">0</button>'},400)</script>`);
  });
  await new Promise<void>((resolve, reject) => { server.once('error', reject); server.listen(0, '127.0.0.1', resolve); });
  try {
    const address = server.address();
    if (!address || typeof address === 'string') throw new Error('Missing fixture address');
    const navigate = createAuditNavigation(`http://127.0.0.1:${address.port}`, async () => undefined);
    await navigate(page, '/fixture-busy');
    await expect(page.getByRole('button', { name: 'Loading control' })).toHaveCount(0);
    const effect = await observeClickEffects(page, (await page.getByRole('button', { name: '0', exact: true }).elementHandle())!);
    expect(hasMeaningfulClickEffect(effect)).toBe(true);
    await expect(page.getByRole('button', { name: '1', exact: true })).toBeVisible();
  } finally { await new Promise<void>((resolve, reject) => server.close(error => error ? reject(error) : resolve())); }
});


test('recognizes focus moved to a visible input during the same trusted click', async ({ page }) => {
  await page.setContent(`<button onclick="document.querySelector('input').focus()">Add attachments</button><input aria-label="Attachments">`);
  const effect=await observeClickEffects(page,(await page.getByRole('button',{name:'Add attachments'}).elementHandle())!);
  await expect(page.getByRole('textbox',{name:'Attachments'})).toBeFocused();
  expect(effect.changed).toBe(false);
  expect(effect.focusSeen).toBe(true);
  expect(hasMeaningfulClickEffect(effect)).toBe(true);
});

test('preparation focus does not make an inert button meaningful', async ({ page }) => {
  await page.setContent('<button>Dead control</button><input aria-label="Unrelated">');
  await page.getByRole('textbox').focus();
  const effect=await observeClickEffects(page,(await page.getByRole('button',{name:'Dead control'}).elementHandle())!);
  expect(effect.focusSeen).toBe(false);
  expect(hasMeaningfulClickEffect(effect)).toBe(false);
});

test('delayed preparation focus cannot certify a dead click', async ({ page }) => {
  await page.setContent(`<button onfocus="setTimeout(()=>document.querySelector('input').focus(),100)">Dead control</button><input aria-label="Unrelated">`);
  const effect=await observeClickEffects(page,(await page.getByRole('button',{name:'Dead control'}).elementHandle())!);
  await expect(page.getByRole('textbox')).toBeFocused();
  expect(effect.focusSeen).toBe(false);
  expect(effect.changed).toBe(false);
  expect(hasMeaningfulClickEffect(effect)).toBe(false);
});

test('focus moved into a hidden field or the document body is not a meaningful control effect', async ({ page }) => {
  for (const selector of ['input','body']) {
    await page.setContent(`<body tabindex="-1"><button onclick="document.querySelector('${selector}').focus()">Dead control</button><input hidden></body>`);
    const effect=await observeClickEffects(page,(await page.getByRole('button',{name:'Dead control'}).elementHandle())!);
    expect(effect.focusSeen).toBe(false);
    expect(hasMeaningfulClickEffect(effect)).toBe(false);
  }
});


test('a vanished discovered button fails the real-browser crawl before claiming exhaustion', async ({ page }) => {
  const baseline = '<button id="dismiss" onclick="document.querySelector(\'#approve\').remove()">Dismiss</button><button id="approve">Approve</button>';
  await page.setContent(baseline);
  const discovered: string[] = [], observed = new Set<string>();
  await expect(runDynamicClickInventory(discovered, observed, {
    discover: () => tagClickTargets(page),
    visit: async (candidate: { key: string }) => {
      const target = await resolveAuditTarget(page, candidate.key, () => tagClickTargets(page));
      expect(hasMeaningfulClickEffect(await observeClickEffects(page, target))).toBe(true);
      observed.add(candidate.key);
    },
    // Reproduce a backend-persisted dismissal surviving a document reset.
    reset: async () => { await replaceAuditDocument(page); await page.setContent('<button id="dismiss">Dismiss</button>'); },
  })).rejects.toThrow(/missing=.*Approve/);
  expect(discovered).toHaveLength(2);
  expect([...observed]).toHaveLength(1);
});
