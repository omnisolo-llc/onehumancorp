import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { builderDraftKey } from './ownedDraft';
import { initializeBuilderDraft, useBuilderStore, LEGACY_BUILDER_DRAFT_KEY, builderDraftError } from './store';
import { installBuilderLocks } from './testLocks';
let owner = { userId: 'a', tenantId: 'ta' };
beforeEach(() => {
  localStorage.clear(); notifyQueueIdentityChange(); installBuilderLocks(); owner = { userId: 'a', tenantId: 'ta' };
  vi.stubGlobal('fetch', vi.fn(async () => Response.json({ ...owner, expiresAt: Date.now() + 60_000 })));
});
afterEach(() => { notifyQueueIdentityChange(); vi.unstubAllGlobals(); vi.restoreAllMocks(); });

it('restores each owner draft without exposing or overwriting the other owner', async () => {
  const a = await initializeBuilderDraft();
  useBuilderStore.getState().setBio('Private A');
  const keyA = builderDraftKey(LEGACY_BUILDER_DRAFT_KEY, a.scope); const savedA = localStorage.getItem(keyA);
  owner = { userId: 'b', tenantId: 'tb' }; notifyQueueIdentityChange();
  const b = await initializeBuilderDraft(); expect(useBuilderStore.getState().bio).toBe('');
  useBuilderStore.getState().setBio('Private B');
  expect(localStorage.getItem(keyA)).toBe(savedA); b.release();
  owner = { userId: 'a', tenantId: 'ta' }; notifyQueueIdentityChange();
  const returned = await initializeBuilderDraft(); expect(useBuilderStore.getState().bio).toBe('Private A'); returned.release();
});
it('retains an unowned cache unchanged and never uses its claimed live URL', async () => {
  const held = JSON.stringify({ state: { bio: 'Unowned bytes', status: 'live', liveUrl: 'https://invented.invalid' }, version: 2 });
  localStorage.setItem('builder-storage', held);
  const editor = await initializeBuilderDraft();
  expect(useBuilderStore.getState().bio).toBe(''); expect(useBuilderStore.getState().liveUrl).toBe('');
  useBuilderStore.getState().setBio('New owned draft');
  expect(localStorage.getItem('builder-storage')).toBe(held); editor.release();
});
it('holds malformed owned state without overwriting its original bytes', async () => {
  const editor = await initializeBuilderDraft(); const key = builderDraftKey(LEGACY_BUILDER_DRAFT_KEY, editor.scope); editor.release();
  const original = JSON.stringify({ format: 1, revision: 'bad', data: { state: { blocks: 'invalid' }, version: 3 } });
  localStorage.setItem(key, original);
  await expect(initializeBuilderDraft()).rejects.toThrow(/remains held/);
  expect(localStorage.getItem(key)).toBe(original);
});
it('keeps the previous saved copy when storage rejects a new edit', async () => {
  const editor = await initializeBuilderDraft(); useBuilderStore.getState().setBio('Saved content');
  const key = builderDraftKey(LEGACY_BUILDER_DRAFT_KEY, editor.scope); const before = localStorage.getItem(key);
  const original = localStorage.setItem.bind(localStorage);
  const blocked = vi.spyOn(localStorage, 'setItem').mockImplementation((name, value) => { if (name === key) throw new Error('quota'); original(name, value); });
  useBuilderStore.getState().setBio('Unsaved content');
  expect(builderDraftError()).toMatch(/could not save|previous saved copy/i);
  expect(localStorage.getItem(key)).toBe(before); blocked.mockRestore(); editor.release();
});
it('never persists a local live claim or public URL as a publication receipt', async () => {
  const editor = await initializeBuilderDraft();
  useBuilderStore.getState().setBlocks([{ type: 'Hero', props: { headline: 'Private draft' } }]);
  useBuilderStore.getState().setStatus('live'); useBuilderStore.getState().setLiveUrl('https://invented.invalid');
  const raw = localStorage.getItem(builderDraftKey(LEGACY_BUILDER_DRAFT_KEY, editor.scope))!;
  expect(JSON.parse(raw).data.state.status).toBe('draft'); expect(raw).not.toContain('invented.invalid');
  editor.release(); const reopened = await initializeBuilderDraft();
  expect(useBuilderStore.getState().status).toBe('draft'); expect(useBuilderStore.getState().liveUrl).toBe(''); reopened.release();
});
