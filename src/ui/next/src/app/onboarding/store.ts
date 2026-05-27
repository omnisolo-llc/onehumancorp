import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface OnboardingState {
  step: number;
  businessDescription: string;
  isLoading: boolean;
  error: string;
  startResult: any;
  setStep: (step: number) => void;
  setBusinessDescription: (desc: string) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setStartResult: (result: any) => void;
}

export const useOnboardingStore = create<OnboardingState>()(
  persist(
    (set) => ({
      step: 1,
      businessDescription: '',
      isLoading: false,
      error: '',
      startResult: null,
      setStep: (step) => set({ step }),
      setBusinessDescription: (businessDescription) => set({ businessDescription }),
      setIsLoading: (isLoading) => set({ isLoading }),
      setError: (error) => set({ error }),
      setStartResult: (startResult) => set({ startResult }),
    }),
    {
      name: 'onboarding-storage-v2', // Changed name to avoid cache collision
    }
  )
);
