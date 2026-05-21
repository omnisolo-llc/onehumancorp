import { create } from 'zustand';

interface OnboardingState {
  step: number;
  businessName: string;
  businessCategory: string;
  isLoading: boolean;
  error: string;
  intakeData: any;
  startResult: any;
  setStep: (step: number) => void;
  setBusinessName: (name: string) => void;
  setBusinessCategory: (category: string) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setIntakeData: (data: any) => void;
  setStartResult: (result: any) => void;
  loadState: () => Promise<void>;
  saveState: () => Promise<void>;
}

let debounceTimer: any = null;

export const useOnboardingStore = create<OnboardingState>((set, get) => ({
  step: 1,
  businessName: '',
  businessCategory: '',
  isLoading: false,
  error: '',
  intakeData: null,
  startResult: null,
  setStep: (step) => {
    set({ step });
    get().saveState(); // Steps change immediately
  },
  setBusinessName: (businessName) => {
    set({ businessName });
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => get().saveState(), 500);
  },
  setBusinessCategory: (businessCategory) => {
    set({ businessCategory });
    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => get().saveState(), 500);
  },
  setIsLoading: (isLoading) => set({ isLoading }),
  setError: (error) => set({ error }),
  setIntakeData: (intakeData) => {
    set({ intakeData });
    get().saveState();
  },
  setStartResult: (startResult) => {
    set({ startResult });
    get().saveState();
  },
  loadState: async () => {
    try {
      const tenantId = localStorage.getItem('tenant_id') || 'default_tenant';
      const userId = localStorage.getItem('user_id') || 'default_user';

      const res = await fetch('/api/onboarding/state', {
        headers: {
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId
        }
      });
      if (res.ok) {
        const data = await res.json();
        if (data && Object.keys(data).length > 0) {
          set({
            step: data.step || 1,
            businessName: data.businessName || data.bio || '',
            businessCategory: data.businessCategory || '',
            intakeData: data.intakeData || null,
            startResult: data.startResult || null,
          });
        }
      }
    } catch (e) {
      console.error('Failed to load state', e);
    }
  },
  saveState: async () => {
    try {
      const tenantId = localStorage.getItem('tenant_id') || 'default_tenant';
      const userId = localStorage.getItem('user_id') || 'default_user';
      const state = get();

      await fetch('/api/onboarding/state', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId
        },
        body: JSON.stringify({
          step: state.step,
          bio: state.businessName,
          businessName: state.businessName,
          businessCategory: state.businessCategory,
          intakeData: state.intakeData,
          startResult: state.startResult,
        })
      });
    } catch (e) {
      console.error('Failed to save state', e);
    }
  }
}));
