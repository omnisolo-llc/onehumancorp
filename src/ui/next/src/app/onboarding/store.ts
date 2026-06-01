import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface OnboardingState {
  step: number;
  chatStep: number;
  businessDescription: string;
  businessName: string;
  whatYouSell: string;
  location: string;
  businessType: string;
  preferredLanguage: string;
  categories: string[];
  websiteTemplate: string;
  domainChoice: string;
  firstProductName: string;
  firstProductPrice: string;
  aiAgents: string[];
  aiAutoRespond: boolean;
  isLoading: boolean;
  error: string;
  startResult: any;
  setStep: (step: number) => void;
  setChatStep: (step: number) => void;
  setBusinessDescription: (desc: string) => void;
  setBusinessName: (name: string) => void;
  setWhatYouSell: (what: string) => void;
  setLocation: (location: string) => void;
  setBusinessType: (type: string) => void;
  setPreferredLanguage: (lang: string) => void;
  setCategories: (categories: string[]) => void;
  setWebsiteTemplate: (template: string) => void;
  setDomainChoice: (domain: string) => void;
  setFirstProductName: (name: string) => void;
  setFirstProductPrice: (price: string) => void;
  setAiAgents: (agents: string[]) => void;
  setAiAutoRespond: (autoRespond: boolean) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setStartResult: (result: any) => void;
}

export const useOnboardingStore = create<OnboardingState>()(
  persist(
    (set) => ({
      step: 1,
      chatStep: 1,
      businessDescription: '',
      businessName: '',
      whatYouSell: '',
      location: '',
      businessType: 'Online Store',
      preferredLanguage: 'English',
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
      setStep: (step) => set({ step }),
      setChatStep: (chatStep) => set({ chatStep }),
      setBusinessDescription: (businessDescription) => set({ businessDescription }),
      setBusinessName: (businessName) => set({ businessName }),
      setWhatYouSell: (whatYouSell) => set({ whatYouSell }),
      setLocation: (location) => set({ location }),
      setBusinessType: (businessType) => set({ businessType }),
      setPreferredLanguage: (preferredLanguage) => set({ preferredLanguage }),
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
    }),
    {
      name: 'onboarding-storage-v3', // Changed name to avoid cache collision with new structure
    }
  )
);
