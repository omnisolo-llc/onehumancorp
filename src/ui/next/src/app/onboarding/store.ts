import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface OnboardingState {
  step: number;
  bio: string;
  businessDescription: string;
  businessGoal: string;
  businessName: string;
  businessType: string;
  categories: string[];
  websiteTemplate: string;
  domainChoice: string;
  firstProductName: string;
  firstProductPrice: string;
  adminName: string;
  adminEmail: string;
  adminPassword: string;
  aiAgents: string[];
  aiAutoRespond: boolean;
  isLoading: boolean;
  error: string;
  startResult: any;
  setStep: (step: number) => void;
  setBio: (bio: string) => void;
  setBusinessDescription: (desc: string) => void;
  setBusinessGoal: (goal: string) => void;
  setBusinessName: (name: string) => void;
  setBusinessType: (type: string) => void;
  setCategories: (categories: string[]) => void;
  setWebsiteTemplate: (template: string) => void;
  setDomainChoice: (domain: string) => void;
  setFirstProductName: (name: string) => void;
  setFirstProductPrice: (price: string) => void;
  setAdminName: (name: string) => void;
  setAdminEmail: (email: string) => void;
  setAdminPassword: (password: string) => void;
  setAiAgents: (agents: string[]) => void;
  setAiAutoRespond: (autoRespond: boolean) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setStartResult: (result: any) => void;
}

export const useOnboardingStore = create<OnboardingState>()(
  persist(
    (set) => ({
      step: 0,
      bio: '',
      businessDescription: '',
      businessGoal: '',
      businessName: '',
      businessType: 'Online Store',
      categories: [],
      websiteTemplate: 'Modern',
      domainChoice: 'subdomain',
      firstProductName: '',
      firstProductPrice: '',
      adminName: '',
      adminEmail: '',
      adminPassword: '',
      aiAgents: [],
      aiAutoRespond: true,
      isLoading: false,
      error: '',
      startResult: null,
      setStep: (step) => set({ step }),
      setBio: (bio) => set({ bio }),
      setBusinessDescription: (businessDescription) => set({ businessDescription }),
      setBusinessGoal: (businessGoal) => set({ businessGoal }),
      setBusinessName: (businessName) => set({ businessName }),
      setBusinessType: (businessType) => set({ businessType }),
      setCategories: (categories) => set({ categories }),
      setWebsiteTemplate: (websiteTemplate) => set({ websiteTemplate }),
      setDomainChoice: (domainChoice) => set({ domainChoice }),
      setFirstProductName: (firstProductName) => set({ firstProductName }),
      setFirstProductPrice: (firstProductPrice) => set({ firstProductPrice }),
      setAdminName: (adminName) => set({ adminName }),
      setAdminEmail: (adminEmail) => set({ adminEmail }),
      setAdminPassword: (adminPassword) => set({ adminPassword }),
      setAiAgents: (aiAgents) => set({ aiAgents }),
      setAiAutoRespond: (aiAutoRespond) => set({ aiAutoRespond }),
      setIsLoading: (isLoading) => set({ isLoading }),
      setError: (error) => set({ error }),
      setStartResult: (startResult) => set({ startResult }),
    }),
    {
      name: 'onboarding-storage-v4', // Version bump since we removed keys
    }
  )
);
