import { create } from 'zustand';
import { persist } from 'zustand/middleware';

export interface OnboardingState {
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
  loadFromServer: () => Promise<void>;
}

// Debounce helper
let saveTimeout: any = null;

const syncToServer = (state: Partial<OnboardingState>) => {
  if (typeof window === 'undefined') return; // Only in browser

  clearTimeout(saveTimeout);
  saveTimeout = setTimeout(async () => {
    try {
      const tenantId = localStorage.getItem('tenant_id') || 'test-tenant';
      const userId = localStorage.getItem('user_id') || 'test-user';

      // Load current state to get full object since state might only contain partial updates from middleware
      const currentState = useOnboardingStore.getState();

      // Save state
      await fetch('/api/onboarding/state', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId
        },
        body: JSON.stringify({
          step: currentState.step,
          businessType: currentState.businessType,
          businessName: currentState.businessName,
          businessCategory: currentState.businessCategory,
          intakeData: currentState.intakeData
        })
      });
    } catch (e) {
      console.error('Failed to sync onboarding state to server:', e);
    }
  }, 500);
};

export const useOnboardingStore = create<OnboardingState>()(
  persist(
    (set, get) => ({
      step: 1,
      businessType: '',
      businessName: '',
      businessCategory: '',
      isLoading: false,
      error: '',
      intakeData: null,
      startResult: null,
      setStep: (step) => { set({ step }); syncToServer({ step }); },
      setBusinessType: (businessType) => { set({ businessType }); syncToServer({ businessType }); },
      setBusinessName: (businessName) => { set({ businessName }); syncToServer({ businessName }); },
      setBusinessCategory: (businessCategory) => { set({ businessCategory }); syncToServer({ businessCategory }); },
      setIsLoading: (isLoading) => set({ isLoading }),
      setError: (error) => set({ error }),
      setIntakeData: (intakeData) => { set({ intakeData }); syncToServer({ intakeData }); },
      setStartResult: (startResult) => set({ startResult }),
      loadFromServer: async () => {
        try {
          const tenantId = localStorage.getItem('tenant_id') || 'test-tenant';
          const userId = localStorage.getItem('user_id') || 'test-user';
          const res = await fetch('/api/onboarding/state', {
            headers: {
              'X-Tenant-ID': tenantId,
              'X-User-ID': userId
            }
          });
          if (res.ok) {
            const data = await res.json();
            if (data && Object.keys(data).length > 0 && !data.bio) { // Ignore dummy bio if returned
               set((state) => ({
                 step: data.step !== undefined ? data.step : state.step,
                 businessType: data.businessType !== undefined ? data.businessType : state.businessType,
                 businessName: data.businessName !== undefined ? data.businessName : state.businessName,
                 businessCategory: data.businessCategory !== undefined ? data.businessCategory : state.businessCategory,
                 intakeData: data.intakeData !== undefined ? data.intakeData : state.intakeData,
               }));
            }
          }
        } catch (e) {
          console.error('Failed to load state from server:', e);
        }
      }
    }),
    {
      name: 'onboarding-storage', // name of the item in the storage
      onRehydrateStorage: () => (state) => {
         // Optionally load from server after local rehydration
         if (state) {
            state.loadFromServer();
         }
      }
    }
  )
);
