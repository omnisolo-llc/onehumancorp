import { afterEach, beforeEach, expect, it, vi } from 'vitest';
import { notifyQueueIdentityChange } from '@/lib/sync/queueIdentity';
import { openBuilderEditor, openBuilderScope, readBuilderDraft, releaseBuilderEditor, writeBuilderDraft } from '../builder/ownedDraft';
import { installBuilderLocks } from '../builder/testLocks';
const key = 'agent-publication-draft';
const owner = { userId: 'owner-a', tenantId: 'tenant-a' };
beforeEach(() => { localStorage.clear(); notifyQueueIdentityChange(); installBuilderLocks(); vi.stubGlobal('fetch', vi.fn(async () => Response.json({ ...owner, expiresAt: Date.now() + 60000 }))); });
afterEach(() => { notifyQueueIdentityChange(); vi.unstubAllGlobals(); });
it('requires the shared editor lease before changing a publication draft', async () => {
  const scope = await openBuilderScope();
  expect(() => writeBuilderDraft(key, { name: 'Uncoordinated edit' }, scope)).toThrow();
  expect(readBuilderDraft(key, scope)).toBeNull();
});
it('holds a second editor and reloads the latest owned draft after the first closes', async () => {
  const first = await openBuilderEditor(key);
  try {
    await expect(openBuilderEditor(key)).rejects.toThrow('another view');
    writeBuilderDraft(key, { name: 'Latest reviewed edit' }, first);
  } finally { releaseBuilderEditor(first); }
  await new Promise(resolve => setTimeout(resolve, 0));
  const next = await openBuilderEditor(key);
  try {
    expect(readBuilderDraft(key, next)?.data).toEqual({ name: 'Latest reviewed edit' });
    expect(() => writeBuilderDraft(key, { name: 'Late old edit' }, first)).toThrow();
  } finally { releaseBuilderEditor(next); }
});
it('does not acquire an editor after its mounting view retires during verification', async () => {
  let release!: (response: Response) => void; let active = true;
  vi.mocked(fetch).mockImplementationOnce(() => new Promise(resolve => { release = resolve; }));
  const pending = openBuilderEditor(key, () => active);
  active = false; release(Response.json({ ...owner, expiresAt: Date.now() + 60000 }));
  const result = await pending.then(scope => { releaseBuilderEditor(scope); return null; }, error => error);
  expect(result).toBeInstanceOf(Error);
});
