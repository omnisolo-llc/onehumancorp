import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface OnboardingState {
  step: number;
  businessBio: string;
  firstProductName: string;
  firstProductPrice: string;
  template: string;
  domain: string;
  isLoading: boolean;
  error: string;
  intakeData: any;
  startResult: any;
  setStep: (step: number) => void;
  setBusinessBio: (bio: string) => void;
  setFirstProductName: (name: string) => void;
  setFirstProductPrice: (price: string) => void;
  setTemplate: (template: string) => void;
  setDomain: (domain: string) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setIntakeData: (data: any) => void;
  setStartResult: (result: any) => void;
}

export const useOnboardingStore = create<OnboardingState>()(
  persist(
    (set) => ({
      step: 1,
      businessBio: '',
      firstProductName: '',
      firstProductPrice: '',
      template: 'Modern',
      domain: 'free',
      isLoading: false,
      error: '',
      intakeData: null,
      startResult: null,
      setStep: (step) => set({ step }),
      setBusinessBio: (businessBio) => set({ businessBio }),
      setFirstProductName: (firstProductName) => set({ firstProductName }),
      setFirstProductPrice: (firstProductPrice) => set({ firstProductPrice }),
      setTemplate: (template) => set({ template }),
      setDomain: (domain) => set({ domain }),
      setIsLoading: (isLoading) => set({ isLoading }),
      setError: (error) => set({ error }),
      setIntakeData: (intakeData) => set({ intakeData }),
      setStartResult: (startResult) => set({ startResult }),
    }),
    {
      name: 'onboarding-storage',
    }
  )
);
