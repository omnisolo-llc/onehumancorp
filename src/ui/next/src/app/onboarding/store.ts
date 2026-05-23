import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface OnboardingState {
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
  loadStateFromBackend: () => Promise<void>;
  syncStateToBackend: () => Promise<void>;
}

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
      setStep: (step) => {
        set({ step });
        get().syncStateToBackend();
      },
      setBusinessType: (businessType) => { set({ businessType }); get().syncStateToBackend(); },
      setBusinessName: (businessName) => { set({ businessName }); get().syncStateToBackend(); },
      setBusinessCategory: (businessCategory) => { set({ businessCategory }); get().syncStateToBackend(); },
      setIsLoading: (isLoading) => set({ isLoading }),
      setError: (error) => set({ error }),
      setIntakeData: (intakeData) => set({ intakeData }),
      setStartResult: (startResult) => set({ startResult }),
      loadStateFromBackend: async () => {
        try {
          const res = await fetch('/api/onboarding/state');
          if (res.ok) {
            const data = await res.json();
            if (data && typeof data.step === 'number') {
              set({
                step: data.step || 1,
                businessType: data.businessType || '',
                businessName: data.businessName || '',
                businessCategory: data.businessCategory || ''
              });
            }
          }
        } catch (e) {
          console.error("Failed to load onboarding state", e);
        }
      },
      syncStateToBackend: async () => {
        const { step, businessType, businessName, businessCategory } = get();

        if (typeof window !== 'undefined') {
          if ((window as any)._syncDebounce) {
            clearTimeout((window as any)._syncDebounce);
          }

          (window as any)._syncDebounce = setTimeout(async () => {
            try {
              await fetch('/api/onboarding/state', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                  step,
                  businessType,
                  businessName,
                  businessCategory
                }),
              });
            } catch (e) {
              console.error("Failed to sync onboarding state", e);
            }
          }, 500); // 500ms debounce
        } else {
          try {
            await fetch('/api/onboarding/state', {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({
                step,
                businessType,
                businessName,
                businessCategory
              }),
            });
          } catch (e) {
            console.error("Failed to sync onboarding state", e);
          }
        }
      },
    }),
    {
      name: 'onboarding-storage', // name of the item in the storage (must be unique)
    }
  )
);
