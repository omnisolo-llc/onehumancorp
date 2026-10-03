import { create } from 'zustand';
import { createJSONStorage, persist } from 'zustand/middleware';
import { openBuilderEditor, releaseBuilderEditor, assertBuilderEditor, builderScopeActive, readBuilderDraft, writeBuilderDraft, type BuilderScope } from '../builder/ownedDraft';
import { subscribeOnboardingInvalidation } from '../onboarding/draftSession';
import { canonicalRequest } from '../onboarding/contracts';
import type { BuilderBlock } from '@/lib/builder-types';

export interface WebsiteBuilderState {
  wizardStep: number | string;
  businessName: string;
  businessType: string;
  hasPhysicalProducts: boolean;
  hasDigitalProducts: boolean;
  productName: string;
  productPrice: string;
  paymentMethod: string;
  template: string;
  bio: string;
  domainChoice: string;
  aiAgents: string[];
  aiAutoRespond: boolean;
  setWizardStep: (step: number | string) => void;
  setBusinessName: (name: string) => void;
  setBusinessType: (type: string) => void;
  setHasPhysicalProducts: (has: boolean) => void;
  setHasDigitalProducts: (has: boolean) => void;
  setProductName: (name: string) => void;
  setProductPrice: (price: string) => void;
  setPaymentMethod: (method: string) => void;
  setTemplate: (template: string) => void;
  setBio: (bio: string) => void;
  setDomainChoice: (domain: string) => void;
  setAiAgents: (agents: string[]) => void;
  setAiAutoRespond: (autoRespond: boolean) => void;
  blocks: BuilderBlock[];
  status: "idle" | "generating" | "draft" | "live";
  liveUrl: string;
  setBlocks: (blocks: BuilderBlock[]) => void;
  moveBlock: (fromIndex: number, toIndex: number) => void;
  setStatus: (status: "idle" | "generating" | "draft" | "live") => void;
  setLiveUrl: (url: string) => void;
  loadState?: (state: Partial<WebsiteBuilderState>) => void;
}

export const WEBSITE_DRAFT_KEY = 'website-builder-draft';
let scope: BuilderScope | null = null;
let hydrating = false;
let lastValue: string | null = null;
let references = 0;
let initialising: Promise<void> | null = null;
let persistenceError = '';
const persistenceListeners = new Set<() => void>();
export function websiteDraftError() { return persistenceError; }
export function subscribeWebsitePersistence(listener: () => void) { persistenceListeners.add(listener); return () => { persistenceListeners.delete(listener); }; }
function notifyPersistence() { for (const listener of persistenceListeners) listener(); }
const businessFields = ['wizardStep','businessName','businessType','hasPhysicalProducts','hasDigitalProducts','productName','productPrice','paymentMethod','template','bio','domainChoice','aiAgents','aiAutoRespond','blocks','status'] as const;
function projectDraft(value: Partial<WebsiteBuilderState>) {
  const saved: Record<string, unknown> = {};
  for (const field of businessFields) if (value[field] !== undefined) saved[field] = value[field];
  if (saved.status === 'live' || saved.status === 'generating') saved.status = Array.isArray(saved.blocks) && saved.blocks.length ? 'draft' : 'idle';
  return saved;
}
const storage = createJSONStorage<Record<string, unknown>>(() => ({
  getItem: () => {
    if (!scope) return null;
    const record = readBuilderDraft<{ state: Record<string, unknown>; version: number }>(WEBSITE_DRAFT_KEY, scope);
    if (record && (!record.data.state || typeof record.data.state !== 'object' || Array.isArray(record.data.state))) throw new Error('This saved builder draft could not be read. Its original copy remains held.');
    return record ? JSON.stringify(record.data) : null;
  },
  setItem: (_name: string, value: string) => {
    if (hydrating || !builderScopeActive(scope)) return;
    try {
      const data = JSON.parse(value);
      const fingerprint = canonicalRequest(data);
      if (fingerprint === lastValue) return;
      writeBuilderDraft(WEBSITE_DRAFT_KEY, data, scope);
      lastValue = fingerprint; persistenceError = "";
    } catch (error) { persistenceError = error instanceof Error ? error.message : 'This device could not save your latest edits.'; }
    notifyPersistence();
  },
  removeItem: () => {},
}));

export const useWebsiteBuilderStore = create<WebsiteBuilderState>()(
  persist<WebsiteBuilderState, [], [], Record<string, unknown>>(
    (set) => ({
      wizardStep: 0,
      businessName: '',
      businessType: '',
      hasPhysicalProducts: false,
      hasDigitalProducts: false,
      productName: '',
      productPrice: '',
      paymentMethod: '',
      template: '',
      bio: '',
      domainChoice: 'subdomain',
      aiAgents: [],
      aiAutoRespond: false,
      setWizardStep: (wizardStep) => set({ wizardStep }),
      setBusinessName: (businessName) => set({ businessName }),
      setBusinessType: (businessType) => set({ businessType }),
      setHasPhysicalProducts: (hasPhysicalProducts) => set({ hasPhysicalProducts }),
      setHasDigitalProducts: (hasDigitalProducts) => set({ hasDigitalProducts }),
      setProductName: (productName) => set({ productName }),
      setProductPrice: (productPrice) => set({ productPrice }),
      setPaymentMethod: (paymentMethod) => set({ paymentMethod }),
      setTemplate: (template) => set({ template }),
      setBio: (bio) => set({ bio }),
      setDomainChoice: (domainChoice) => set({ domainChoice }),
      setAiAgents: (aiAgents) => set({ aiAgents }),
      setAiAutoRespond: (aiAutoRespond) => set({ aiAutoRespond }),
      blocks: [],
      status: "idle",
      liveUrl: "",
      setBlocks: (blocks) => set({ blocks }),
      moveBlock: (fromIndex, toIndex) =>
        set((state) => {
          if (
            toIndex < 0 ||
            toIndex >= state.blocks.length ||
            fromIndex === toIndex
          ) {
            return state;
          }
          const newBlocks = [...state.blocks];
          const [moved] = newBlocks.splice(fromIndex, 1);
          newBlocks.splice(toIndex, 0, moved);

          return { blocks: newBlocks };
        }),
      setStatus: (status) => set({ status }),
      setLiveUrl: (liveUrl) => set({ liveUrl }),
      loadState: (state) => set(projectDraft(state)),
    }),
    {
      name: 'website-builder-owned-draft',
      version: 3,
      storage,
      skipHydration: true,
      partialize: projectDraft,
      onRehydrateStorage: () => (_state, error) => { if (error) persistenceError = error instanceof Error ? error.message : 'This saved draft could not be read.'; },
      migrate: persisted => projectDraft((persisted ?? {}) as Partial<WebsiteBuilderState>),
      merge: (persisted, current) => ({ ...current, ...projectDraft((persisted ?? {}) as Partial<WebsiteBuilderState>), liveUrl: '' }),
    }
  )
);
const empty = useWebsiteBuilderStore.getInitialState();
subscribeOnboardingInvalidation(() => {
  releaseBuilderEditor(scope); scope = null; references = 0; initialising = null; lastValue = null; persistenceError = ''; hydrating = true;
  useWebsiteBuilderStore.setState(empty, true); hydrating = false; notifyPersistence();
});
export async function initializeWebsiteDraft() {
  if (!builderScopeActive(scope)) {
    if (!initialising) {
      const pending = (async () => {
        const next = await openBuilderEditor(WEBSITE_DRAFT_KEY);
        scope = next; hydrating = true; persistenceError = '';
        try {
          useWebsiteBuilderStore.setState(empty, true);
          await useWebsiteBuilderStore.persist.rehydrate();
          if (!builderScopeActive(next)) throw new Error('Your session changed. Reopen the builder.');
          if (persistenceError) throw new Error(persistenceError);
          lastValue = canonicalRequest({ state: projectDraft(useWebsiteBuilderStore.getState()), version: 3 });
        } catch (error) { releaseBuilderEditor(next); if (scope === next) scope = null; throw error; }
        finally { hydrating = false; }
      })();
      initialising = pending;
      void pending.finally(() => { if (initialising === pending) initialising = null; }).catch(() => {});
    }
    await initialising;
  }
  const current = scope;
  if (!builderScopeActive(current)) throw new Error('Your session changed. Reopen the builder.');
  assertBuilderEditor(current, WEBSITE_DRAFT_KEY);
  const record = readBuilderDraft<{ state: Record<string, unknown>; version: number }>(WEBSITE_DRAFT_KEY, current);
  references += 1;
  let released = false;
  return { scope: current, hasLocal: !!record, release: () => {
    if (released) return;
    released = true;
    if (scope !== current) return;
    references -= 1;
    if (references === 0) { releaseBuilderEditor(current); scope = null; }
  } };
}
