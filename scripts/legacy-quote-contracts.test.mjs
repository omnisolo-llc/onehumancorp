import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { JSDOM, VirtualConsole } from './test-support/offline-dom.mjs';

const files = ['src/ui/tauri/src/ui/quote.html', 'src/ui/next/public/quote.html', 'src/ui/next/public/ui/quote.html'];
const quoteId = 'be72a36d-1dfd-46c0-b52d-29147bbbaacf';
const invoiceId = '026e3349-c238-486b-a276-cdcfd810f984';
const receipt = extra => ({ success: true, status: 'accepted', quote_id: quoteId, invoice_id: invoiceId, invoice_status: 'Draft', payment_status: 'unverified', stripe_payment_link: '', checkout_status: 'not_configured', ...extra });
const tick = () => new Promise(resolve => setTimeout(resolve, 15));
async function load(file, options = {}) {
  const requests = [], alerts = [], errors = [];
  const state = { id: quoteId, tenant_id: 'verified-tenant', status: options.status ?? 'SENT', updated_at: options.noVersion ? null : options.version ?? '2026-10-01T03:00:00Z', total_amount_cents: 5000, required_deposit_cents: 1000 };
  let storedLines = structuredClone(options.lineItems ?? [{ description: options.description ?? 'Reviewed service', unit_price_cents: options.linePrice ?? 5000, quantity: 1, is_optional: false, service_item_id: null }]);
  let writes = 0;
  const persist = payload => {
    state.total_amount_cents = payload.total_amount;
    state.required_deposit_cents = payload.required_deposit;
    state.status = payload.status ?? state.status;
    state.updated_at = `2026-10-01T04:00:00.${String(writes).padStart(6, '0')}+00:00`;
    storedLines = structuredClone(payload.line_items);
  };
  const console = new VirtualConsole(); console.on('jsdomError', error => errors.push(error.message));
  const html = await readFile(new URL(`../${file}`, import.meta.url), 'utf8');
  const dom = new JSDOM(html, { url: `https://workspace.example/quote.html?id=${quoteId}&tenant=forged-tenant${options.owner ? '&mode=owner' : ''}`, runScripts: 'dangerously', pretendToBeVisual: true, virtualConsole: console, beforeParse(window) {
    window.alert = message => alerts.push(message);
    window.fetch = async (url, init = {}) => {
      requests.push({ url, init });
      if (url === `/api/v1/quotes/${quoteId}/accept`) {
        if (options.accept) return options.accept();
        state.status = 'ACCEPTED';
        return Response.json(receipt({ stripe_payment_link: options.paymentLink ?? '', checkout_status: options.paymentLink ? 'available' : 'not_configured' }));
      }
      if (url === `/api/v1/quotes/${quoteId}` && init.method === 'PUT') {
        const payload = JSON.parse(init.body); writes++;
        if (options.save) return options.save(payload, state, persist);
        persist(payload);
        return Response.json({ success: true, updated_at: state.updated_at });
      }
      if (url === `/api/v1/quotes/${quoteId}`) {
        const data = structuredClone({ quote: state, acceptance: state.status === 'ACCEPTED' ? options.loadedReceipt ?? receipt() : undefined, line_items: storedLines });
        if (writes && options.readback) options.readback(data);
        return Response.json(data);
      }
      return Response.json([]);
    };
  } });
  await new Promise(resolve => dom.window.addEventListener('load', resolve, { once: true })); await tick();
  return { dom, doc: dom.window.document, requests, alerts, errors };
}
for (const file of files) {
  test(`${file}: acceptance is an unpaid receipt and referral uses server tenant`, async () => {
    const x = await load(file);
    try {
      x.doc.getElementById('btn-pay-card').click(); await tick();
      assert.deepEqual(JSON.parse(x.requests.find(r => r.url.endsWith('/accept')).init.body), { expected_updated_at: '2026-10-01T03:00:00Z' });
      assert.equal(x.doc.getElementById('quote-status').textContent, 'Accepted — payment pending');
      assert.match(x.doc.getElementById('quote-acceptance').textContent, new RegExp(invoiceId));
      assert.equal(x.doc.getElementById('continue-payment').hidden, true);
      assert.equal(x.dom.window.location.pathname, '/quote.html');
      assert.doesNotMatch(x.doc.body.textContent, /Deposit Paid!/);
      assert.equal(x.doc.querySelector('[data-testid="referral-success-card"] a').getAttribute('href'), '/setup.html?ref=verified-tenant&source=quote_accepted');
      assert.deepEqual(x.errors, []);
    } finally { x.dom.window.close(); }
  });
  test(`${file}: edit dialog and backdrop cover the floating referral badge`, async () => {
    const x = await load(file, { owner: true });
    try {
      x.doc.getElementById('btn-edit-quote').click();
      const sheet = x.doc.getElementById('edit-quote-sheet');
      const overlay = x.doc.getElementById('edit-sheet-overlay');
      const badge = x.doc.getElementById('viral-badge');
      const stack = element => Number(x.dom.window.getComputedStyle(element).zIndex);
      assert.equal(sheet.style.display, 'block');
      assert.ok(stack(sheet) > stack(overlay), 'the edit form must be above its backdrop');
      assert.ok(stack(overlay) > stack(badge), 'the referral must not intercept modal editing');
    } finally { x.dom.window.close(); }
  });
  test(`${file}: accepted reload never posts acceptance again`, async () => {
    const x = await load(file, { status: 'ACCEPTED' });
    try {
      assert.equal(x.doc.getElementById('quote-status').textContent, 'Accepted — payment pending');
      x.doc.getElementById('btn-pay-card').click(); await tick();
      assert.equal(x.requests.filter(r => r.url.endsWith('/accept')).length, 0);
      assert.equal(x.doc.querySelector('[data-testid="referral-success-card"]').hidden, false);
    } finally { x.dom.window.close(); }
  });
  test(`${file}: accepted owner view cannot reopen or resubmit commercial terms`, async () => {
    const x = await load(file, { status: 'ACCEPTED', owner: true });
    try {
      assert.equal(x.doc.getElementById('btn-approve-send').disabled, true);
      assert.equal(x.doc.getElementById('btn-edit-quote').disabled, true);
      assert.equal(x.doc.getElementById('deposit-slider').disabled, true);
      x.doc.getElementById('btn-approve-send').dispatchEvent(new x.dom.window.Event('click'));
      x.doc.getElementById('btn-save-edits').dispatchEvent(new x.dom.window.Event('click'));
      await tick();
      assert.equal(x.requests.filter(r => r.init.method === 'PUT').length, 0);
    } finally { x.dom.window.close(); }
  });
  test(`${file}: paid invoice replay preserves recorded state without inviting payment again`, async () => {
    const x = await load(file, { status: 'ACCEPTED', loadedReceipt: receipt({ invoice_status: 'Paid', payment_status: 'paid', checkout_status: 'available', stripe_payment_link: 'https://checkout.stripe.com/c/pay/cs_test_fixture' }) });
    try {
      assert.equal(x.doc.getElementById('quote-status').textContent, 'Quote accepted');
      assert.match(x.doc.getElementById('quote-invoice-reference').textContent, /invoice status: Paid/);
      assert.match(x.doc.getElementById('quote-invoice-reference').textContent, /payment status: paid/);
      assert.equal(x.doc.getElementById('continue-payment').hidden, true);
      assert.equal(x.requests.filter(r => r.url.endsWith('/accept')).length, 0);
    } finally { x.dom.window.close(); }
  });
  test(`${file}: missing observed version cannot submit first acceptance`, async () => {
    const x = await load(file, { noVersion: true });
    try {
      assert.equal(x.doc.getElementById('btn-pay-card').disabled, true);
      x.doc.getElementById('btn-pay-card').dispatchEvent(new x.dom.window.Event('click'));
      await tick();
      assert.equal(x.requests.filter(r => r.url.endsWith('/accept')).length, 0);
    } finally { x.dom.window.close(); }
  });
  test(`${file}: a receipt for another quote cannot acknowledge this quote`, async () => {
    const x = await load(file, { accept: () => Response.json(receipt({ quote_id: invoiceId })) });
    try {
      x.doc.getElementById('btn-pay-card').click(); await tick();
      assert.equal(x.doc.getElementById('quote-acceptance').hidden, true);
      assert.equal(x.doc.getElementById('continue-payment').hidden, true);
      assert.equal(x.doc.getElementById('btn-pay-card').disabled, true);
    } finally { x.dom.window.close(); }
  });
  test(`${file}: unknown acceptance cannot become paid or be repeated`, async () => {
    let finish; const pending = new Promise(resolve => { finish = resolve; });
    const x = await load(file, { accept: () => pending });
    try {
      const button = x.doc.getElementById('btn-pay-card'); button.click(); button.click();
      assert.equal(x.requests.filter(r => r.url.endsWith('/accept')).length, 1);
      finish(Response.json({}, { status: 503 })); await tick(); button.click(); await tick();
      assert.equal(button.disabled, true);
      assert.equal(x.requests.filter(r => r.url.endsWith('/accept')).length, 1);
      assert.match(x.doc.getElementById('quote-action-status').textContent, /confirm|reconcil/i);
      assert.notEqual(x.doc.getElementById('quote-status').textContent, 'Accepted — payment pending');
    } finally { x.dom.window.close(); }
  });
  test(`${file}: failed owner update never claims message delivery`, async () => {
    const x = await load(file, { owner: true, save: () => Response.json({ success: false }, { status: 500 }) });
    try {
      x.doc.getElementById('btn-approve-send').click(); await tick();
      assert.doesNotMatch(x.alerts.join(' '), /sent to customer/i);
      assert.match(x.doc.getElementById('quote-action-status').textContent, /not confirm|failed/i);
      assert.equal(x.dom.window.location.pathname, '/quote.html');
    } finally { x.dom.window.close(); }
  });
  test(`${file}: owner save carries the exact reviewed version with edited terms`, async () => {
    const version = '2026-10-01T03:00:00.123456+00:00';
    const x = await load(file, { owner: true, version });
    try {
      x.doc.getElementById('btn-edit-quote').click();
      const description = x.doc.querySelector('.edit-desc');
      description.value = 'Reviewed sink repair'; description.dispatchEvent(new x.dom.window.Event('change'));
      x.doc.querySelector('.edit-price').value = '62.50';
      x.doc.getElementById('edit-total-amount').value = '62.50';
      x.doc.getElementById('edit-required-deposit').value = '12.50';
      x.doc.getElementById('btn-save-edits').click(); await tick();
      const writes = x.requests.filter(r => r.init.method === 'PUT');
      assert.equal(writes.length, 1);
      assert.deepEqual(JSON.parse(writes[0].init.body), {
        expected_updated_at: version, total_amount: 6250, required_deposit: 1250,
        line_items: [{ description: 'Reviewed sink repair', unit_price_cents: 6250, quantity: 1, is_optional: false, service_item_id: null }],
      });
      assert.match(x.doc.getElementById('quote-action-status').textContent, /changes saved/);
    } finally { x.dom.window.close(); }
  });
  test(`${file}: owner approval carries the exact reviewed version and approval status`, async () => {
    const version = '2026-10-01T03:00:00.654321+00:00';
    const x = await load(file, { owner: true, version });
    try {
      const slider = x.doc.getElementById('deposit-slider');
      slider.value = '50'; slider.dispatchEvent(new x.dom.window.Event('input'));
      x.doc.getElementById('btn-approve-send').click(); await tick();
      const writes = x.requests.filter(r => r.init.method === 'PUT');
      assert.equal(writes.length, 1);
      assert.deepEqual(JSON.parse(writes[0].init.body), {
        expected_updated_at: version, total_amount: 5000, required_deposit: 2500,
        line_items: [{ description: 'Reviewed service', unit_price_cents: 5000, quantity: 1, is_optional: false, service_item_id: null }], status: 'SENT',
      });
      assert.match(x.doc.getElementById('quote-action-status').textContent, /approval saved.*delivery.*not confirmed/);
    } finally { x.dom.window.close(); }
  });
  test(`${file}: missing observed version holds every owner mutation control and handler`, async () => {
    const x = await load(file, { owner: true, noVersion: true });
    try {
      for (const id of ['btn-edit-quote', 'btn-save-edits', 'btn-approve-send', 'btn-add-line-item', 'deposit-slider']) {
        assert.equal(x.doc.getElementById(id).disabled, true, id);
        x.doc.getElementById(id).dispatchEvent(new x.dom.window.Event(id === 'deposit-slider' ? 'input' : 'click'));
      }
      assert.equal(x.doc.getElementById('edit-total-amount').readOnly, true);
      assert.equal(x.doc.getElementById('edit-required-deposit').readOnly, true);
      await tick();
      assert.equal(x.doc.getElementById('edit-quote-sheet').style.display, 'none');
      assert.equal(x.doc.querySelectorAll('.edit-desc').length, 0);
      assert.equal(x.requests.filter(r => r.init.method === 'PUT').length, 0);
    } finally { x.dom.window.close(); }
  });
  test(`${file}: approval after saved edits uses the newly loaded exact version`, async () => {
    const version = '2026-10-01T03:00:00.000001+00:00';
    const savedVersion = '2026-10-01T03:00:00.000009+00:00';
    const x = await load(file, { owner: true, version, save: (payload, state, persist) => {
      const nextVersion = state.updated_at === version ? savedVersion : '2026-10-01T03:00:00.000019+00:00';
      persist(payload);
      state.updated_at = nextVersion;
      return Response.json({ success: true, updated_at: nextVersion });
    } });
    try {
      x.doc.getElementById('btn-edit-quote').click();
      x.doc.getElementById('btn-save-edits').click(); await tick();
      assert.equal(x.doc.getElementById('edit-quote-sheet').style.display, 'none');
      x.doc.getElementById('btn-approve-send').click(); await tick();
      const writes = x.requests.filter(r => r.init.method === 'PUT').map(r => JSON.parse(r.init.body));
      assert.equal(writes.length, 2);
      assert.equal(writes[0].expected_updated_at, version);
      assert.equal(writes[1].expected_updated_at, savedVersion);
      assert.equal(writes[1].status, 'SENT');
      assert.equal(x.requests.filter(r => r.url === `/api/v1/quotes/${quoteId}` && !r.init.method).length, 3);
    } finally { x.dom.window.close(); }
  });
  test(`${file}: concurrent readback cannot replace saved edits or supply approval authority`, async () => {
    const version = '2026-10-01T03:00:00.000001+00:00';
    const savedVersion = '2026-10-01T03:00:00.000009+00:00';
    const x = await load(file, { owner: true, version, save: (_payload, state) => {
      state.updated_at = '2026-10-01T03:00:00.000010+00:00';
      state.total_amount_cents = 100000;
      return Response.json({ success: true, updated_at: savedVersion });
    } });
    try {
      x.doc.getElementById('btn-edit-quote').click();
      const input = x.doc.querySelector('.edit-desc');
      input.value = 'My saved edit'; input.dispatchEvent(new x.dom.window.Event('change'));
      x.doc.getElementById('edit-total-amount').value = '75';
      x.doc.getElementById('btn-save-edits').click(); await tick();
      assert.equal(input.value, 'My saved edit');
      assert.equal(x.doc.getElementById('edit-total-amount').value, '75');
      assert.equal(x.doc.getElementById('edit-quote-sheet').style.display, 'block');
      assert.match(x.doc.getElementById('quote-action-status').textContent, /not confirm/);
      assert.doesNotMatch(x.doc.getElementById('quote-action-status').textContent, /changes saved/);
      assert.equal(x.doc.getElementById('btn-approve-send').disabled, true);
      x.doc.getElementById('btn-approve-send').dispatchEvent(new x.dom.window.Event('click')); await tick();
      assert.equal(x.requests.filter(r => r.init.method === 'PUT').length, 1);
    } finally { x.dom.window.close(); }
  });
  test(`${file}: concurrent approval readback cannot claim the loaded version was approved`, async () => {
    const x = await load(file, { owner: true, save: (_payload, state) => {
      state.updated_at = '2026-10-01T03:00:00.000010+00:00';
      return Response.json({ success: true, updated_at: '2026-10-01T03:00:00.000009+00:00' });
    } });
    try {
      x.doc.getElementById('btn-approve-send').click(); await tick();
      assert.match(x.doc.getElementById('quote-action-status').textContent, /not confirm quote approval/);
      assert.equal(x.doc.getElementById('btn-approve-send').disabled, true);
      assert.equal(x.doc.getElementById('btn-edit-quote').disabled, true);
    } finally { x.dom.window.close(); }
  });
  test(`${file}: owner success without a committed version is held for reconciliation`, async () => {
    for (const action of ['btn-save-edits', 'btn-approve-send']) {
      const x = await load(file, { owner: true, save: () => Response.json({ success: true }) });
      try {
        if (action === 'btn-save-edits') x.doc.getElementById('btn-edit-quote').click();
        x.doc.getElementById(action).click(); await tick();
        assert.match(x.doc.getElementById('quote-action-status').textContent, /not confirm/);
        assert.equal(x.doc.getElementById('btn-approve-send').disabled, true);
        assert.equal(x.requests.filter(r => r.url === `/api/v1/quotes/${quoteId}` && !r.init.method).length, 1);
        if (action === 'btn-save-edits') assert.equal(x.doc.getElementById('edit-quote-sheet').style.display, 'block');
      } finally { x.dom.window.close(); }
    }
  });
  const richLines = [
    { description: 'Catalog service', unit_price_cents: 5000, quantity: 2, is_optional: false, service_item_id: invoiceId },
    { description: 'Signed adjustment', unit_price_cents: -500, quantity: 1, is_optional: true, service_item_id: null },
    { description: 'Catalog service', unit_price_cents: 5000, quantity: 2, is_optional: false, service_item_id: invoiceId },
  ];
  const changedTerms = [
    ['total', data => { data.quote.total_amount_cents++; }],
    ['deposit', data => { data.quote.required_deposit_cents++; }],
    ['description', data => { data.line_items[0].description = 'Unreviewed replacement'; }],
    ['signed price', data => { data.line_items[1].unit_price_cents = 500; }],
    ['quantity', data => { data.line_items[0].quantity++; }],
    ['optional flag', data => { data.line_items[1].is_optional = false; }],
    ['catalog reference', data => { data.line_items[0].service_item_id = null; }],
    ['line multiplicity', data => { data.line_items[0] = { ...data.line_items[1] }; }],
  ];
  for (const [label, readback] of changedTerms) {
    for (const action of ['save', 'approval']) {
      test(`${file}: ${action} rejects same-token readback with mismatched ${label}`, async () => {
        const x = await load(file, { owner: true, status: 'DRAFT', lineItems: richLines, readback });
        try {
          x.doc.getElementById('btn-edit-quote').click();
          x.doc.getElementById('edit-total-amount').value = '75';
          const button = x.doc.getElementById(action === 'save' ? 'btn-save-edits' : 'btn-approve-send');
          button.click(); await tick();
          assert.match(x.doc.getElementById('quote-action-status').textContent, /not confirm/);
          assert.doesNotMatch(x.doc.getElementById('quote-action-status').textContent, /changes saved|approval saved/);
          assert.equal(x.doc.getElementById('edit-quote-sheet').style.display, 'block');
          assert.equal(x.doc.getElementById('edit-total-amount').value, '75');
          assert.equal(x.doc.querySelector('.edit-desc').value, 'Catalog service');
          assert.equal(x.doc.getElementById('btn-approve-send').disabled, true);
          button.dispatchEvent(new x.dom.window.Event('click')); await tick();
          assert.equal(x.requests.filter(r => r.init.method === 'PUT').length, 1);
        } finally { x.dom.window.close(); }
      });
    }
  }
  test(`${file}: approval rejects same-token readback without the requested SENT status`, async () => {
    for (const status of ['DRAFT', 'APPROVED', 'ACCEPTED', null]) {
      const x = await load(file, { owner: true, status: 'DRAFT', readback: data => { data.quote.status = status; } });
      try {
        x.doc.getElementById('btn-approve-send').click(); await tick();
        assert.match(x.doc.getElementById('quote-action-status').textContent, /not confirm quote approval/);
        assert.equal(x.doc.getElementById('btn-approve-send').disabled, true);
        assert.equal(x.doc.getElementById('quote-acceptance').hidden, true);
      } finally { x.dom.window.close(); }
    }
  });
  test(`${file}: matching persisted signed optional catalog terms permit regenerated IDs and reordered duplicates`, async () => {
    const x = await load(file, { owner: true, status: 'DRAFT', lineItems: richLines, readback: data => {
      data.line_items = [...data.line_items.slice(1), data.line_items[0]];
      data.line_items = data.line_items.map((line, index) => ({ ...line, id: `regenerated-${index}`, quote_id: quoteId, created_at: data.quote.updated_at, updated_at: data.quote.updated_at }));
    } });
    try {
      x.doc.getElementById('btn-edit-quote').click();
      x.doc.getElementById('edit-total-amount').value = '75';
      x.doc.getElementById('edit-required-deposit').value = '15';
      x.doc.getElementById('btn-save-edits').click(); await tick();
      assert.match(x.doc.getElementById('quote-action-status').textContent, /changes saved/);
      assert.equal(x.doc.getElementById('edit-quote-sheet').style.display, 'none');
      assert.equal(x.doc.getElementById('quote-total').innerText, '$75.00');
      assert.equal(x.doc.getElementById('deposit-amount').innerText, '$15.00');
      x.doc.getElementById('btn-approve-send').click(); await tick();
      assert.match(x.doc.getElementById('quote-action-status').textContent, /approval saved/);
      const writes = x.requests.filter(r => r.init.method === 'PUT').map(r => JSON.parse(r.init.body));
      assert.equal(writes.length, 2);
      assert.notEqual(writes[0].expected_updated_at, writes[1].expected_updated_at);
      assert.equal(writes[1].status, 'SENT');
      assert.equal(writes[1].line_items.length, 3);
      assert.equal(writes[1].line_items.filter(line => line.unit_price_cents === -500 && line.is_optional && line.service_item_id === null).length, 1);
      assert.equal(writes[1].line_items.filter(line => line.service_item_id === invoiceId && line.quantity === 2).length, 2);
    } finally { x.dom.window.close(); }
  });
  test(`${file}: stale owner save preserves edits without fetching a new token or retrying`, async () => {
    const version = '2026-10-01T03:00:00.123456+00:00';
    const x = await load(file, { owner: true, version, save: () => Response.json({ error: 'Quote changed; review it again.' }, { status: 409 }) });
    try {
      x.doc.getElementById('btn-edit-quote').click();
      const input = x.doc.querySelector('.edit-desc');
      input.value = 'Keep my reviewed edit'; input.dispatchEvent(new x.dom.window.Event('change'));
      x.doc.getElementById('edit-total-amount').value = '75';
      x.doc.getElementById('btn-save-edits').click(); await tick();
      assert.equal(input.value, 'Keep my reviewed edit');
      assert.equal(x.doc.getElementById('edit-total-amount').value, '75');
      assert.equal(x.doc.getElementById('edit-quote-sheet').style.display, 'block');
      assert.match(x.doc.getElementById('quote-action-status').textContent, /not confirm/);
      assert.doesNotMatch(x.doc.getElementById('quote-action-status').textContent, /changes saved/);
      x.doc.getElementById('btn-save-edits').dispatchEvent(new x.dom.window.Event('click'));
      x.doc.getElementById('btn-approve-send').dispatchEvent(new x.dom.window.Event('click')); await tick();
      const writes = x.requests.filter(r => r.init.method === 'PUT');
      assert.equal(writes.length, 1);
      assert.equal(JSON.parse(writes[0].init.body).expected_updated_at, version);
      assert.equal(x.requests.filter(r => r.url === `/api/v1/quotes/${quoteId}` && !r.init.method).length, 1);
    } finally { x.dom.window.close(); }
  });
  test(`${file}: stale owner approval cannot refresh and approve a different version`, async () => {
    const version = '2026-10-01T03:00:00.123456+00:00';
    const x = await load(file, { owner: true, version, save: () => Response.json({ error: 'Quote changed; review it again.' }, { status: 409 }) });
    try {
      x.doc.getElementById('btn-approve-send').click(); await tick();
      assert.match(x.doc.getElementById('quote-action-status').textContent, /not confirm quote approval/);
      assert.equal(x.doc.getElementById('btn-approve-send').disabled, true);
      x.doc.getElementById('btn-approve-send').dispatchEvent(new x.dom.window.Event('click')); await tick();
      const writes = x.requests.filter(r => r.init.method === 'PUT');
      assert.equal(writes.length, 1);
      assert.equal(JSON.parse(writes[0].init.body).expected_updated_at, version);
      assert.equal(x.requests.filter(r => r.url === `/api/v1/quotes/${quoteId}` && !r.init.method).length, 1);
    } finally { x.dom.window.close(); }
  });
  test(`${file}: signed line adjustments remain representable without changing pricing policy`, async () => {
    const x = await load(file, { owner: true, linePrice: -500 });
    try {
      assert.match(x.doc.getElementById('line-items-container').textContent, /-\$5\.00/);
      x.doc.getElementById('btn-edit-quote').click();
      assert.equal(x.doc.querySelector('.edit-price').value, '-5');
    } finally { x.dom.window.close(); }
  });
  test(`${file}: only verified receipt links offer a separate payment step`, async () => {
    const x = await load(file, { paymentLink: 'https://checkout.stripe.com/c/pay/cs_test_fixture' });
    try {
      x.doc.getElementById('btn-pay-card').click(); await tick();
      const link = x.doc.getElementById('continue-payment');
      assert.equal(link.hidden, false);
      assert.equal(link.href, 'https://checkout.stripe.com/c/pay/cs_test_fixture');
      assert.match(link.textContent, /Continue to payment/);
      assert.doesNotMatch(x.doc.getElementById('quote-status').textContent, /paid/i);
    } finally { x.dom.window.close(); }
  });
  test(`${file}: unsafe links and contradictory success never prove payment`, async () => {
    const x = await load(file, { paymentLink: 'javascript:alert(1)' });
    try {
      x.doc.getElementById('btn-pay-card').click(); await tick();
      assert.equal(x.doc.getElementById('continue-payment').hidden, true);
      assert.equal(x.doc.getElementById('continue-payment').getAttribute('href'), null);
    } finally { x.dom.window.close(); }
    const y = await load(file, { accept: () => Response.json(receipt({ error: 'contradiction' })) });
    try {
      y.doc.getElementById('btn-pay-card').click(); await tick();
      assert.equal(y.doc.getElementById('quote-acceptance').hidden, true);
      assert.equal(y.doc.getElementById('btn-pay-card').disabled, true);
    } finally { y.dom.window.close(); }
  });
  test(`${file}: a late acceptance cannot mutate a departed document`, async () => {
    let finish; const pending = new Promise(resolve => { finish = resolve; });
    const x = await load(file, { accept: () => pending });
    try {
      x.doc.getElementById('btn-pay-card').click();
      x.dom.window.dispatchEvent(new x.dom.window.Event('pagehide'));
      finish(Response.json(receipt())); await tick();
      assert.equal(x.doc.getElementById('quote-acceptance').hidden, true);
      assert.equal(x.doc.querySelector('[data-testid="referral-success-card"]').hidden, true);
    } finally { x.dom.window.close(); }
  });
  test(`${file}: in-flight editing cannot lose newer text or retry an unknown save`, async () => {
    let finish; const pending = new Promise(resolve => { finish = resolve; });
    const x = await load(file, { owner: true, save: () => pending });
    try {
      x.doc.getElementById('btn-edit-quote').click();
      const input = x.doc.querySelector('.edit-desc'); input.value = 'Reviewed edit'; input.dispatchEvent(new x.dom.window.Event('change'));
      const save = x.doc.getElementById('btn-save-edits'); save.click();
      assert.equal(input.readOnly, true);
      finish(Response.json({ success: false }, { status: 500 })); await tick();
      assert.equal(save.disabled, true);
      assert.equal(input.value, 'Reviewed edit');
      assert.equal(x.doc.getElementById('edit-quote-sheet').style.display, 'block');
      save.click(); await tick();
      assert.equal(x.requests.filter(r => r.init.method === 'PUT').length, 1);
    } finally { x.dom.window.close(); }
  });
  test(`${file}: quote content is inert in display and editor`, async () => {
    const attack = '"><img src=x onerror="window.injected=true">';
    const x = await load(file, { owner: true, description: attack });
    try {
      assert.equal(x.doc.querySelector('#line-items-container img'), null);
      assert.ok(x.doc.getElementById('line-items-container').textContent.includes(attack));
      x.doc.getElementById('btn-edit-quote').click();
      assert.equal(x.doc.querySelector('#edit-line-items-container img'), null);
      assert.equal(x.doc.querySelector('.edit-desc').value, attack);
    } finally { x.dom.window.close(); }
  });
}
