import { fetchForOwnedBusinessAction, openOnboardingSession, onboardingOwner, onboardingSessionEpoch, ownedOnboardingKey, readOwnedOnboardingItem, writeOwnedOnboardingItem, subscribeOnboardingInvalidation } from '../onboarding/draftSession';
import { onboardingWriteVersion } from '../onboarding/draftWriteGate';
import { sameOwner, type QueueOwner } from '@/lib/sync/queueIdentity';
import { canonicalRequest } from '../onboarding/contracts';

type EditorLease = { name: string; active: boolean; raw: string | null; release: () => void };
export type BuilderScope = { owner: QueueOwner; epoch: number; editor?: EditorLease };
const editorDrafts = new Set(['website-builder-draft', 'storefront-builder-draft']);
export async function openBuilderEditor(name: string): Promise<BuilderScope> {
  const scope = await openBuilderScope();
  if (!navigator.locks?.request) throw new Error('Builder editing is unavailable because this browser cannot coordinate safe local drafts. Your saved draft remains held.');
  const key = builderDraftKey(name, scope);
  return new Promise<BuilderScope>((resolve, reject) => {
    void navigator.locks.request(key + ':editor', { mode: 'exclusive', ifAvailable: true }, async lock => {
      if (!lock) throw new Error('This builder draft is open in another view. Close that editor, then reopen this page to load its latest saved edits.');
      assertBuilderScope(scope);
      let finish!: () => void;
      const held = new Promise<void>(done => { finish = done; });
      let unsubscribe = () => {};
      const editor: EditorLease = { name, active: true, raw: localStorage.getItem(key), release: () => {
        if (!editor.active) return;
        editor.active = false; unsubscribe(); finish();
      } };
      scope.editor = editor;
      unsubscribe = subscribeOnboardingInvalidation(() => editor.release());
      resolve(scope);
      await held;
    }).catch(reject);
  });
}
export function releaseBuilderEditor(scope: BuilderScope | null): void { scope?.editor?.release(); }
export function assertBuilderEditor(scope: BuilderScope, name: string): void {
  assertBuilderScope(scope);
  if (!scope.editor || scope.editor.name !== name) throw new Error('Open the builder editor before changing this draft.');
  if (scope.editor.raw !== readOwnedOnboardingItem(name)) throw new Error('Your draft changed in another view. Reopen the builder to restore the latest local draft.');
}
type CacheRecord<T> = { format: 1; revision: string; data: T };
export async function openBuilderScope(): Promise<BuilderScope> {
  const owner = await openOnboardingSession();
  return { owner, epoch: onboardingSessionEpoch() };
}
export function builderScopeActive(scope: BuilderScope | null): scope is BuilderScope {
  const current = onboardingOwner();
  return !!scope && (!scope.editor || scope.editor.active) && !!current && scope.epoch === onboardingSessionEpoch() && sameOwner(scope.owner, current);
}
export function assertBuilderScope(scope: BuilderScope | null): asserts scope is BuilderScope {
  if (!builderScopeActive(scope)) throw new Error('Your session changed. Reopen the builder to restore your draft.');
}
export function readBuilderDraft<T>(name: string, scope: BuilderScope): CacheRecord<T> | null {
  assertBuilderScope(scope);
  const raw = readOwnedOnboardingItem(name);
  if (raw === null) return null;
  let value: CacheRecord<T>;
  try { value = JSON.parse(raw) as CacheRecord<T>; } catch { throw new Error('This saved builder draft could not be read. Its original copy remains held.'); }
  if (!value || value.format !== 1 || typeof value.revision !== 'string' || !value.revision || !value.data || typeof value.data !== 'object' || Array.isArray(value.data)) throw new Error('This saved builder draft could not be read. Its original copy remains held.');
  return value;
}
export function writeBuilderDraft<T extends object>(name: string, data: T, scope: BuilderScope): void {
  assertBuilderScope(scope);
  if (editorDrafts.has(name)) assertBuilderEditor(scope, name);
  const previous = readBuilderDraft<T>(name, scope);
  if (previous && canonicalRequest(previous.data) === canonicalRequest(data)) return;
  try {
    const raw = JSON.stringify({ format: 1, revision: crypto.randomUUID(), data });
    writeOwnedOnboardingItem(name, raw);
    if (scope.editor?.name === name) scope.editor.raw = raw;
  }
  catch { throw new Error('This device could not save your latest builder edits. The previous saved copy remains held.'); }
}
export function captureBuilderRestore(name: string, scope: BuilderScope) {
  assertBuilderScope(scope);
  return { name, scope, raw: readOwnedOnboardingItem(name), writeVersion: onboardingWriteVersion(scope.owner) };
}
export function assertBuilderRestore(snapshot: ReturnType<typeof captureBuilderRestore>): void {
  assertBuilderScope(snapshot.scope);
  if (snapshot.raw !== readOwnedOnboardingItem(snapshot.name) || snapshot.writeVersion !== onboardingWriteVersion(snapshot.scope.owner)) throw new Error('Your draft changed in another view. Reopen the builder to restore the latest local draft.');
}
export function builderDraftKey(name: string, scope: BuilderScope): string {
  assertBuilderScope(scope); return ownedOnboardingKey(name)!;
}
export function hasHeldBuilderLegacy(): boolean {
  return ['website-builder-storage', 'omnisolo_builder_bio', 'omnisolo_builder_blocks', 'omnisolo_builder_status', 'omnisolo_builder_liveUrl'].some(key => localStorage.getItem(key) !== null);
}

export async function publishOwnedLayout(scope: BuilderScope, payload: object): Promise<{ id: string; domain: string | null }> {
  assertBuilderScope(scope);
  if (!navigator.locks?.request) throw new Error('Site saving is unavailable because this browser cannot coordinate safe requests. Your local draft remains held.');
  const key = builderDraftKey('builder-publish-fence', scope);
  const body = JSON.stringify(payload);
  const fingerprint = canonicalRequest(JSON.parse(body));
  return navigator.locks.request(key, { mode: 'exclusive' }, async () => {
    assertBuilderScope(scope);
    if (localStorage.getItem(key) !== null) throw new Error('A previous site save could not be confirmed. Review your saved sites before trying again.');
    const previous = readBuilderDraft<{ fingerprint: string; site: { id: string; domain: string | null } }>('builder-publish-receipt', scope);
    if (previous?.data.fingerprint === fingerprint) throw new Error('This layout has a recorded save. Review your saved sites before resubmitting it.');
    let marker: string | null = null;
    const response = await fetchForOwnedBusinessAction('/api/v1/builder/publish_draft', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body }, scope.owner, () => {
      assertBuilderScope(scope);
      marker = JSON.stringify({ operation: crypto.randomUUID(), body }); localStorage.setItem(key, marker);
    });
    if (!response.ok || response.status !== 200) {
      if (response.status >= 400 && response.status < 500 && marker !== null && localStorage.getItem(key) === marker) localStorage.removeItem(key);
      throw new Error('The site save was not acknowledged. Your local draft remains held.');
    }
    const data = await response.json();
    assertBuilderScope(scope);
    if (!data || data.error != null || data.success === false || typeof data.id !== 'string' || !/^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i.test(data.id) || data.domain !== null && typeof data.domain !== 'string') throw new Error('The site save could not be confirmed. Review your saved sites before trying again.');
    const site = { id: data.id, domain: data.domain };
    writeBuilderDraft('builder-publish-receipt', { fingerprint, site }, scope);
    if (marker === null || localStorage.getItem(key) !== marker) throw new Error('The site save needs reconciliation. Your local draft remains held.');
    localStorage.removeItem(key);
    return site;
  });
}
