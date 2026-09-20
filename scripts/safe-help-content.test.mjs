import test from 'node:test';
import assert from 'node:assert/strict';
import { JSDOM } from 'jsdom';
import { readFile } from 'node:fs/promises';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { renderHelpMessage, renderHelpVideos, renderWalkthroughStep } from '../src/ui/tauri/src/ui/safe-help-content.mjs';

function fixture() {
  const dom = new JSDOM('<main id="content"></main>', { url: 'https://workspace.example/' });
  return { dom, node: dom.window.document.getElementById('content') };
}
test('help messages and article titles remain inert text', () => {
  const { dom, node } = fixture();
  try {
    renderHelpMessage(node, '<img src=x onerror=alert(1)>', { url: '/help/setup', title: '<script>bad()</script>' });
    assert.equal(node.querySelector('img,script'), null);
    assert.match(node.textContent, /<img/);
    assert.equal(node.querySelector('a').href, 'https://workspace.example/help/setup');
    assert.equal(node.querySelector('a').textContent, 'Read the full article: <script>bad()</script> →');
  } finally { dom.window.close(); }
});
test('help links reject script, data, credentials and insecure cross-origin URLs', () => {
  const { dom, node } = fixture();
  try {
    for (const url of ['javascript:alert(1)', 'data:text/html,bad', 'https://user:pass@other.example/', 'http://other.example/']) {
      renderHelpMessage(node, 'Text', { url, title: 'Open' });
      assert.equal(node.querySelector('a'), null, url);
    }
  } finally { dom.window.close(); }
});
test('video metadata cannot inject HTML or executable inline handlers', () => {
  const { dom, node } = fixture();
  try {
    renderHelpVideos(node, [{ title: '<img onerror=bad()>', duration: '<script>bad()</script>', video_url: 'https://cdn.example/video.mp4' }]);
    assert.equal(node.querySelector('img,script,[onclick]'), null);
    const button = node.querySelector('button');
    assert.equal(button.textContent, 'Play <img onerror=bad()>');
    button.click();
    assert.equal(node.querySelector('video').src, 'https://cdn.example/video.mp4');
    assert.equal(node.querySelector('video').controls, true);
    assert.equal(node.querySelector('h4').textContent, '<img onerror=bad()>');
  } finally { dom.window.close(); }
});
test('invalid video collections fail explicitly and unsafe media are not rendered', () => {
  const { dom, node } = fixture();
  try {
    assert.throws(() => renderHelpVideos(node, { videos: [] }), TypeError);
    renderHelpVideos(node, [null, { title: 'Unsafe', video_url: 'javascript:bad()' }]);
    assert.equal(node.children.length, 0);
  } finally { dom.window.close(); }
});
test('walkthrough metadata is text and controls remain scoped and functional', () => {
  const { dom, node } = fixture();
  try {
    const duplicate = dom.window.document.createElement('button');
    duplicate.id = 'wt-next'; dom.window.document.body.prepend(duplicate);
    const controls = renderWalkthroughStep(node, {
      title: '<img src=x onerror=bad()>', content: '<script>bad()</script>',
    }, 0, 2);
    assert.equal(node.querySelector('img,script,[onclick]'), null);
    assert.equal(node.querySelector('h4').textContent, '<img src=x onerror=bad()>');
    assert.equal(node.querySelector('p').textContent, '<script>bad()</script>');
    assert.equal(controls.previous, null);
    assert.equal(controls.next.textContent, 'Next');
    assert.notEqual(controls.next, duplicate);
    let advanced = false; controls.next.onclick = () => { advanced = true; };
    controls.next.click(); assert.equal(advanced, true);
    const final = renderWalkthroughStep(node, { text: 'Last step' }, 1, 2);
    assert.equal(final.previous.textContent, 'Back');
    assert.equal(final.next.textContent, 'Finish');
    assert.equal(final.close.getAttribute('aria-label'), 'Close walkthrough');
    assert.equal(node.querySelector('p').textContent, 'Last step');
    assert.throws(() => renderWalkthroughStep(node, null, 0, 2), TypeError);
    assert.throws(() => renderWalkthroughStep(node, {}, 2, 2), TypeError);
  } finally { dom.window.close(); }
});

test('legacy help and voice scripts parse, and voice does not manufacture tenant identity', async () => {
  for (const name of ['help-widget.mjs', 'voice-assistant.mjs']) {
    const url = new URL(`../src/ui/tauri/src/ui/${name}`, import.meta.url);
    const result = spawnSync(process.execPath, ['--check', fileURLToPath(url)], { encoding: 'utf8', timeout: 10000 });
    assert.ifError(result.error); assert.equal(result.status, 0, result.stderr);
  }
  const source = await readFile(new URL('../src/ui/tauri/src/ui/voice-assistant.mjs', import.meta.url), 'utf8');
  assert.doesNotMatch(source, /x-spiffe-id|localStorage\.getItem\(['"]tenant/);
  assert.match(source, /typeof data\.transcription !== 'string'/);
});
