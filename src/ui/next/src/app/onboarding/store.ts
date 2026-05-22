import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface OnboardingState {
  step: number;
  businessName: string;
  businessCategory: string;
  isLoading: boolean;
  error: string;
  intakeData: any;
  startResult: any;
  setStep: (step: number) => void;
  setBusinessName: (name: string) => void;
  setBusinessCategory: (category: string) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setIntakeData: (data: any) => void;
  setStartResult: (result: any) => void;
}

export const useOnboardingStore = create<OnboardingState>()(
  persist(
    (set) => ({
      step: 1,
      businessName: '',
      businessCategory: '',
      isLoading: false,
      error: '',
      intakeData: null,
      startResult: null,
      setStep: (step) => set({ step }),
      setBusinessName: (businessName) => set({ businessName }),
      setBusinessCategory: (businessCategory) => set({ businessCategory }),
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
