import { hasOnboardingWriteFence, subscribeOnboardingWriteState, onboardingWriteVersion } from './draftWriteGate';
import { create } from 'zustand';
import { createJSONStorage, persist } from 'zustand/middleware';
import { canonicalRequest } from './contracts';
import { sameOwner, type QueueOwner } from '@/lib/sync/queueIdentity';
import { openOnboardingSession, onboardingOwner, onboardingSessionEpoch, ownedOnboardingKey, readOwnedOnboardingItem, writeOwnedOnboardingItem, subscribeOnboardingInvalidation } from './draftSession';
import type { OnboardingResult } from '@/lib/builder-types';

export interface OnboardingState {
  step: number;
  chatStep: number;
  bio: string;
  businessDescription: string;
  businessGoal: string;
  businessName: string;
  whatYouSell: string;
  location: string;
  targetAudience: string;
  businessType: string;
  categories: string[];
  websiteTemplate: string;
  domainChoice: string;
  firstProductName: string;
  firstProductPrice: string;
  aiAgents: string[];
  aiAutoRespond: boolean;
  isLoading: boolean;
  error: string;
  startResult: OnboardingResult | null;
  instantImageUrl: string;
  skipped: boolean;
  setStep: (step: number) => void;
  setChatStep: (step: number) => void;
  setBio: (bio: string) => void;
  setBusinessDescription: (desc: string) => void;
  setBusinessGoal: (goal: string) => void;
  setBusinessName: (name: string) => void;
  setWhatYouSell: (what: string) => void;
  setLocation: (location: string) => void;
  setTargetAudience: (target: string) => void;
  setBusinessType: (type: string) => void;
  setCategories: (categories: string[]) => void;
  setWebsiteTemplate: (template: string) => void;
  setDomainChoice: (domain: string) => void;
  setFirstProductName: (name: string) => void;
  setFirstProductPrice: (price: string) => void;
  setAiAgents: (agents: string[]) => void;
  setAiAutoRespond: (autoRespond: boolean) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setStartResult: (result: OnboardingResult) => void;
  setInstantImageUrl: (url: string) => void;
  setSkipped: (skipped: boolean) => void;
  updateState: (updates: Partial<OnboardingState>) => void;
}

let hydrating = false;
let storageFailure: unknown;
let hydratedKey: string | null = null;
let lastLocalFingerprint: string | null = null;
function persistedDraft(state: OnboardingState): Record<string, unknown> {
  const saved = { ...state } as Record<string, unknown>;
  for (const key of ['adminName', 'adminEmail', 'adminPassword', 'isLoading', 'error', 'startResult']) delete saved[key];
  saved.step = state.step >= 4 ? 3 : state.step;
  return JSON.parse(JSON.stringify(saved)) as Record<string, unknown>;
}
type DraftRow = { state: Record<string, unknown>; version: number; revision?: string; acknowledgedRevision?: string | null; acknowledgedWriteVersion?: string | null };
const persistenceListeners = new Set<() => void>();
function notifyPersistence() { for (const listener of persistenceListeners) listener(); }
subscribeOnboardingWriteState(notifyPersistence);
export function subscribeOnboardingPersistence(listener: () => void) { persistenceListeners.add(listener); return () => { persistenceListeners.delete(listener); }; }
function readDraftRow(): DraftRow | null {
  const raw = readOwnedOnboardingItem('draft');
  if (!raw) return null;
  const row = JSON.parse(raw) as DraftRow;
  if (!row || !row.state || typeof row.state !== 'object' || Array.isArray(row.state) || typeof row.version !== 'number') throw new Error('The saved draft could not be read. Its original bytes remain held.');
  return row;
}
export function onboardingDraftPending(): boolean {
  if (!onboardingOwner()) return false;
  if (storageFailure || hasOnboardingWriteFence(onboardingOwner()!)) return true;
  try { const row = readDraftRow(); return !!row && (!row.revision || row.revision !== row.acknowledgedRevision || (row.acknowledgedWriteVersion ?? null) !== onboardingWriteVersion(onboardingOwner()!)); }
  catch { return true; }
}
export function markOnboardingDraftFromServer(expectedWriteVersion?: string | null): void {
  try {
    const row = readDraftRow();
    if (!row || !onboardingOwner() || hasOnboardingWriteFence(onboardingOwner()!)) return;
    const writeVersion = onboardingWriteVersion(onboardingOwner()!);
    if (expectedWriteVersion !== undefined && expectedWriteVersion !== writeVersion) return;
    const revision = row.revision || crypto.randomUUID();
    writeOwnedOnboardingItem('draft', JSON.stringify({ ...row, revision, acknowledgedRevision: revision, acknowledgedWriteVersion: writeVersion }));
  } catch (cause) { storageFailure = cause; }
  notifyPersistence();
}
export function markOnboardingDraftPending(expected: QueueOwner | null): void {
  try {
    const owner = onboardingOwner(); const row = readDraftRow();
    if (!expected || !owner || !sameOwner(owner, expected) || !row) return;
    writeOwnedOnboardingItem('draft', JSON.stringify({ ...row, acknowledgedRevision: null }));
  } catch (cause) { storageFailure = cause; }
  notifyPersistence();
}
export type DraftWriteStamp = { owner: QueueOwner; epoch: number; revision: string; snapshot: string };
export function captureOnboardingDraftWrite(body: BodyInit | null | undefined): DraftWriteStamp | null {
  try {
    const owner = onboardingOwner(); const row = readDraftRow();
    if (!owner || !row?.revision || storageFailure || typeof body !== 'string') return null;
    const payload = JSON.parse(body) as Record<string, unknown>;
    if (!Object.entries(row.state).every(([key, value]) => key in payload && canonicalRequest(payload[key]) === canonicalRequest(value))) return null;
    const current = useOnboardingStore.getState();
    if (Object.entries(payload).some(([key, value]) => canonicalRequest(current[key as keyof OnboardingState]) !== canonicalRequest(value))) return null;
    return { owner, epoch: onboardingSessionEpoch(), revision: row.revision, snapshot: canonicalRequest(row.state) };
  } catch { return null; }
}
export function acknowledgeOnboardingDraft(stamp: DraftWriteStamp | null, committedWriteVersion?: string): boolean {
  if (!stamp) return false;
  try {
    const owner = onboardingOwner(); const row = readDraftRow();
    if (!owner || !sameOwner(owner, stamp.owner) || stamp.epoch !== onboardingSessionEpoch() || !row || hasOnboardingWriteFence(owner)) return false;
    const writeVersion = onboardingWriteVersion(owner);
    if (committedWriteVersion !== undefined && committedWriteVersion !== writeVersion) return false;
    const matches = row.revision === stamp.revision && canonicalRequest(row.state) === stamp.snapshot;
    // An older write may have reached the server after a newer one. Keep the current local snapshot pending.
    writeOwnedOnboardingItem('draft', JSON.stringify({ ...row, acknowledgedRevision: matches ? row.revision : null, acknowledgedWriteVersion: matches ? writeVersion : null }));
    notifyPersistence(); return matches;
  } catch (cause) { storageFailure = cause; notifyPersistence(); return false; }
}
const ownedStorage = createJSONStorage(() => ({
  getItem: () => { try { return readOwnedOnboardingItem('draft'); } catch (cause) { storageFailure = cause; throw cause; } },
  setItem: (_name: string, value: string) => {
    if (hydrating || !onboardingOwner()) return;
    try {
      const next = JSON.parse(value) as DraftRow;
      const fingerprint = canonicalRequest({ state: next.state, version: next.version });
      if (fingerprint === lastLocalFingerprint) return;
      const previous = readDraftRow();
      const unchanged = previous && canonicalRequest(previous.state) === canonicalRequest(next.state) && previous.version === next.version;
      const revision = unchanged && previous.revision ? previous.revision : crypto.randomUUID();
      writeOwnedOnboardingItem('draft', JSON.stringify({ ...next, revision, acknowledgedRevision: previous?.acknowledgedRevision ?? null, acknowledgedWriteVersion: previous?.acknowledgedWriteVersion ?? null }));
      lastLocalFingerprint = fingerprint;
    } catch (cause) { storageFailure = cause; }
    notifyPersistence();
  },
  // No automatic deletion: legacy and other owners' drafts remain held.
  removeItem: () => {},
}));

export const useOnboardingStore = create<OnboardingState>()(
  persist(
    (set) => ({
      step: -2,
      chatStep: 1,
      bio: '',
      businessDescription: '',
      businessGoal: '',
      businessName: '',
      whatYouSell: '',
      location: '',
      targetAudience: '',
      businessType: 'Online Store',
      categories: [],
      websiteTemplate: 'Modern',
  domainChoice: 'subdomain',
      firstProductName: '',
      firstProductPrice: '',
      aiAgents: [],
      aiAutoRespond: true,
      isLoading: false,
      error: '',
      startResult: null,
      instantImageUrl: '',
      skipped: false,
      setStep: (step) => set({ step }),
      setChatStep: (chatStep) => set({ chatStep }),
      setBio: (bio) => set({ bio }),
      setBusinessDescription: (businessDescription) => set({ businessDescription }),
      setBusinessGoal: (businessGoal) => set({ businessGoal }),
      setBusinessName: (businessName) => set({ businessName }),
      setWhatYouSell: (whatYouSell) => set({ whatYouSell }),
      setLocation: (location) => set({ location }),
      setTargetAudience: (targetAudience) => set({ targetAudience }),
      setBusinessType: (businessType) => set({ businessType }),
      setCategories: (categories) => set({ categories }),
      setWebsiteTemplate: (websiteTemplate) => set({ websiteTemplate }),
  setDomainChoice: (domainChoice) => set({ domainChoice }),
      setFirstProductName: (firstProductName) => set({ firstProductName }),
      setFirstProductPrice: (firstProductPrice) => set({ firstProductPrice }),
      setAiAgents: (aiAgents) => set({ aiAgents }),
      setAiAutoRespond: (aiAutoRespond) => set({ aiAutoRespond }),
      setIsLoading: (isLoading) => set({ isLoading }),
      setError: (error) => set({ error }),
      setStartResult: (startResult) => set({ startResult }),
      setInstantImageUrl: (instantImageUrl) => set({ instantImageUrl }),
      setSkipped: (skipped) => set({ skipped }),
      updateState: (updates) => set((state) => ({ ...state, ...updates })),
    }),
    {
      name: 'onboarding-owned-draft',
      storage: ownedStorage,
      skipHydration: true,
      onRehydrateStorage: () => (_state, error) => { if (error) storageFailure = error; },
      version: 6,
      partialize: persistedDraft,
      migrate: (persistedState) => persistedState as Record<string, unknown>,
      merge: (persistedState, current) => {
        const saved = persistedState && typeof persistedState === 'object' ? { ...persistedState } as Record<string, unknown> : {};
        for (const key of ['adminName', 'adminEmail', 'adminPassword', 'isLoading', 'error', 'startResult']) delete saved[key];
        if (typeof saved.step === 'number' && saved.step >= 4) saved.step = 3;
        return { ...current, ...saved, isLoading: false, error: '', startResult: null } as OnboardingState;
      },
    }
  )
);

const emptyDraft = useOnboardingStore.getInitialState();
subscribeOnboardingInvalidation(() => {
  hydratedKey = null; lastLocalFingerprint = null;
  hydrating = true;
  useOnboardingStore.setState(emptyDraft, true);
  hydrating = false;
});
export async function initializeOnboardingDraft() {
  const owner = await openOnboardingSession();
  const key = ownedOnboardingKey('draft');
  if (hydratedKey === key) {
    const row = readDraftRow();
    if (storageFailure || (row && canonicalRequest({ state: row.state, version: row.version }) === lastLocalFingerprint)) return owner;
  }
  const epoch = onboardingSessionEpoch();
  storageFailure = undefined; hydrating = true;
  try {
    useOnboardingStore.setState(emptyDraft, true);
    await useOnboardingStore.persist.rehydrate();
    if (storageFailure) throw new Error('The saved draft could not be read. It remains held on this device.');
    readDraftRow();
    if (epoch !== onboardingSessionEpoch()) throw new Error('Your session changed. Please reopen setup.');
    hydratedKey = key;
    lastLocalFingerprint = canonicalRequest({ state: persistedDraft(useOnboardingStore.getState()), version: 6 });
    return owner;
  } finally { hydrating = false; }
}
export function onboardingStorageFailed(): boolean { return storageFailure !== undefined; }
