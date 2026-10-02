import { create } from 'zustand';
import { createJSONStorage, persist } from 'zustand/middleware';
import { openBuilderEditor, releaseBuilderEditor, assertBuilderEditor, builderScopeActive, readBuilderDraft, writeBuilderDraft, type BuilderScope } from './ownedDraft';
import { subscribeOnboardingInvalidation } from '../onboarding/draftSession';
import { canonicalRequest } from '../onboarding/contracts';
import type { BuilderBlock } from '@/lib/builder-types';

export interface BuilderState {
  bio: string;
  businessName: string;
  businessCategory: string;
  vibe: string;
  wizardStep: number;
  blocks: BuilderBlock[];
  drafts: BuilderBlock[][];
  status: "onboarding" | "idle" | "generating" | "draft" | "selection" | "live";
  businessGoal: "products" | "services" | "work" | null;
  liveUrl: string;
  seoMetadata: Record<string, unknown>;

  setBio: (bio: string) => void;
  setBusinessName: (name: string) => void;
  setBusinessCategory: (category: string) => void;
  setVibe: (vibe: string) => void;
  setWizardStep: (step: number) => void;
  setBlocks: (blocks: BuilderBlock[]) => void;
  setDrafts: (drafts: BuilderBlock[][]) => void;
  setStatus: (status: "onboarding" | "idle" | "generating" | "draft" | "selection" | "live") => void;
  setBusinessGoal: (goal: "products" | "services" | "work" | null) => void;
  setSeoMetadata: (metadata: Record<string, unknown>) => void;
  setLiveUrl: (url: string) => void;
}

export const LEGACY_BUILDER_DRAFT_KEY = 'legacy-builder-draft';
let scope: BuilderScope | null = null;
let hydrating = false;
let hydrationEpoch = 0;
let lastValue: string | null = null;
let references = 0;
let initialising: Promise<void> | null = null;
let persistenceError = '';
const persistenceListeners = new Set<() => void>();
export function builderDraftError() { return persistenceError; }
export function subscribeBuilderPersistence(listener: () => void) { persistenceListeners.add(listener); return () => { persistenceListeners.delete(listener); }; }
function notifyPersistence() { for (const listener of persistenceListeners) listener(); }
const businessFields = ['bio','businessName','businessCategory','vibe','wizardStep','blocks','drafts','status','businessGoal','seoMetadata'] as const;
export function isBuilderBlocks(value: unknown): value is BuilderBlock[] {
  const scalar = (item: unknown) => typeof item === 'string' || typeof item === 'boolean' || typeof item === 'number' && Number.isFinite(item);
  return Array.isArray(value) && value.length <= 100 && value.every(block => block && typeof block === 'object' && !Array.isArray(block)
    && typeof block.type === 'string' && !!block.type.trim() && block.props && typeof block.props === 'object' && !Array.isArray(block.props)
    && Object.values(block.props).every(item => scalar(item) || Array.isArray(item) && item.every(entry => entry && typeof entry === 'object' && !Array.isArray(entry) && Object.values(entry).every(scalar))));
}
function projectDraft(value: Partial<BuilderState>) {
  const saved: Record<string, unknown> = {};
  for (const field of businessFields) if (value[field] !== undefined) saved[field] = value[field];
  if (['bio','businessName','businessCategory','vibe'].some(field => saved[field] !== undefined && typeof saved[field] !== 'string')
    || saved.wizardStep !== undefined && (!Number.isInteger(saved.wizardStep) || Number(saved.wizardStep) < 1 || Number(saved.wizardStep) > 3)
    || saved.blocks !== undefined && !isBuilderBlocks(saved.blocks)
    || saved.drafts !== undefined && (!Array.isArray(saved.drafts) || !saved.drafts.every(isBuilderBlocks))
    || saved.status !== undefined && !['onboarding','idle','generating','draft','selection','live'].includes(String(saved.status))
    || saved.businessGoal !== undefined && saved.businessGoal !== null && !['products','services','work'].includes(String(saved.businessGoal))
    || saved.seoMetadata !== undefined && (!saved.seoMetadata || typeof saved.seoMetadata !== 'object' || Array.isArray(saved.seoMetadata))) {
    throw new Error('This saved builder draft could not be read. Its original copy remains held.');
  }
  if (saved.status === 'live' || saved.status === 'generating') saved.status = Array.isArray(saved.blocks) && saved.blocks.length ? 'draft' : 'idle';
  return saved;
}
const storage = createJSONStorage<Record<string, unknown>>(() => ({
  getItem: () => {
    if (!scope) return null;
    const record = readBuilderDraft<{ state: Record<string, unknown>; version: number }>(LEGACY_BUILDER_DRAFT_KEY, scope);
    if (record && (record.data.version !== 3 || !record.data.state || typeof record.data.state !== 'object' || Array.isArray(record.data.state))) throw new Error('This saved builder draft could not be read. Its original copy remains held.');
    return record ? JSON.stringify(record.data) : null;
  },
  setItem: (_name: string, value: string) => {
    if (hydrating || !builderScopeActive(scope)) return;
    try {
      const data = JSON.parse(value);
      const fingerprint = canonicalRequest(data);
      if (fingerprint === lastValue) return;
      writeBuilderDraft(LEGACY_BUILDER_DRAFT_KEY, data, scope);
      lastValue = fingerprint; persistenceError = "";
    } catch (error) { persistenceError = error instanceof Error ? error.message : 'This device could not save your latest edits.'; }
    notifyPersistence();
  },
  removeItem: () => {},
}));

export const useBuilderStore = create<BuilderState>()(
  persist<BuilderState, [], [], Record<string, unknown>>(
    (set) => ({
      bio: "",
      businessName: "",
      businessCategory: "",
      vibe: "",
      wizardStep: 1,
      blocks: [],
      drafts: [],
      status: "onboarding",
      businessGoal: null,
      liveUrl: "",
      seoMetadata: {},

      setBio: (bio) => set({ bio }),
      setBusinessName: (businessName) => set({ businessName }),
      setBusinessCategory: (businessCategory) => set({ businessCategory }),
      setVibe: (vibe) => set({ vibe }),
      setWizardStep: (wizardStep) => set({ wizardStep }),
      setBlocks: (blocks) => set({ blocks }),
      setDrafts: (drafts) => set({ drafts }),
      setStatus: (status) => set({ status }),
      setBusinessGoal: (businessGoal) => set({ businessGoal }),
      setSeoMetadata: (seoMetadata) => set({ seoMetadata }),
      setLiveUrl: (liveUrl) => set({ liveUrl }),
    }),
    {
      name: 'legacy-builder-owned-draft',
      version: 3,
      storage,
      skipHydration: true,
      partialize: projectDraft,
      onRehydrateStorage: () => (_state, error) => { if (error) persistenceError = error instanceof Error ? error.message : 'This saved draft could not be read.'; },
      migrate: persisted => projectDraft((persisted ?? {}) as Partial<BuilderState>),
      merge: (persisted, current) => ({ ...current, ...projectDraft((persisted ?? {}) as Partial<BuilderState>), liveUrl: '' }),
    }
  )
);
const empty = useBuilderStore.getInitialState();
subscribeOnboardingInvalidation(() => {
  releaseBuilderEditor(scope); scope = null; references = 0; initialising = null; lastValue = null; persistenceError = ''; ++hydrationEpoch; hydrating = true;
  useBuilderStore.setState(empty, true); hydrating = false; notifyPersistence();
});
export async function initializeBuilderDraft() {
  if (!builderScopeActive(scope)) {
    if (!initialising) {
      const pending = (async () => {
        const next = await openBuilderEditor(LEGACY_BUILDER_DRAFT_KEY);
        scope = next; const hydration = ++hydrationEpoch; hydrating = true; persistenceError = '';
        try {
          useBuilderStore.setState(empty, true);
          await useBuilderStore.persist.rehydrate();
          if (!builderScopeActive(next)) throw new Error('Your session changed. Reopen the builder.');
          if (persistenceError) throw new Error(persistenceError);
          lastValue = canonicalRequest({ state: projectDraft(useBuilderStore.getState()), version: 3 });
        } catch (error) { releaseBuilderEditor(next); if (scope === next) scope = null; throw error; }
        finally { if (hydration === hydrationEpoch) hydrating = false; }
      })();
      initialising = pending;
      void pending.finally(() => { if (initialising === pending) initialising = null; }).catch(() => {});
    }
    await initialising;
  }
  const current = scope;
  if (!builderScopeActive(current)) throw new Error('Your session changed. Reopen the builder.');
  assertBuilderEditor(current, LEGACY_BUILDER_DRAFT_KEY);
  const record = readBuilderDraft<{ state: Record<string, unknown>; version: number }>(LEGACY_BUILDER_DRAFT_KEY, current);
  references += 1;
  let released = false;
  return { scope: current, hasLocal: !!record, release: () => {
    if (released) return;
    released = true;
    if (scope !== current) return;
    references -= 1;
    if (references === 0) { releaseBuilderEditor(current); scope = null; ++hydrationEpoch; hydrating = true; useBuilderStore.setState(empty, true); hydrating = false; }
  } };
}
