import { test, expect } from './fixtures';
import { fillEmptyAuditControls, hasMeaningfulClickEffect, observeClickEffects, replaceAuditDocument } from './support/ui_click_audit';

import { createServer } from 'node:http';

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

test('fills native browser controls without invoking an instance value tracker', async ({ page }) => {
  await page.setContent('<input type="email" required><textarea></textarea><input type="number" min="5"><input type="checkbox"><input type="file"><output></output>');
  await page.evaluate(() => {
    const control=document.querySelector('input')!;
    const descriptor=Object.getOwnPropertyDescriptor(HTMLInputElement.prototype,'value')!;
    Object.defineProperty(control,'value',{configurable:true,get(){return descriptor.get!.call(this);},set(value){this.dataset.instanceAssigned='true';descriptor.set!.call(this,value);}});
    control.addEventListener('input',()=>{document.querySelector('output')!.textContent=control.value;});
  });
  await page.evaluate(fillEmptyAuditControls);
  expect(await page.locator('input[type=email]').inputValue()).toBe('ui-audit@example.test');
  expect(await page.locator('input[type=email]').getAttribute('data-instance-assigned')).toBeNull();
  expect(await page.locator('input[type=email]').evaluate((input: HTMLInputElement)=>input.validity.valid)).toBe(true);
  expect(await page.locator('output').textContent()).toBe('ui-audit@example.test');
  expect(await page.locator('textarea').inputValue()).toBe('Audit value');
  expect(await page.locator('input[type=number]').inputValue()).toBe('5');
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
    expect(old.isClosed()).toBe(true);
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
