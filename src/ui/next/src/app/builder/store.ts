import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface BuilderState {
  bio: string;
  businessName: string;
  businessCategory: string;
  vibe: string;
  wizardStep: number;
  blocks: any[];
  drafts: any[][];
  status: "onboarding" | "idle" | "generating" | "draft" | "selection" | "live";
  businessGoal: "products" | "services" | "work" | null;
  agentTeam: string;
  liveUrl: string;

  setBio: (bio: string) => void;
  setBusinessName: (name: string) => void;
  setBusinessCategory: (category: string) => void;
  setVibe: (vibe: string) => void;
  setWizardStep: (step: number) => void;
  setBlocks: (blocks: any[]) => void;
  setDrafts: (drafts: any[][]) => void;
  setStatus: (status: "onboarding" | "idle" | "generating" | "draft" | "selection" | "live") => void;
  setBusinessGoal: (goal: "products" | "services" | "work" | null) => void;
  setAgentTeam: (team: string) => void;
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
      agentTeam: "Customer Support",
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
      setAgentTeam: (agentTeam) => set({ agentTeam }),
      setLiveUrl: (liveUrl) => set({ liveUrl }),
    }),
    {
      name: 'builder-storage',
    }
  )
);
