import { create } from 'zustand';
interface OnboardingState {
  step: number;
  bio: string;
  isLoading: boolean;
  error: string;
  intakeData: any;
  startResult: any;
  setStep: (step: number) => void;
  setBio: (bio: string) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setIntakeData: (data: any) => void;
  setStartResult: (result: any) => void;
}
export const useOnboardingStore = create<OnboardingState>((set) => ({
  step: 1,
  bio: '',
  isLoading: false,
  error: '',
  intakeData: null,
  startResult: null,
  setStep: (step) => set({ step }),
  setBio: (bio) => set({ bio }),
  setIsLoading: (isLoading) => set({ isLoading }),
  setError: (error) => set({ error }),
  setIntakeData: (intakeData) => set({ intakeData }),
  setStartResult: (startResult) => set({ startResult }),
}));
