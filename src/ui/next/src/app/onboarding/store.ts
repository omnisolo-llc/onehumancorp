import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface OnboardingState {
  step: number;
  businessType: string;
  businessName: string;
  businessCategory: string;
  intakeData: any | null;
  startResult: any | null;
  setStep: (step: number) => void;
  setBusinessType: (type: string) => void;
  setBusinessName: (name: string) => void;
  setBusinessCategory: (category: string) => void;
  setIntakeData: (data: any) => void;
  setStartResult: (result: any) => void;
  reset: () => void;
}

export const useOnboardingStore = create<OnboardingState>()(
  persist(
    (set) => ({
      step: 1,
      businessType: '',
      businessName: '',
      businessCategory: '',
      intakeData: null,
      startResult: null,
      setStep: (step) => set({ step }),
      setBusinessType: (type) => set({ businessType: type }),
      setBusinessName: (name) => set({ businessName: name }),
      setBusinessCategory: (category) => set({ businessCategory: category }),
      setIntakeData: (data) => set({ intakeData: data }),
      setStartResult: (result) => set({ startResult: result }),
      reset: () => set({
        step: 1,
        businessType: '',
        businessName: '',
        businessCategory: '',
        intakeData: null,
        startResult: null,
      }),
    }),
    {
      name: 'onboarding-storage', // name of item in the storage (must be unique)
    }
  )
);
