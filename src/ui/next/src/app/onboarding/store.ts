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
  setStep: (step: number, skipSync?: boolean) => void;
  setChatStep: (step: number, skipSync?: boolean) => void;
  setBusinessDescription: (desc: string, skipSync?: boolean) => void;
  setBusinessName: (name: string, skipSync?: boolean) => void;
  setWhatYouSell: (what: string, skipSync?: boolean) => void;
  setLocation: (location: string, skipSync?: boolean) => void;
  setBusinessType: (type: string, skipSync?: boolean) => void;
  setCategories: (categories: string[], skipSync?: boolean) => void;
  setWebsiteTemplate: (template: string, skipSync?: boolean) => void;
  setFirstProductName: (name: string, skipSync?: boolean) => void;
  setFirstProductPrice: (price: string, skipSync?: boolean) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setStartResult: (result: any) => void;
  syncWithBackend: () => Promise<void>;
  hydrateState: (data: Partial<OnboardingState>) => void;
}

let syncTimeoutId: any = null;

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
      hydrateState: (data) => set({ ...data }),
      setStep: (step, skipSync = false) => { set({ step }); if (!skipSync) get().syncWithBackend(); },
      setChatStep: (chatStep, skipSync = false) => { set({ chatStep }); if (!skipSync) get().syncWithBackend(); },
      setBusinessDescription: (businessDescription, skipSync = false) => { set({ businessDescription }); if (!skipSync) get().syncWithBackend(); },
      setBusinessName: (businessName, skipSync = false) => { set({ businessName }); if (!skipSync) get().syncWithBackend(); },
      setWhatYouSell: (whatYouSell, skipSync = false) => { set({ whatYouSell }); if (!skipSync) get().syncWithBackend(); },
      setLocation: (location, skipSync = false) => { set({ location }); if (!skipSync) get().syncWithBackend(); },
      setBusinessType: (businessType, skipSync = false) => { set({ businessType }); if (!skipSync) get().syncWithBackend(); },
      setCategories: (categories, skipSync = false) => { set({ categories }); if (!skipSync) get().syncWithBackend(); },
      setWebsiteTemplate: (websiteTemplate, skipSync = false) => { set({ websiteTemplate }); if (!skipSync) get().syncWithBackend(); },
      setFirstProductName: (firstProductName, skipSync = false) => { set({ firstProductName }); if (!skipSync) get().syncWithBackend(); },
      setFirstProductPrice: (firstProductPrice, skipSync = false) => { set({ firstProductPrice }); if (!skipSync) get().syncWithBackend(); },
      setIsLoading: (isLoading) => set({ isLoading }),
      setError: (error) => set({ error }),
      setStartResult: (startResult) => set({ startResult }),
      syncWithBackend: async () => {
        if (syncTimeoutId) clearTimeout(syncTimeoutId);

        syncTimeoutId = setTimeout(async () => {
          try {
            const currentState = get();
            const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
            const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

            await fetch('/api/onboarding/state', {
              method: 'POST',
              headers: {
                'Content-Type': 'application/json',
                'x-tenant-id': tenantId,
                'x-user-id': userId
              },
              body: JSON.stringify({
                step: currentState.step,
                chatStep: currentState.chatStep,
                businessName: currentState.businessName,
                whatYouSell: currentState.whatYouSell,
                location: currentState.location,
                businessType: currentState.businessType,
                categories: currentState.categories,
                websiteTemplate: currentState.websiteTemplate,
                firstProductName: currentState.firstProductName,
                firstProductPrice: currentState.firstProductPrice,
              })
            });
          } catch (e) {
            console.error("Failed to sync onboarding state to backend", e);
          }
        }, 1000);
      },
    }),
    {
      name: 'onboarding-storage-v3',
    }
  )
);
