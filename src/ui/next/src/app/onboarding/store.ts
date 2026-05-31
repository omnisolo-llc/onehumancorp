import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface OnboardingState {
  step: number;
  businessType: string;
  uploadedPhotos: string[];
  businessName: string;
  isLoading: boolean;
  error: string;
  startResult: any;
  setStep: (step: number) => void;
  setBusinessType: (type: string) => void;
  setUploadedPhotos: (photos: string[]) => void;
  setBusinessName: (name: string) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setStartResult: (result: any) => void;
}

export const useOnboardingStore = create<OnboardingState>()(
  persist(
    (set) => ({
      step: 1,
      businessType: '',
      uploadedPhotos: [],
      businessName: '',
      isLoading: false,
      error: '',
      startResult: null,
      setStep: (step) => set({ step }),
      setBusinessType: (businessType) => set({ businessType }),
      setUploadedPhotos: (uploadedPhotos) => set({ uploadedPhotos }),
      setBusinessName: (businessName) => set({ businessName }),
      setIsLoading: (isLoading) => set({ isLoading }),
      setError: (error) => set({ error }),
      setStartResult: (startResult) => set({ startResult }),
    }),
    {
      name: 'onboarding-storage-v4',
    }
  )
);
