import {createOriginLockManager} from './test-support/origin-locks.mjs';
import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { JSDOM } from 'jsdom';

const paths = [
  'src/ui/tauri/src/ui/setup.html',
  ...['setup.html', 'ui/setup.html', 'api/ui/setup.html', 'api/v1/ui/setup.html'].map(p => `src/ui/next/public/${p}`),
];

const testOwner={userId:'user-1',tenantId:'org-1'};
const draftKey='omnisolo_onboarding_owned_v1:'+encodeURIComponent(JSON.stringify([testOwner.userId,testOwner.tenantId]))+':legacy-draft';
const ownerHeaders=['Content-Type','x-ohc-expected-user','x-ohc-expected-tenant'];
async function waitUntil(predicate) { const deadline=Date.now()+1000; while(!predicate()) { if(Date.now()>deadline)throw new Error('Expected request was not dispatched'); await new Promise(resolve=>setTimeout(resolve,0)); } }
async function setup(path, fetch, options = {}) {
  const html = await readFile(new URL(`../${path}`, import.meta.url), 'utf8');
  const dom = new JSDOM(html, { url: options.url || 'https://workspace.example/setup.html', runScripts: 'outside-only', pretendToBeVisual: true });
  Object.defineProperty(dom.window.navigator,'locks',{configurable:true,value:options.locks===false?undefined:options.locks||createOriginLockManager()});
  dom.window.fetch = (url, request) => url.endsWith('/session-identity') ? Promise.resolve(Response.json({...testOwner,expiresAt:Date.now()+60_000})) : fetch(url,request);
  const initial = options.stored ?? { work_context: 'Agency', business_name: 'Nora Studio', first_offer: 'Logo Design', location: 'Portland, OR', target_audience: 'Local founders', categories: 'Design', assistant_name: 'Nora' };
  dom.window.localStorage.setItem(draftKey, JSON.stringify({format:1,state:initial,revision:'fixture-acknowledged',acknowledgedRevision:'fixture-acknowledged'}));
  options.prepare?.(dom.window);
  dom.window.HTMLElement.prototype.scrollTo = () => {};
  dom.window.HTMLElement.prototype.scrollIntoView = () => {};
  // jsdom does not implement innerText, which the legacy buttons use.
  Object.defineProperty(dom.window.HTMLElement.prototype, 'innerText', { get() { return this.textContent; }, set(value) { this.textContent = value; } });
  const main = [...dom.window.document.scripts].find(script => script.textContent.includes('const container ='));
  dom.window.eval(main.textContent);
  await new Promise(resolve => setTimeout(resolve, 0));
  return dom;
}

for (const path of paths) {
  test(`${path} does not invent a completed business draft when chat fails`, async () => {
    let submitted = 0;
    const dom = await setup(path, async url => {
      if (url.endsWith('/start')) submitted += 1;
      if (url.endsWith('/chat')) return { ok: false, status: 503, json: async () => ({ error: 'Unavailable' }) };
      return { ok: true, json: async () => ({}) };
    });
    try {
      dom.window.goToStep('step-chat');
      dom.window.document.getElementById('chat-input').value = 'A pottery studio';
      dom.window.document.getElementById('chat-send-btn').click();
      await new Promise(resolve => setTimeout(resolve, 20));
      assert.equal(submitted, 0);
      assert.equal(dom.window.pendingOnboardingReq, undefined);
      assert.ok(dom.window.document.getElementById('step-chat').classList.contains('active'));
      assert.match(dom.window.document.getElementById('chat-messages').textContent, /Failed to connect/);
      assert.equal(dom.window.document.getElementById('chat-send-btn').disabled, false);
      assert.equal(dom.window.localStorage.getItem('has_onboarded'), null);
    } finally { dom.window.close(); }
  });
  test(`${path} does not send browser-derived authority with setup chat`, async () => {
    let options;
    const dom = await setup(path, async (url, request) => {
      if (url.endsWith('/chat')) { options = request; return { ok: true, json: async () => ({ reply: 'Tell me more', is_complete: false }) }; }
      return { ok: true, json: async () => ({}) };
    });
    try {
      dom.window.goToStep('step-chat');
      dom.window.document.getElementById('chat-input').value = 'A pottery studio';
      dom.window.document.getElementById('chat-send-btn').click();
      await new Promise(resolve => setTimeout(resolve, 20));
      assert.deepEqual(Object.keys(options.headers), ownerHeaders);
    } finally { dom.window.close(); }
  });
}

for (const path of paths) {
  test(`${path} sends business setup without replacement credentials`, async () => {
    let submitted;
    const dom = await setup(path, async (url, options) => {
      if (url.endsWith('/start')) {
        submitted = { url, body: JSON.parse(options.body) };
        return new Promise(() => {});
      }
      return { ok: true, json: async () => ({}) };
    });
    try {
      dom.window.document.getElementById('template-selection').value = 'Modern';
      dom.window.document.getElementById('finish-btn').click();
      await waitUntil(()=>submitted);
      assert.ok(submitted, 'the actual finish handler must submit setup');
      for (const field of ['admin_email', 'admin_name', 'admin_password']) assert.equal(Object.hasOwn(submitted.body, field), false, field);
      assert.equal(submitted.url, '/api/v1/onboarding/start');
      assert.equal(submitted.body.company_description, 'Logo Design');
      assert.equal(submitted.body.location, 'Portland, OR');
      assert.equal(submitted.body.target_audience, 'Local founders');
    } finally { dom.window.close(); }
  });
}

for (const path of paths) {
  test(`${path} shows one review step and waits for explicit launch approval`, async () => {
    let starts = 0;
    const dom = await setup(path, async (url) => {
      if (url.endsWith('/start')) { starts++; return { ok: false, json: async () => ({ error: 'Launch rejected' }) }; }
      if (url.endsWith('/chat')) return { ok: true, json: async () => ({ is_complete: true, reply: 'Ready for review', intake_data: { business_name: '<img src=x onerror=alert(1)>', business_type: 'Service', initial_products: [{ name: 'Owner service', price: '25.00' }] } }) };
      return { ok: true, json: async () => ({}) };
    });
    try {
      dom.window.goToStep('step-chat');
      dom.window.document.getElementById('chat-input').value = 'I provide owner services';
      dom.window.document.getElementById('chat-send-btn').click();
      await new Promise(resolve => setTimeout(resolve, 150));
      const review = dom.window.document.getElementById('step-approval');
      assert.ok(review.classList.contains('active'));
      assert.equal(starts, 0, 'preparation must wait for the owner to launch');
      assert.equal(dom.window.document.querySelectorAll('#approval-details').length, 1);
      assert.match(review.textContent, /Owner service/);
      assert.equal(review.querySelector('img'), null, 'business content is inert text');
      dom.window.document.getElementById('approve-publish-btn').click();
      await new Promise(resolve => setTimeout(resolve, 0));
      assert.equal(starts, 1);
      assert.equal(dom.window.localStorage.getItem('has_onboarded'), null);
      assert.equal(dom.window.document.getElementById('approval-error').textContent, 'Launch rejected');
      assert.equal(dom.window.document.getElementById('approve-publish-btn').disabled, false);
      assert.equal(dom.window.document.getElementById('approve-publish-btn').textContent, 'Approve & Complete Setup');
    } finally { dom.window.close(); }
  });
}

for (const path of paths) {
  test(`${path} projects nullable intake fields into the actual strict preparation request`, async () => {
    let submitted;
    const dom = await setup(path, async (url, options) => {
      if (url.endsWith('/start')) { submitted = JSON.parse(options.body); return new Promise(() => {}); }
      if (url.endsWith('/chat')) return Response.json({ is_complete: true, reply: 'Ready to review', intake_data: {
        business_name: 'Owner studio', business_type: 'Service', initial_products: [
          { name: 'First service', price: '25.00', description: null, variants: null },
          { name: 'Second service', price: 15, description: 'Reviewed details', variants: [{ name: 'Extended', price_modifier: 5, model_note: 'not an API field' }], model_note: 'not an API field' },
        ],
      } });
      return Response.json({});
    });
    try {
      dom.window.goToStep('step-chat');
      dom.window.document.getElementById('chat-input').value = 'I provide owner services';
      dom.window.document.getElementById('chat-send-btn').click();
      await waitUntil(() => dom.window.pendingOnboardingReq);
      dom.window.document.getElementById('approve-publish-btn').click();
      await waitUntil(() => submitted);
      assert.deepEqual(submitted.initial_products, [
        { name: 'First service', price: '25.00', description: '', variants: [] },
        { name: 'Second service', price: '15', description: 'Reviewed details', variants: [{ name: 'Extended', price_modifier: '5' }] },
      ]);
    } finally { dom.window.close(); }
  });
}

const legacyCredentialFields = ['admin_name', 'admin_email', 'admin_password'];
const legacyCredentials = Object.fromEntries(legacyCredentialFields.map(field => [field, `obsolete-${field}`]));

function assertBusinessOnly(draft) {
  for (const field of legacyCredentialFields) {
    assert.equal(Object.hasOwn(draft, field), false, `${field} must not enter a business draft`);
  }
}

for (const path of paths) {
  test(`${path} clears saved credentials before waiting for draft retrieval`, async () => {
    const dom = await setup(path, () => new Promise(() => {}), {
      stored: { business_name: 'Preserved Studio', step: 5, ...legacyCredentials },
      prepare(window) { window.localStorage.setItem('unrelated-setting', 'preserved'); },
    });
    try {
      const draft = JSON.parse(dom.window.localStorage.getItem(draftKey)).state;
      assertBusinessOnly(draft);
      assert.equal(draft.business_name, 'Preserved Studio');
      assert.equal(draft.step, 5, 'saved navigation remains compatible');
      assert.equal(dom.window.localStorage.getItem('unrelated-setting'), 'preserved');
    } finally { dom.window.close(); }
  });

  test(`${path} keeps resumed and newly saved business drafts credential-free`, async () => {
    const submissions = [];
    const writes = [];
    const dom = await setup(path, async (url, options = {}) => {
      if (options.method === 'POST') submissions.push(JSON.parse(options.body));
      return { ok: true, json: async () => options.method === 'POST' ? {} : { business_name: 'Server Studio', step: 5, ...legacyCredentials } };
    }, {
      stored: { business_name: 'Local Studio', first_offer: 'Initial service', ...legacyCredentials },
      prepare(window) {
        const original = window.Storage.prototype.setItem;
        window.Storage.prototype.setItem = function (key, value) {
          if (key === draftKey) writes.push(JSON.parse(value).state ?? JSON.parse(value));
          return original.call(this, key, value);
        };
      },
    });
    try {
      const document = dom.window.document;
      assert.equal(document.querySelector('input[type="password"], #admin-email, #admin-name'), null);
      assert.ok(document.getElementById('step-admin').classList.contains('active'), 'resumes the existing team review step');
      document.querySelector('#step-admin .next-step-btn').click();
      assert.ok(document.getElementById('step-offer').classList.contains('active'), 'business setup needs no replacement account');
      const offer = document.getElementById('first-offer');
      offer.value = 'Updated service';
      offer.dispatchEvent(new dom.window.Event('input', { bubbles: true }));
      await new Promise(resolve => setTimeout(resolve, 1600));
      document.querySelector('#step-offer .save-draft-btn').click();
      await new Promise(resolve => setTimeout(resolve, 150));
      assert.ok(writes.length > 0, 'real navigation and autosave handlers persist drafts');
      assert.ok(submissions.length > 0, 'the actual draft request is submitted');
      for (const draft of [...writes, ...submissions]) assertBusinessOnly(draft);
      const saved = JSON.parse(dom.window.localStorage.getItem(draftKey)).state;
      assert.equal(saved.business_name, 'Server Studio');
      assert.equal(saved.first_offer, 'Updated service');
      assertBusinessOnly(saved);
    } finally { dom.window.close(); }
  });
}

const preparation = { preparation_id: 'prep-1', status: 'prepared', organization_id: 'org-1', user_id: 'user-1', primary_product_id: 'p1', reviewed_request: { company_name: 'Studio', first_product_name: 'Consulting', first_product_price: '25.00' }, catalog: [{ product_id: 'p1', name: 'Consulting', price: '25.00', description: '', variants: [] }] };
for (const path of paths) {
 test(`${path} never says Saved for a200 rejected draft and sends no forged authority`, async () => {
  const requests = [];
  const dom = await setup(path, async (url, options = {}) => {
   if (options.method === 'POST' && url.endsWith('/draft')) { requests.push(options); return Response.json({success:false,error:'Disk save failed'}); }
   return Response.json({});
  });
  try {
   const button = dom.window.document.querySelector('.save-draft-btn'); button.click();
   await new Promise(r => setTimeout(r,20));
   assert.equal(button.textContent, 'Error!'); assert.equal(requests.length,1);
   assert.deepEqual(Object.keys(requests[0].headers), ownerHeaders);
  } finally { dom.window.close(); }
 });
}
const prepared = { success: true, ...preparation, preparation };
for (const path of paths) {
 test(`${path} refuses malformed start acknowledgement and never marks onboarded`, async () => {
  let launches = 0;
  const dom = await setup(path, async url => {
   if (url.endsWith('/start')) return { ok: true, json: async () => ({ success: false }) };
   if (url.endsWith('/launch')) launches++;
   return { ok: true, json: async () => ({}) };
  });
  try {
   dom.window.document.getElementById('template-selection').value = 'Modern'; dom.window.document.getElementById('finish-btn').click();
   await new Promise(r => setTimeout(r, 20));
   assert.equal(launches, 0); assert.equal(dom.window.localStorage.getItem('has_onboarded'), null);
   assert.match(dom.window.document.getElementById('submit-error').textContent, /acknowledged|verified/i);
  } finally { dom.window.close(); }
 });
 test(`${path} holds a committed preparation after malformed launch and reuses it`, async () => {
  let starts = 0; const launches = []; let stateReads = 0;
  const dom = await setup(path, async (url, options = {}) => {
   if (url.endsWith('/start')) { starts++; return { ok: true, json: async () => prepared }; }
   if (url.endsWith('/launch')) { launches.push(JSON.parse(options.body)); return { ok: true, json: async () => ({}) }; }
   if (url.endsWith('/state')) { stateReads++; return { ok: true, json: async () => stateReads > 1 ? { preparation } : {} }; }
   return { ok: true, json: async () => ({}) };
  });
  try {
   dom.window.document.getElementById('template-selection').value = 'Modern';
   dom.window.document.getElementById('finish-btn').click(); await new Promise(r => setTimeout(r,20));
   assert.equal(dom.window.localStorage.getItem('has_onboarded'), null);
   dom.window.document.getElementById('finish-btn').click(); await new Promise(r => setTimeout(r,20));
   assert.equal(starts,1); assert.deepEqual(launches,[{ preparation_id: 'prep-1' },{ preparation_id: 'prep-1' }]);
  } finally { dom.window.close(); }
 });
 test(`${path} blank instant validation leaves the submission lock clear`, async () => {
  const dom = await setup(path, async () => ({ ok: true, json: async () => ({}) }));
  try {
   const button = dom.window.document.getElementById('generate-storefront-btn'); button.disabled = false; button.click();
   assert.equal(button.dataset.submitting, undefined);
  } finally { dom.window.close(); }
 });
 test(`${path} instant preparation opens review without launching or fabricating identity`, async () => {
  let options; let launches = 0;
  const dom = await setup(path, async (url, request) => {
   if (url.endsWith('/start_zero_click')) { options = request; return { ok: true, json: async () => prepared }; }
   if (url.endsWith('/launch')) launches++;
   return { ok: true, json: async () => ({}) };
  });
  try {
   const input = dom.window.document.getElementById('instant-bio'); input.value = 'Consulting studio'; input.dispatchEvent(new dom.window.Event('input'));
   dom.window.document.getElementById('generate-storefront-btn').click(); await new Promise(r => setTimeout(r,20));
   assert.ok(options); assert.deepEqual(Object.keys(options.headers), ownerHeaders); assert.equal(launches,0);
   assert.equal(dom.window.localStorage.getItem('has_onboarded'),null); assert.ok(dom.window.document.getElementById('step-approval').classList.contains('active'));
  } finally { dom.window.close(); }
 });
 test(`${path} ignores preparation after navigation to another step`, async () => {
  let resolve; let launches = 0;
  const dom = await setup(path, async url => {
   if (url.endsWith('/start')) return new Promise(done => { resolve = done; });
   if (url.endsWith('/launch')) launches++;
   return { ok:true,json:async()=>({}) };
  });
  try {
   dom.window.document.getElementById('template-selection').value='Modern'; dom.window.document.getElementById('finish-btn').click();
   await waitUntil(()=>typeof resolve==='function');
   dom.window.goToStep('step-chat'); resolve({ok:true,json:async()=>prepared}); await new Promise(r=>setTimeout(r,20));
   assert.equal(launches,0); assert.equal(dom.window.localStorage.getItem('has_onboarded'),null);
  } finally { dom.window.close(); }
 });
}

for (const path of paths) {
 test(`${path} uses the same-origin canonical chat proxy with session credentials`, async () => {
  let submitted;
  const dom = await setup(path, async (url, options = {}) => {
   if (url.endsWith('/chat')) submitted = {url,options};
   return Response.json(url.endsWith('/chat') ? {reply:'Tell me more',is_complete:false} : {});
  }, {url:'http://localhost:1420/setup.html', prepare(window) { window.__TAURI__={core:{invoke(){throw new Error('Unexpected native invoke');}}}; }});
  try {
   dom.window.goToStep('step-chat'); dom.window.document.getElementById('chat-input').value='A studio'; dom.window.document.getElementById('chat-send-btn').click();
   await new Promise(r=>setTimeout(r,20));
   assert.equal(submitted.url,'/api/v1/onboarding/chat');
   assert.equal(submitted.options.credentials,'same-origin');
   assert.deepEqual(Object.keys(submitted.options.headers),ownerHeaders);
  } finally { dom.window.close(); }
 });
}

for (const path of paths) {
 test(`${path} rejects a foreign revision receipt without launching`, async () => {
  let launches = 0;
  const dom = await setup(path, async url => {
   if (url.endsWith('/state')) return Response.json({preparation});
   if (url.endsWith('/start')) return Response.json({...prepared,organization_id:'foreign',preparation:{...preparation,organization_id:'foreign'}});
   if (url.endsWith('/launch')) launches++;
   return Response.json({});
  });
  try {
   dom.window.document.getElementById('template-selection').value='Modern';
   dom.window.document.getElementById('finish-btn').click(); await new Promise(r=>setTimeout(r,20));
   assert.equal(launches,0); assert.match(dom.window.document.getElementById('submit-error').textContent,/identity does not match/);
   assert.equal(dom.window.localStorage.getItem('has_onboarded'),null);
  } finally { dom.window.close(); }
 });
}
