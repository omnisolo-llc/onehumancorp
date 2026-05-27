import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface OnboardingState {
  step: number;
  businessName: string;
  businessCategory: string;
  businessGoal: string;
  isLoading: boolean;
  error: string;
  startResult: any;
  setStep: (step: number) => void;
  setBusinessName: (name: string) => void;
  setBusinessCategory: (category: string) => void;
  setBusinessGoal: (goal: string) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setStartResult: (result: any) => void;
}

export const useOnboardingStore = create<OnboardingState>()(
  persist(
    (set) => ({
      step: 1,
      businessName: '',
      businessCategory: '',
      businessGoal: '',
      isLoading: false,
      error: '',
      startResult: null,
      setStep: (step) => set({ step }),
      setBusinessName: (businessName) => set({ businessName }),
      setBusinessCategory: (businessCategory) => set({ businessCategory }),
      setBusinessGoal: (businessGoal) => set({ businessGoal }),
      setIsLoading: (isLoading) => set({ isLoading }),
      setError: (error) => set({ error }),
      setStartResult: (startResult) => set({ startResult }),
    }),
    {
      name: 'onboarding-storage-v3', // Changed name to avoid cache collision
    }
  )
);