import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { JSDOM, VirtualConsole } from './test-support/offline-dom.mjs';
import { renderWalkthroughStep } from '../src/ui/tauri/src/ui/safe-help-content.mjs';
const roots = ['src/ui/tauri/src/ui', 'src/ui/next/public', 'src/ui/next/public/ui', 'src/ui/next/public/api/ui', 'src/ui/next/public/api/v1/ui'];
const tick = () => new Promise(resolve => setTimeout(resolve, 15));
async function load(root, fetchOverride) {
  const html = await readFile(new URL(`../${root}/help.html`, import.meta.url), 'utf8');
  const errors = [];
  const virtualConsole = new VirtualConsole();
  virtualConsole.on('jsdomError', error => errors.push(error));
  const dom = new JSDOM(html, { url: 'https://workspace.example/api/v1/ui/help.html', runScripts: 'dangerously', pretendToBeVisual: true, virtualConsole, beforeParse(window) {
    // JSDOM does not load modules; use the genuine module named by Tauri HTML.
    if (root === 'src/ui/tauri/src/ui') window.renderWalkthroughStep = renderWalkthroughStep;
    window.HTMLMediaElement.prototype.play = async () => {};
    window.HTMLMediaElement.prototype.pause = () => {};
    window.fetch = fetchOverride ?? (async url => ({ ok: true, json: async () => url.includes('/videos') ? [{ id: 1, title: 'Recorded tutorial', duration: '1:00', video_url: 'https://cdn.example/tutorial.mp4' }] : url.includes('/help') ? [{ title: 'Recorded article', desc: 'Article summary', category: 'Guides', link: '/help/recorded' }] : {} }));
  } });
  await new Promise(resolve => dom.window.addEventListener('load', resolve, { once: true }));
  await tick(); dom.testErrors = errors; return dom;
}
for (const root of roots) {
  test(`${root}: the Help widget starts hidden and opens as a positioned overlay`, async () => {
    const dom = await load(root);
    try {
      const doc = dom.window.document;
      const widget = doc.getElementById('ohc-floating-help-widget');
      assert.ok(widget);
      assert.equal(dom.window.getComputedStyle(widget).position, 'fixed');
      assert.equal(dom.window.getComputedStyle(widget).display, 'none');
      doc.getElementById('ohc-floating-help-btn').click();
      assert.equal(dom.window.getComputedStyle(widget).display, 'flex');
      widget.querySelector('#omnisolo-floating-help-close').click();
      assert.equal(dom.window.getComputedStyle(widget).display, 'none');
    } finally { dom.window.close(); assert.deepEqual(dom.testErrors, []); }
  });
  test(`${root}: Help retains an actionable release-notes tab`, async () => {
    const dom = await load(root);
    try {
      const widget = dom.window.document.querySelector('#ohc-floating-help-widget, #omnisolo-floating-help-widget');
      const tab = widget.querySelector('[data-target="tab-changelog"]');
      assert.ok(tab); tab.click();
      const content = widget.querySelector('#tab-changelog');
      assert.ok(content.classList.contains('active'));
      assert.equal(new URL(content.querySelector('a').href).pathname, '/api/v1/ui/changelog.html');
    } finally { dom.window.close(); assert.deepEqual(dom.testErrors, []); }
  });
  test(`${root}: widget tutorials open the real shared video player`, async () => {
    const dom = await load(root);
    try {
      const doc = dom.window.document;
      const widget = doc.querySelector('#ohc-floating-help-widget, #omnisolo-floating-help-widget');
      widget.querySelector('[data-target="tab-videos"]').click(); await tick();
      const button = widget.querySelector('#video-list button');
      assert.ok(button, 'The tutorial must be an actionable video control'); button.click();
      assert.equal(doc.getElementById('video-player').src, 'https://cdn.example/tutorial.mp4');
      assert.equal(doc.getElementById('video-modal').style.display, 'flex');
      doc.getElementById('close-video').click();
      assert.equal(doc.getElementById('video-modal').style.display, 'none');
    } finally { dom.window.close(); assert.deepEqual(dom.testErrors, []); }
  });
  test(`${root}: video dialog stays above Help and restores keyboard focus and scroll`, async () => {
    const dom = await load(root);
    try {
      const doc = dom.window.document;
      const launcher = doc.getElementById('ohc-floating-help-btn');
      launcher.click();
      const widget = doc.getElementById('ohc-floating-help-widget');
      widget.querySelector('[data-target="tab-videos"]').click(); await tick();
      const trigger = widget.querySelector('#video-list button');
      doc.body.style.overflow = 'auto';
      trigger.focus(); trigger.click();
      const modal = doc.getElementById('video-modal');
      const close = doc.getElementById('close-video');
      const player = doc.getElementById('video-player');
      assert.ok(Number(dom.window.getComputedStyle(modal).zIndex) > Number(dom.window.getComputedStyle(widget).zIndex));
      assert.ok(Number(dom.window.getComputedStyle(modal).zIndex) > Number(dom.window.getComputedStyle(launcher).zIndex));
      assert.equal(modal.getAttribute('role'), 'dialog');
      assert.equal(modal.getAttribute('aria-modal'), 'true');
      assert.equal(doc.activeElement, close);
      assert.equal(doc.body.style.overflow, 'hidden');
      player.focus();
      const nativeTab = new dom.window.KeyboardEvent('keydown', { key: 'Tab', bubbles: true, cancelable: true });
      player.dispatchEvent(nativeTab);
      assert.equal(nativeTab.defaultPrevented, false, 'Leave native media-controls Tab navigation to the browser');
      const start = modal.querySelector('[data-video-focus-boundary="start"]');
      const end = modal.querySelector('[data-video-focus-boundary="end"]');
      assert.ok(start); assert.ok(end);
      end.focus(); assert.equal(doc.activeElement, close);
      start.focus(); assert.equal(doc.activeElement, player);
      player.dispatchEvent(new dom.window.KeyboardEvent('keydown', { key: 'Escape', bubbles: true, cancelable: true }));
      assert.equal(modal.style.display, 'none');
      assert.equal(doc.activeElement, trigger);
      assert.equal(doc.body.style.overflow, 'auto');
      trigger.click();
      close.click();
      assert.equal(modal.style.display, 'none');
      assert.equal(doc.body.style.overflow, 'auto');
      trigger.click();
      modal.click();
      assert.equal(modal.style.display, 'none');
      assert.equal(doc.activeElement, trigger);
    } finally { dom.window.close(); assert.deepEqual(dom.testErrors, []); }
  });
  test(`${root}: the final walkthrough control exposes and completes Finish`, async () => {
    const dom = await load(root);
    try {
      const doc = dom.window.document;
      doc.getElementById('ohc-floating-help-btn').click();
      const widget = doc.getElementById('ohc-floating-help-widget');
      widget.querySelector('[data-target="tab-tours"]').click();
      widget.querySelector('.omnisolo-tour-card').click();
      const bubble = doc.getElementById('walkthrough-bubble');
      assert.ok(bubble);
      const finish = bubble.querySelector('#wt-next');
      assert.equal(finish.textContent, 'Finish');
      assert.equal(finish.getAttribute('aria-label') || finish.textContent, 'Finish');
      finish.click();
      assert.equal(doc.getElementById('walkthrough-bubble'), null);
      assert.equal(doc.getElementById('walkthrough-overlay'), null);
    } finally { dom.window.close(); assert.deepEqual(dom.testErrors, []); }
  });
  test(`${root}: actual chat links resolve legacy article IDs to maintained documents`, async () => {
    let responseLink = '';
    const dom = await load(root, async url => ({ ok: true, json: async () => url.includes('/chat') ? { reply: 'Recorded article response', link: { title: 'Open document', url: responseLink } } : [] }));
    try {
      const doc = dom.window.document;
      doc.getElementById('ohc-floating-help-btn').click();
      const widget = doc.getElementById('ohc-floating-help-widget');
      widget.querySelector('[data-target="tab-chat"]').click();
      for (const [url, path] of [
        ['/help?article=my-store-1', '/help/add-products'],
        ['/help?article=payments-1', '/help/accept-payments'],
        ['/help?article=marketing-tools', '/help/marketing-tools'],
        ['/help?article=../settings', null],
      ]) {
        responseLink = url;
        const input = widget.querySelector('#ohc-help-chat-input');
        input.value = 'How do I add a product?'; input.dispatchEvent(new dom.window.Event('input'));
        widget.querySelector('#ohc-help-chat-send').click(); await tick();
        const message = widget.querySelector('#ohc-help-chat-messages, #omnisolo-help-chat-messages').lastElementChild;
        assert.match(message.textContent, /Recorded article response/);
        const link = message.querySelector('a');
        if (path) {
          assert.ok(link); assert.equal(link.href, `https://workspace.example${path}`);
        } else assert.equal(link, null);
      }
    } finally { dom.window.close(); assert.deepEqual(dom.testErrors, []); }
  });
  test(`${root}: initial Help data preserves the current query and rejects an older search response`, async () => {
    const currentArticle = { title: 'Current product guide', desc: 'Matching guide', category: 'Guides', link: '/help/current' };
    const unrelatedArticle = { title: 'Unrelated invoice guide', desc: 'Another guide', category: 'Guides', link: '/help/unrelated' };
    const response = value => ({ ok: true, json: async () => value });
    let resolveArticles;
    let resolveVideos;
    let resolveOlderSearch;
    const dom = await load(root, url => {
      if (url === '/api/v1/help') return new Promise(resolve => { resolveArticles = resolve; });
      if (url === '/api/v1/videos') return new Promise(resolve => { resolveVideos = resolve; });
      if (url === '/api/v1/help/search?q=older') return new Promise(resolve => { resolveOlderSearch = resolve; });
      if (url === '/api/v1/help/search?q=product') return Promise.resolve(response([currentArticle]));
      return Promise.resolve(response([]));
    });
    try {
      const doc = dom.window.document;
      const input = doc.getElementById('search-input');
      const results = doc.getElementById('results');
      assert.equal(doc.getElementById('loading-state').style.display, 'block');
      input.value = 'older'; input.dispatchEvent(new dom.window.Event('input'));
      input.value = 'product'; input.dispatchEvent(new dom.window.Event('input'));
      await tick();
      assert.match(results.textContent, /Current product guide/);

      resolveArticles(response([currentArticle, unrelatedArticle]));
      resolveVideos(response([
        { title: 'Product tutorial', duration: '1:00', video_url: 'https://cdn.example/product.mp4' },
        { title: 'Unrelated invoice tutorial', duration: '2:00', video_url: 'https://cdn.example/invoice.mp4' },
      ]));
      await tick();
      assert.equal(doc.getElementById('loading-state').style.display, 'none');
      assert.equal(input.value, 'product');
      assert.match(results.textContent, /Current product guide/);
      assert.match(results.textContent, /Product tutorial/);
      assert.doesNotMatch(results.textContent, /Unrelated invoice/);

      resolveOlderSearch(response([{ title: 'Stale older guide', desc: 'Old result', link: '/help/stale' }]));
      await tick();
      assert.equal(input.value, 'product');
      assert.match(results.textContent, /Current product guide/);
      assert.doesNotMatch(results.textContent, /Stale older guide|Unrelated invoice/);
    } finally { dom.window.close(); assert.deepEqual(dom.testErrors, []); }
  });
  test(`${root}: late search results cannot replace a cleared search`, async () => {
    let resolveSearch;
    const dom = await load(root, async url => {
      if (url.includes('/help/search')) return new Promise(resolve => { resolveSearch = resolve; });
      return { ok: true, json: async () => url.includes('/help') ? [{ title: 'Current article', desc: 'Summary', category: 'Guides', link: '/help/current' }] : [] };
    });
    try {
      const input = dom.window.document.querySelector('input[placeholder="Search for help articles and videos..."]');
      input.value = 'older'; input.dispatchEvent(new dom.window.Event('input'));
      input.value = ''; input.dispatchEvent(new dom.window.Event('input'));
      resolveSearch({ ok: true, json: async () => [{ title: 'Stale article', desc: 'Old', link: '/help/stale' }] }); await tick();
      assert.match(dom.window.document.body.textContent, /Current article/);
      assert.doesNotMatch(dom.window.document.body.textContent, /Stale article/);
    } finally { dom.window.close(); assert.deepEqual(dom.testErrors, []); }
  });
  test(`${root}: Help renders remote metadata and typed chat as inert text`, async () => {
    const attack = '<img src=x onerror="window.injection=true">';
    const dom = await load(root, async url => ({ ok: true, json: async () => url.includes('/videos') ? [{ id: 1, title: attack, duration: attack, video_url: 'javascript:alert(1)' }] : url.includes('/help') ? [{ title: attack, desc: attack, category: attack, link: '/help/recorded' }] : { reply: attack } }));
    try {
      const doc = dom.window.document;
      assert.equal(doc.querySelector('img[src="x"]'), null);
      const widget = doc.querySelector('#ohc-floating-help-widget, #omnisolo-floating-help-widget');
      widget.querySelector('[data-target="tab-chat"]').click();
      const input = widget.querySelector('#ohc-help-chat-input'); input.value = attack; input.dispatchEvent(new dom.window.Event('input'));
      widget.querySelector('#ohc-help-chat-send').click(); await tick();
      assert.equal(widget.querySelector('img,script'), null);
      assert.match(widget.textContent, /<img src=x/);
      widget.querySelector('[data-target="tab-videos"]').click(); await tick();
      assert.equal(widget.querySelector('[src^="javascript:"],[onclick*="alert"]'), null);
    } finally { dom.window.close(); assert.deepEqual(dom.testErrors, []); }
  });
}
