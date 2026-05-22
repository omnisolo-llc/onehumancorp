import { create } from 'zustand';

interface OnboardingState {
  step: number;
  businessName: string;
  businessCategory: string;
  isInstantBuild: boolean;
  businessDescription: string;
  isLoading: boolean;
  error: string;
  intakeData: any;
  startResult: any;
  setStep: (step: number) => void;
  setBusinessName: (name: string) => void;
  setBusinessCategory: (category: string) => void;
  setIsInstantBuild: (isInstant: boolean) => void;
  setBusinessDescription: (desc: string) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setIntakeData: (data: any) => void;
  setStartResult: (result: any) => void;
}

export const useOnboardingStore = create<OnboardingState>((set) => ({
  step: 1,
  businessName: '',
  businessCategory: '',
  isInstantBuild: false,
  businessDescription: '',
  isLoading: false,
  error: '',
  intakeData: null,
  startResult: null,
  setStep: (step) => set({ step }),
  setBusinessName: (businessName) => set({ businessName }),
  setBusinessCategory: (businessCategory) => set({ businessCategory }),
  setIsInstantBuild: (isInstantBuild) => set({ isInstantBuild }),
  setBusinessDescription: (businessDescription) => set({ businessDescription }),
  setIsLoading: (isLoading) => set({ isLoading }),
  setError: (error) => set({ error }),
  setIntakeData: (intakeData) => set({ intakeData }),
  setStartResult: (startResult) => set({ startResult }),
}));
