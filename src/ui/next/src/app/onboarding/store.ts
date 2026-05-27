import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface OnboardingState {
  step: number;
  whatDoYouCreate: string;
  instagramHandle: string;
  stripeConnected: boolean;

  businessDescription: string;
  businessName: string;
  businessType: string;
  categories: string[];
  websiteTemplate: string;
  firstProductName: string;
  firstProductPrice: string;
  isLoading: boolean;
  error: string;
  startResult: any;

  setStep: (step: number) => void;
  setWhatDoYouCreate: (desc: string) => void;
  setInstagramHandle: (handle: string) => void;
  setStripeConnected: (connected: boolean) => void;

  setBusinessDescription: (desc: string) => void;
  setBusinessName: (name: string) => void;
  setBusinessType: (type: string) => void;
  setCategories: (categories: string[]) => void;
  setWebsiteTemplate: (template: string) => void;
  setFirstProductName: (name: string) => void;
  setFirstProductPrice: (price: string) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setStartResult: (result: any) => void;
}

export const useOnboardingStore = create<OnboardingState>()(
  persist(
    (set) => ({
      step: 1,
      whatDoYouCreate: '',
      instagramHandle: '',
      stripeConnected: false,
      businessDescription: '',
      businessName: '',
      businessType: 'Online Store',
      categories: [],
      websiteTemplate: 'Modern',
      firstProductName: '',
      firstProductPrice: '',
      isLoading: false,
      error: '',
      startResult: null,

      setStep: (step) => set({ step }),
      setWhatDoYouCreate: (whatDoYouCreate) => set({ whatDoYouCreate }),
      setInstagramHandle: (instagramHandle) => set({ instagramHandle }),
      setStripeConnected: (stripeConnected) => set({ stripeConnected }),

      setBusinessDescription: (businessDescription) => set({ businessDescription }),
      setBusinessName: (businessName) => set({ businessName }),
      setBusinessType: (businessType) => set({ businessType }),
      setCategories: (categories) => set({ categories }),
      setWebsiteTemplate: (websiteTemplate) => set({ websiteTemplate }),
      setFirstProductName: (firstProductName) => set({ firstProductName }),
      setFirstProductPrice: (firstProductPrice) => set({ firstProductPrice }),
      setIsLoading: (isLoading) => set({ isLoading }),
      setError: (error) => set({ error }),
      setStartResult: (startResult) => set({ startResult }),
    }),
    {
      name: 'onboarding-storage-v4', // Changed name to avoid cache collision with new structure
    }
  )
);
