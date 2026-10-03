import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { test } from 'node:test';
import { JSDOM, VirtualConsole } from './test-support/offline-dom.mjs';

const files = [
  'src/ui/next/public/dashboard.html',
  'src/ui/next/public/ui/dashboard.html',
  'src/ui/tauri/src/ui/dashboard.html',
];
const turn = () => new Promise(resolve => setImmediate(resolve));
const proposal = {
  id: 'recorded-approval', action_type: 'Draft Reply', lifecycle_state: 'PROPOSED',
  proposed_action: { customer_name: 'Fixture customer', project_description: 'Recorded quote', total_cost: 12500 },
};
const unrelated = [
  ['/api/v1/growth/referrals/tier', { current_tier: 'Free', total_conversions: 0 }],
  ['/api/v1/growth/team-invites/aggregated-metrics', { total_invites: 0, metrics: {} }],
  ['/api/v1/growth/milestones/check', { milestones: [] }],
  ['/api/v1/growth/wrapped', { total_sales: 0, total_orders: 0 }],
];

for (const file of files) {
  for (const [endpoint, payload] of unrelated) {
    test(`${file}: a late ${endpoint} response cannot erase recorded approvals`, async () => {
      let release;
      let delayedReads = 0;
      let currentProposal = proposal;
      const gate = new Promise(resolve => { release = resolve; });
      const errors = [];
      const virtualConsole = new VirtualConsole();
      virtualConsole.on('jsdomError', error => errors.push(String(error)));
      virtualConsole.on('error', (...args) => errors.push(args.map(String).join(' ')));
      const dom = new JSDOM(await readFile(file, 'utf8'), {
        url: 'https://app.example.test/dashboard.html', runScripts: 'dangerously',
        virtualConsole,
        beforeParse(window) {
          window.Headers = Headers;
          window.Response = Response;
          window.alert = () => {};
          window.matchMedia = () => ({ matches: false, addEventListener() {}, removeEventListener() {} });
          window.fetch = async url => {
            const path = new URL(String(url), window.location.href).pathname;
            if (path === '/api/v1/ui/dashboard/unified-agent-feed') {
              return Response.json({ pending_approvals: [currentProposal], agent_feed: [] });
            }
            if (path === endpoint) {
              delayedReads++;
              await gate;
              return Response.json(payload);
            }
            return Response.json({ error: 'No fixture for this unrelated request' }, { status: 503 });
          };
        },
      });
      try {
        await turn(); await turn();
        const document = dom.window.document;
        assert.ok(document.getElementById('triage-recorded-approval'), `the actual feed reader must first render its recorded approval: ${errors.join('; ')}; ${document.getElementById('triage-queue').innerHTML.slice(0,500)}`);
        assert.ok(delayedReads > 0, 'the actual unrelated reader must be waiting on its response');
        release();
        await turn(); await turn();
        document.getElementById('tab-activity').click();
        document.getElementById('tab-proposals').click();
        assert.ok(document.getElementById('triage-recorded-approval'), 'changing tabs must retain the server feed after unrelated requests finish');
        currentProposal = { ...proposal, id: 'new-recorded-approval' };
        await dom.window.loadUnifiedFeed();
        assert.ok(document.getElementById('triage-new-recorded-approval'), 'the actual feed endpoint must still update its own records');
        assert.equal(document.getElementById('triage-recorded-approval'), null, 'a real feed refresh replaces obsolete records');
      } finally { release(); await turn(); await turn(); dom.window.close(); }
    });
  }
}

for (const file of files) {
  test(`${file}: actual quote and invoice proposals render their own payloads`, async () => {
    const dom = new JSDOM(await readFile(file, 'utf8'), {
      url: 'https://app.example.test/dashboard.html', runScripts: 'dangerously',
      virtualConsole: new VirtualConsole(),
      beforeParse(window) {
        window.Headers = Headers;
        window.Response = Response;
        window.matchMedia = () => ({ matches: false, addEventListener() {}, removeEventListener() {} });
        window.fetch = async url => new URL(String(url), window.location.href).pathname === '/api/v1/ui/dashboard/unified-agent-feed'
          ? Response.json({ pending_approvals: [
            { ...proposal, id: 'quote', action_type: 'ReviewDraftQuote' },
            { id: 'invoice', action_type: 'invoice_draft', proposed_action: { amount_cents: 27500, project_name: 'Recorded project', milestone_name: 'Recorded milestone' } },
          ], agent_feed: [] })
          : Response.json({ error: 'Unrelated fixture unavailable' }, { status: 503 });
      },
    });
    try {
      await turn(); await turn();
      const quote = dom.window.document.getElementById('triage-quote');
      const invoice = dom.window.document.getElementById('triage-invoice');
      assert.ok(quote, 'quote records must not fall into the empty-feed fallback after a renderer exception');
      assert.match(quote.textContent, /Recorded quote for Fixture customer/);
      assert.ok(invoice, 'invoice records must render without referencing another callback’s group variable');
      assert.match(invoice.textContent, /Recorded project - Recorded milestone/);
      assert.match(invoice.textContent, /\$275\.00/);
    } finally { dom.window.close(); }
  });
}
