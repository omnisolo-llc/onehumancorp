import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface OnboardingState {
  step: number;
  chatStep: number;
  bio: string;
  businessDescription: string;
  businessGoal: string;
  businessName: string;
  whatYouSell: string;
  location: string;
  targetAudience: string;
  businessType: string;
  categories: string[];
  websiteTemplate: string;
  domainChoice: string;
  firstProductName: string;
  firstProductPrice: string;
  aiAgents: string[];
  aiAutoRespond: boolean;
  isLoading: boolean;
  error: string;
  startResult: any;
  instantImageUrl: string;
  updateState: (updates: Partial<OnboardingState>) => void;
}

export const useOnboardingStore = create<OnboardingState>()(
  persist(
    (set) => ({
      step: -2,
      chatStep: 1,
      bio: '',
      businessDescription: '',
      businessGoal: '',
      businessName: '',
      whatYouSell: '',
      location: '',
      targetAudience: '',
      businessType: 'Online Store',
      categories: [],
      websiteTemplate: 'Modern',
  domainChoice: 'subdomain',
      firstProductName: '',
      firstProductPrice: '',
      aiAgents: [],
      aiAutoRespond: true,
      isLoading: false,
      error: '',
      startResult: null,
      instantImageUrl: '',
      updateState: (updates) => set((state) => ({ ...state, ...updates })),
    }),
    {
      name: 'onboarding-storage-v4', // Upgraded structure for seamless cross-device resumes
      version: 5,
      migrate: (persistedState) => {
        if (persistedState && typeof persistedState === 'object') {
          const {
            adminName: _adminName,
            adminEmail: _adminEmail,
            adminPassword: _adminPassword,
            ...state
          } = persistedState as Record<string, unknown>;
          return state as unknown as OnboardingState;
        }
        return persistedState as OnboardingState;
      },
    }
  )
);
