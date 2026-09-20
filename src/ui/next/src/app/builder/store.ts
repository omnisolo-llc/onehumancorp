import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { BuilderBlock } from '@/lib/builder-types';

interface BuilderState {
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

  setBio: (bio: string) => void;
  setBusinessName: (name: string) => void;
  setBusinessCategory: (category: string) => void;
  setVibe: (vibe: string) => void;
  setWizardStep: (step: number) => void;
  setBlocks: (blocks: BuilderBlock[]) => void;
  setDrafts: (drafts: BuilderBlock[][]) => void;
  setStatus: (status: "onboarding" | "idle" | "generating" | "draft" | "selection" | "live") => void;
  setBusinessGoal: (goal: "products" | "services" | "work" | null) => void;
  setLiveUrl: (url: string) => void;
}

export const useBuilderStore = create<BuilderState>()(
  persist(
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

      setBio: (bio) => set({ bio }),
      setBusinessName: (businessName) => set({ businessName }),
      setBusinessCategory: (businessCategory) => set({ businessCategory }),
      setVibe: (vibe) => set({ vibe }),
      setWizardStep: (wizardStep) => set({ wizardStep }),
      setBlocks: (blocks) => set({ blocks }),
      setDrafts: (drafts) => set({ drafts }),
      setStatus: (status) => set({ status }),
      setBusinessGoal: (businessGoal) => set({ businessGoal }),
      setLiveUrl: (liveUrl) => set({ liveUrl }),
    }),
    {
      name: 'builder-storage',
      version: 2,
      migrate: (persistedState) => {
        const legacy = (persistedState ?? {}) as Record<string, unknown>;
        return legacy as unknown as BuilderState;
      },
    }
  )
);
