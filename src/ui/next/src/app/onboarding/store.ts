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
  syncStateToBackend: () => Promise<void>;
  loadStateFromBackend: () => Promise<void>;
}

export const useOnboardingStore = create<OnboardingState>()(
  persist(
    (set, get) => ({
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
      syncStateToBackend: async () => {
        try {
          const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
          const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';
          const state = get();

          const res = await fetch('/api/onboarding/state', {
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
          if (!res.ok) {
            throw new Error('Sync failed with status ' + res.status);
          }
        } catch (err: any) {
          console.error('Failed to sync state to backend', err);
          set({ error: 'Failed to save progress to backend.' });
        }
      },
      loadStateFromBackend: async () => {
        try {
          const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront' : 'storefront';
          const userId = typeof localStorage !== 'undefined' ? localStorage.getItem('user_id') || 'test-user' : 'test-user';

          const res = await fetch('/api/onboarding/state', {
            headers: {
              'X-Tenant-ID': tenantId,
              'X-User-ID': userId,
            }
          });

          if (res.ok) {
            const data = await res.json();
            if (data && Object.keys(data).length > 0) {
              // Only override state if step is found in backend
              if (data.step) {
                set({
                  step: data.step || 1,
                  businessDescription: data.businessDescription || '',
                  businessName: data.businessName || '',
                  businessType: data.businessType || 'Online Store',
                  categories: data.categories || [],
                  websiteTemplate: data.websiteTemplate || 'Modern',
                  firstProductName: data.firstProductName || '',
                  firstProductPrice: data.firstProductPrice || '',
                });
              }
            }
          } else {
            throw new Error('Load failed with status ' + res.status);
          }
        } catch (err: any) {
          console.error('Failed to load state from backend', err);
          set({ error: 'Failed to load progress from backend.' });
        }
      }
    }),
    {
      name: 'onboarding-storage-v4', // Changed name to avoid cache collision with new structure
    }
  )
);
