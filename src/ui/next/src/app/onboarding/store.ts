import { create } from 'zustand';

interface OnboardingState {
  step: number;
  businessName: string;
  businessCategory: string;
  isInstantBuild: boolean;
  businessDescription: string;
  isLoading: boolean;
  error: string;
  intakeData: any;
  startResult: any;
  setStep: (step: number) => void;
  setBusinessName: (name: string) => void;
  setBusinessCategory: (category: string) => void;
  setIsInstantBuild: (isInstant: boolean) => void;
  setBusinessDescription: (desc: string) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setIntakeData: (data: any) => void;
  setStartResult: (result: any) => void;
}

const loadInitialState = () => {
  if (typeof window !== 'undefined') {
    try {
      const saved = localStorage.getItem('ohc_react_wizard_state');
      if (saved) {
        const parsed = JSON.parse(saved);
        return {
          step: parsed.step || 1,
          businessName: parsed.businessName || '',
          businessCategory: parsed.businessCategory || '',
          isInstantBuild: parsed.isInstantBuild || false,
          businessDescription: parsed.businessDescription || '',
          intakeData: parsed.intakeData || null,
        };
      }
    } catch (e) {
      console.error('Failed to parse saved state', e);
    }
  }
  return {
    step: 1,
    businessName: '',
    businessCategory: '',
    isInstantBuild: false,
    businessDescription: '',
    intakeData: null,
  };
};

export const useOnboardingStore = create<OnboardingState>((set) => {
  const initialState = loadInitialState();

  const updateStateAndSave = (updates: Partial<OnboardingState>) => {
    set((state) => {
      const nextState = { ...state, ...updates };
      if (typeof window !== 'undefined') {
        localStorage.setItem('ohc_react_wizard_state', JSON.stringify({
          step: nextState.step,
          businessName: nextState.businessName,
          businessCategory: nextState.businessCategory,
          isInstantBuild: nextState.isInstantBuild,
          businessDescription: nextState.businessDescription,
          intakeData: nextState.intakeData,
        }));
      }
      return nextState;
    });
  };

  return {
    ...initialState,
    isLoading: false,
    error: '',
    startResult: null,
    setStep: (step) => updateStateAndSave({ step }),
    setBusinessName: (businessName) => updateStateAndSave({ businessName }),
    setBusinessCategory: (businessCategory) => updateStateAndSave({ businessCategory }),
    setIsInstantBuild: (isInstantBuild) => updateStateAndSave({ isInstantBuild }),
    setBusinessDescription: (businessDescription) => updateStateAndSave({ businessDescription }),
    setIsLoading: (isLoading) => set({ isLoading }),
    setError: (error) => set({ error }),
    setIntakeData: (intakeData) => updateStateAndSave({ intakeData }),
    setStartResult: (startResult) => set({ startResult }),
  };
});
