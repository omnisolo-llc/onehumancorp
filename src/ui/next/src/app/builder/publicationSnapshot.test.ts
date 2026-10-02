import { expect, it } from 'vitest';
import * as contracts from './publicationContracts';

const snapshot = () => ({ domain: null, pages: [{ path: '/', title: 'Café owner review', seo_metadata: { description: 'An actual draft\nfor review' }, blocks: [{ block_type: 'HeroBlock', content: { headline: 'Welcome' }, sort_order: 0 }] }] });
const prepare = contracts.prepareSiteSnapshot;
it('freezes the exact reviewed fields independently of later private edits', async () => {
  const draft = snapshot(); const reviewed = await prepare(draft);
  draft.pages[0].blocks[0].content.headline = 'Edited later';
  expect(reviewed.snapshot.pages[0].blocks[0].content.headline).toBe('Welcome');
  expect(reviewed.canonical).toContain('An actual draft\\nfor review');
  expect(reviewed.snapshot_sha256).toMatch(/^[0-9a-f]{64}$/);
  expect(reviewed.snapshot_encoding).toBe('jcs-rfc8785-v1');
  expect(() => { reviewed.snapshot.pages[0].blocks[0].content.headline = 'Mutated approval'; }).toThrow();
});
it('uses RFC8785 numeric serialization and UTF16 key ordering without numeric-key reordering', async () => {
  const draft = snapshot();
  const content = { '2': 2, '10': 10, '\uE000': 'BMP', '😀': 'supplementary', numbers: [Number('333333333.33333329'), 1e15, 4.50, 2e-3, 1e-27, -0] };
  const reviewed = await prepare({ ...draft, pages: [{ ...draft.pages[0], blocks: [{ ...draft.pages[0].blocks[0], content }] }] });
  expect(reviewed.canonical).toContain('"content":{"10":10,"2":2,"numbers":[333333333.3333333,1000000000000000,4.5,0.002,1e-27,0],"😀":"supplementary","\uE000":"BMP"}');
});
it('preserves normalization-sensitive Unicode bytes in the approval digest', async () => {
  const composed = snapshot(); const decomposed = snapshot(); decomposed.pages[0].title = composed.pages[0].title.normalize('NFD');
  expect((await prepare(composed)).snapshot_sha256).not.toBe((await prepare(decomposed)).snapshot_sha256);
});
it.each([
  { domain: 'unverified.example' }, { tenant_id: 'injected' }, { pages: [] },
  { pages: [{ path: '/missing-root', title: 'Home', seo_metadata: {}, blocks: [] }] },
  { pages: [{ path: '/', title: ' ', seo_metadata: {}, blocks: [] }] },
  { pages: [{ path: '/', title: 'Home', seo_metadata: [], blocks: [] }] },
  { pages: [{ path: '/', title: 'Home', seo_metadata: {}, blocks: [{ block_type: 'HeroBlock', content: {}, sort_order: 0 }, { block_type: 'HeroBlock', content: {}, sort_order: 0 }] }] },
])('rejects invalid or unreviewed snapshot fields %#', change => {
  return expect(prepare({ ...snapshot(), ...change })).rejects.toThrow();
});
it.each(['//outside.example', '/../../secret', '/encoded%2fpath', '/?query=1', '/bad\npath'])('rejects nonlocal document path %s', path => {
  const draft = snapshot(); draft.pages[0].path = path;
  return expect(prepare(draft)).rejects.toThrow();
});
it.each([NaN, Infinity, -Infinity, 1e30, -1e30, undefined, () => 'hidden', '\ud800', 'nul\0byte'])('rejects non-JSON or invalid Unicode content %# rather than silently dropping it', unsafe => {
  const draft = snapshot();
  return expect(prepare({ ...draft, pages: [{ ...draft.pages[0], blocks: [{ ...draft.pages[0].blocks[0], content: { unsafe } }] }] })).rejects.toThrow();
});
it('rejects invalid Unicode keys and circular content before approval', async () => {
  const draft = snapshot(); const circular: Record<string, unknown> = {}; circular.self = circular;
  for (const content of [{ '\ud800': 'invalid key' }, circular]) {
    await expect(prepare({ ...draft, pages: [{ ...draft.pages[0], blocks: [{ ...draft.pages[0].blocks[0], content }] }] })).rejects.toThrow();
  }
});
it('enforces the actual byte/page/block bounds without truncating reviewed content', async () => {
  const draft = snapshot();
  await expect(prepare({ ...draft, pages: Array.from({ length: 51 }, (_, i) => ({ ...draft.pages[0], path: i ? '/' + i : '/' })) })).rejects.toThrow();
  await expect(prepare({ ...draft, pages: [{ ...draft.pages[0], blocks: Array.from({ length: 101 }, (_, i) => ({ ...draft.pages[0].blocks[0], sort_order: i })) }] })).rejects.toThrow();
  await expect(prepare({ ...draft, pages: [{ ...draft.pages[0], seo_metadata: { text: '😀'.repeat(270000) } }] })).rejects.toThrow();
});
it.each(['/nested//page', '/nested/'])('rejects empty document path segments %s', path => {
  const draft = snapshot(); draft.pages.push({ ...draft.pages[0], path });
  return expect(prepare(draft)).rejects.toThrow();
});
it('bounds snapshot containers so its publication request fits the 32-level wire limit', async () => {
  const draft = snapshot();
  let content: Record<string, unknown> = { value: 'leaf' };
  for (let i = 0; i < 32; i++) content = { nested: content };
  await expect(prepare({ ...draft, pages: [{ ...draft.pages[0], blocks: [{ ...draft.pages[0].blocks[0], content }] }] })).rejects.toThrow();
});
