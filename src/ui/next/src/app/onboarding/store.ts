import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface OnboardingState {
  step: number;
  chatStep: number;
  businessDescription: string;
  businessName: string;
  whatYouSell: string;
  location: string;
  businessType: string;
  categories: string[];
  websiteTemplate: string;
  firstProductName: string;
  firstProductPrice: string;
  isLoading: boolean;
  error: string;
  startResult: any;
  setStep: (step: number) => void;
  setChatStep: (step: number) => void;
  setBusinessDescription: (desc: string) => void;
  setBusinessName: (name: string) => void;
  setWhatYouSell: (what: string) => void;
  setLocation: (location: string) => void;
  setBusinessType: (type: string) => void;
  setCategories: (categories: string[]) => void;
  setWebsiteTemplate: (template: string) => void;
  setFirstProductName: (name: string) => void;
  setFirstProductPrice: (price: string) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setStartResult: (result: any) => void;
  syncState: (newState: Partial<OnboardingState>) => void;
}

export const useOnboardingStore = create<OnboardingState>()(
  persist(
    (set, get) => ({
      step: 1,
      chatStep: 1,
      businessDescription: '',
      businessName: '',
      whatYouSell: '',
      location: '',
      businessType: 'Online Store',
      categories: [],
      websiteTemplate: 'Modern',
      firstProductName: '',
      firstProductPrice: '',
      isLoading: false,
      error: '',
      startResult: null,
      setStep: (step) => set({ step }),
      setChatStep: (chatStep) => set({ chatStep }),
      setBusinessDescription: (businessDescription) => set({ businessDescription }),
      setBusinessName: (businessName) => set({ businessName }),
      setWhatYouSell: (whatYouSell) => set({ whatYouSell }),
      setLocation: (location) => set({ location }),
      setBusinessType: (businessType) => set({ businessType }),
      setCategories: (categories) => set({ categories }),
      setWebsiteTemplate: (websiteTemplate) => set({ websiteTemplate }),
      setFirstProductName: (firstProductName) => set({ firstProductName }),
      setFirstProductPrice: (firstProductPrice) => set({ firstProductPrice }),
      setIsLoading: (isLoading) => set({ isLoading }),
      setError: (error) => set({ error }),
      setStartResult: (startResult) => set({ startResult }),
      syncState: (newState) => set((state) => ({ ...state, ...newState })),
    }),
    {
      name: 'onboarding-storage-v3', // Changed name to avoid cache collision with new structure
      onRehydrateStorage: () => (state) => {
        if (state) {
          // Fire and forget fetch state from backend
          fetch('/api/onboarding/state', {
            headers: {
              'X-Tenant-ID': typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'storefront' : 'storefront',
              'X-User-ID': typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user'
            }
          })
          .then(res => res.ok ? res.json() : null)
          .then(data => {
            if (data && Object.keys(data).length > 0) {
              state.syncState(data);
            }
          })
          .catch(console.error);
        }
      }
    }
  )
);

// Subscribe to state changes and push to backend
useOnboardingStore.subscribe(
  (state, prevState) => {
    // Basic debounce logic or just simple diff check to avoid loop.
    // In a real app we'd debounce this.
    const hasChanged = state.step !== prevState.step ||
                       state.chatStep !== prevState.chatStep ||
                       state.businessName !== prevState.businessName ||
                       state.businessType !== prevState.businessType ||
                       state.whatYouSell !== prevState.whatYouSell ||
                       state.location !== prevState.location;

    if (hasChanged) {
      fetch('/api/onboarding/state', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'storefront' : 'storefront',
          'X-User-ID': typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user'
        },
        body: JSON.stringify({
          step: state.step,
          chatStep: state.chatStep,
          businessName: state.businessName,
          businessType: state.businessType,
          whatYouSell: state.whatYouSell,
          location: state.location,
          categories: state.categories,
          websiteTemplate: state.websiteTemplate,
          firstProductName: state.firstProductName,
          firstProductPrice: state.firstProductPrice,
        })
      })?.catch?.(console.error);
    }
  }
);
