import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { JSDOM } from 'jsdom';

const paths = [
  'src/ui/tauri/src/ui/setup.html',
  ...['setup.html', 'ui/setup.html', 'api/ui/setup.html', 'api/v1/ui/setup.html'].map(p => `src/ui/next/public/${p}`),
];

async function setup(path, fetch, options = {}) {
  const html = await readFile(new URL(`../${path}`, import.meta.url), 'utf8');
  const dom = new JSDOM(html, { url: 'https://workspace.example/setup.html', runScripts: 'outside-only', pretendToBeVisual: true });
  dom.window.fetch = fetch;
  dom.window.localStorage.setItem('onboardingState', JSON.stringify(options.stored ?? { work_context: 'Agency', business_name: 'Nora Studio', first_offer: 'Logo Design', location: 'Portland, OR', target_audience: 'Local founders', categories: 'Design', assistant_name: 'Nora' }));
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
      assert.equal(dom.window.document.getElementById('approve-publish-btn').textContent, 'Approve & Publish');
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
      const draft = JSON.parse(dom.window.localStorage.getItem('onboardingState'));
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
          if (key === 'onboardingState') writes.push(JSON.parse(value));
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
      const saved = JSON.parse(dom.window.localStorage.getItem('onboardingState'));
      assert.equal(saved.business_name, 'Server Studio');
      assert.equal(saved.first_offer, 'Updated service');
      assertBusinessOnly(saved);
    } finally { dom.window.close(); }
  });
}
