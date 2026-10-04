import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import test from 'node:test';
import { observeButtonStates } from './playwright/button-states.mjs';
import { JSDOM, VirtualConsole } from './test-support/offline-dom.mjs';

const pages = [
  'src/ui/next/public/viral-receipt-lottery.html',
  'src/ui/next/public/ui/viral-receipt-lottery.html',
  'src/ui/tauri/src/ui/viral-receipt-lottery.html',
];
const nextTurn = () => new Promise(resolve => setImmediate(resolve));

for (const path of pages) {
  test(`${path}: records the actual busy state even when generation finishes before the assertion`, async () => {
    // Offline transport rejects the real page's fetch. This checks observation
    // timing and failure cleanup; it never fabricates a successful API result.
    const dom = new JSDOM(await readFile(new URL(`../${path}`, import.meta.url), 'utf8'), {
      url: 'https://example.test/viral-receipt-lottery.html',
      runScripts: 'dangerously',
      virtualConsole: new VirtualConsole(),
    });
    let observation;
    try {
      await new Promise(resolve => dom.window.addEventListener('load', resolve, { once: true }));
      await nextTurn();
      const button = dom.window.document.getElementById('generate-btn');
      observation = observeButtonStates(button);
      button.click();
      assert.equal(button.disabled, true);
      assert.equal(button.textContent, 'Generating...');

      await nextTurn();
      // A post-click poll misses this short-lived state, but its recorded
      // transition must remain available to the real browser assertion.
      assert.equal(button.disabled, false);
      assert.equal(button.textContent, 'Generate Lottery Link');
      assert.ok(observation.states.some(state => state.disabled && state.text === 'Generating...'));
      assert.equal(dom.window.document.getElementById('share-link').value, '');
      assert.equal(dom.window.getComputedStyle(dom.window.document.getElementById('result-area')).display, 'none');

      observation.disconnect();
      const count = observation.states.length;
      button.disabled = true;
      await nextTurn();
      assert.equal(observation.states.length, count, 'disposal stops observing');
    } finally {
      observation?.disconnect();
      dom.window.close();
    }
  });
}
