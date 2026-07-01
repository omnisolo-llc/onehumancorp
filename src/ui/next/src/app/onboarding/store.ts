import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface OnboardingState {
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
  adminName: string;
  adminEmail: string;
  adminPassword: string;
  aiAgents: string[];
  aiAutoRespond: boolean;
  isLoading: boolean;
  error: string;
  startResult: any;
  instantImageUrl: string;
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
  setAdminName: (name: string) => void;
  setAdminEmail: (email: string) => void;
  setAdminPassword: (password: string) => void;
  setAiAgents: (agents: string[]) => void;
  setAiAutoRespond: (autoRespond: boolean) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setStartResult: (result: any) => void;
  setInstantImageUrl: (url: string) => void;
  updateState: (updates: Partial<OnboardingState>) => void;
}

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
      adminName: '',
      adminEmail: '',
      adminPassword: '',
      aiAgents: [],
      aiAutoRespond: true,
      isLoading: false,
      error: '',
      startResult: null,
      instantImageUrl: '',
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
      setAdminName: (adminName) => set({ adminName }),
      setAdminEmail: (adminEmail) => set({ adminEmail }),
      setAdminPassword: (adminPassword) => set({ adminPassword }),
      setAiAgents: (aiAgents) => set({ aiAgents }),
      setAiAutoRespond: (aiAutoRespond) => set({ aiAutoRespond }),
      setIsLoading: (isLoading) => set({ isLoading }),
      setError: (error) => set({ error }),
      setStartResult: (startResult) => set({ startResult }),
      setInstantImageUrl: (instantImageUrl) => set({ instantImageUrl }),
      updateState: (updates) => set((state) => ({ ...state, ...updates })),
    }),
    {
      name: 'onboarding-storage-v4', // Upgraded structure for seamless cross-device resumes
    }
  )
);
