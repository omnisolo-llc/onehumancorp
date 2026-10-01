import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { JSDOM, VirtualConsole } from 'jsdom';
const roots = ['src/ui/tauri/src/ui', 'src/ui/next/public', 'src/ui/next/public/ui', 'src/ui/next/public/api/ui', 'src/ui/next/public/api/v1/ui'];
const tick = () => new Promise(resolve => setTimeout(resolve, 15));
async function load(root, fetchOverride) {
  const html = await readFile(new URL(`../${root}/help.html`, import.meta.url), 'utf8');
  const errors = [];
  const virtualConsole = new VirtualConsole();
  virtualConsole.on('jsdomError', error => errors.push(error));
  const dom = new JSDOM(html, { url: 'https://workspace.example/api/v1/ui/help.html', runScripts: 'dangerously', pretendToBeVisual: true, virtualConsole, beforeParse(window) {
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
