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
  categories: string[];
  websiteTemplate: string;
  brandTone: string;
  selectedAgents: string[];
  firstProductName: string;
  firstProductPrice: string;
  isLoading: boolean;
  error: string;
  startResult: any;

  // Actions
  setStep: (step: number) => void;
  setChatStep: (step: number) => void;
  setBusinessDescription: (desc: string) => void;
  setBusinessName: (name: string) => void;
  setWhatYouSell: (what: string) => void;
  setLocation: (location: string) => void;
  setBusinessType: (type: string) => void;
  setCategories: (categories: string[]) => void;
  setWebsiteTemplate: (template: string) => void;
  setBrandTone: (tone: string) => void;
  setSelectedAgents: (agents: string[]) => void;
  setFirstProductName: (name: string) => void;
  setFirstProductPrice: (price: string) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setStartResult: (result: any) => void;

  // Validation
  isValidBusinessName: () => boolean;
  isValidProductName: () => boolean;
  isValidPrice: () => boolean;
}

export const useOnboardingStore = create<OnboardingState>()(
  persist(
    (set, get) => ({
      step: 1,
      chatStep: 1,
      businessDescription: '',
      businessName: '',
      whatYouSell: '',
      location: '',
      businessType: 'Online Store',
      categories: [],
      websiteTemplate: 'Modern',
      brandTone: 'Professional',
      selectedAgents: ['The Manager', 'The Promoter', 'The Ambassador'],
      firstProductName: '',
      firstProductPrice: '',
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
      setCategories: (categories) => set({ categories }),
      setWebsiteTemplate: (websiteTemplate) => set({ websiteTemplate }),
      setBrandTone: (brandTone) => set({ brandTone }),
      setSelectedAgents: (selectedAgents) => set({ selectedAgents }),
      setFirstProductName: (firstProductName) => set({ firstProductName }),
      setFirstProductPrice: (firstProductPrice) => set({ firstProductPrice }),
      setIsLoading: (isLoading) => set({ isLoading }),
      setError: (error) => set({ error }),
      setStartResult: (startResult) => set({ startResult }),

      isValidBusinessName: () => get().businessName.trim().length >= 2,
      isValidProductName: () => get().firstProductName.trim().length >= 2,
      isValidPrice: () => {
        const price = get().firstProductPrice;
        return !isNaN(parseFloat(price)) && isFinite(Number(price)) && parseFloat(price) >= 0;
      },
    }),
    {
      name: 'onboarding-storage-v5',
    }
  )
);
