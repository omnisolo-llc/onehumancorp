import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface OnboardingState {
  step: number;
  businessDescription: string;
  businessName: string;
  businessType: string;
  categories: string[];
  websiteTemplate: string;
  firstProductName: string;
  firstProductPrice: string;
  isLoading: boolean;
  error: string;
  startResult: any;
  setStep: (step: number) => void;
  setBusinessDescription: (desc: string) => void;
  setBusinessName: (name: string) => void;
  setBusinessType: (type: string) => void;
  setCategories: (categories: string[]) => void;
  setWebsiteTemplate: (template: string) => void;
  setFirstProductName: (name: string) => void;
  setFirstProductPrice: (price: string) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setStartResult: (result: any) => void;
}

export const useOnboardingStore = create<OnboardingState>()(
  persist(
    (set) => ({
      step: 1,
      businessDescription: '',
      businessName: '',
      businessType: 'Online Store',
      categories: [],
      websiteTemplate: 'Modern',
      firstProductName: '',
      firstProductPrice: '',
      isLoading: false,
      error: '',
      startResult: null,
      setStep: (step) => set({ step }),
      setBusinessDescription: (businessDescription) => set({ businessDescription }),
      setBusinessName: (businessName) => set({ businessName }),
      setBusinessType: (businessType) => set({ businessType }),
      setCategories: (categories) => set({ categories }),
      setWebsiteTemplate: (websiteTemplate) => set({ websiteTemplate }),
      setFirstProductName: (firstProductName) => set({ firstProductName }),
      setFirstProductPrice: (firstProductPrice) => set({ firstProductPrice }),
      setIsLoading: (isLoading) => set({ isLoading }),
      setError: (error) => set({ error }),
      setStartResult: (startResult) => set({ startResult }),
    }),
    {
      name: 'onboarding-storage-v3', // Changed name to avoid cache collision with new structure
    }
  )
);

export const syncStateToServer = async (state: OnboardingState) => {
  try {
    const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
    const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

    await fetch('/api/onboarding/state', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'X-Tenant-ID': tenantId,
        'X-User-ID': userId,
      },
      body: JSON.stringify({
        step: state.step,
        businessDescription: state.businessDescription,
        businessName: state.businessName,
        businessType: state.businessType,
        categories: state.categories,
        websiteTemplate: state.websiteTemplate,
        firstProductName: state.firstProductName,
        firstProductPrice: state.firstProductPrice,
      })
    });
  } catch (err) {
    console.error('Failed to sync onboarding state to server:', err);
  }
};

let syncTimeout: ReturnType<typeof setTimeout> | null = null;

useOnboardingStore.subscribe((state, prevState) => {
  // Prevent sync on load when previous state is effectively uninitialized (step 1 without business description)
  // Or when we just loaded it from server and are setting the local state.
  if (state.isLoading) return; // Don't sync while we are making other API calls

  if (
    state.step !== prevState.step ||
    state.businessDescription !== prevState.businessDescription ||
    state.businessName !== prevState.businessName ||
    state.businessType !== prevState.businessType ||
    state.categories !== prevState.categories ||
    state.websiteTemplate !== prevState.websiteTemplate ||
    state.firstProductName !== prevState.firstProductName ||
    state.firstProductPrice !== prevState.firstProductPrice
  ) {
    if (syncTimeout) {
      clearTimeout(syncTimeout);
    }
    syncTimeout = setTimeout(async () => {
      await syncStateToServer(state);
    }, 1000); // 1s debounce
  }
});
