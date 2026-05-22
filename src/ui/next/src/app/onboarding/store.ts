import { create } from 'zustand';

interface OnboardingState {
  step: number;
  businessType: string;
  businessName: string;
  businessCategory: string;
  isLoading: boolean;
  error: string;
  intakeData: any;
  startResult: any;
  setStep: (step: number) => void;
  setBusinessType: (type: string) => void;
  setBusinessName: (name: string) => void;
  setBusinessCategory: (category: string) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setIntakeData: (data: any) => void;
  setStartResult: (result: any) => void;
}

export const useOnboardingStore = create<OnboardingState>((set) => ({
  step: 1,
  businessType: '',
  businessName: '',
  businessCategory: '',
  isLoading: false,
  error: '',
  intakeData: null,
  startResult: null,
  setStep: (step) => set({ step }),
  setBusinessType: (businessType) => set({ businessType }),
  setBusinessName: (businessName) => set({ businessName }),
  setBusinessCategory: (businessCategory) => set({ businessCategory }),
  setIsLoading: (isLoading) => set({ isLoading }),
  setError: (error) => set({ error }),
  setIntakeData: (intakeData) => set({ intakeData }),
  setStartResult: (startResult) => set({ startResult }),
}));
