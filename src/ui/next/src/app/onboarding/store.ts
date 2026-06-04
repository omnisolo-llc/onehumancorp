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
  domainChoice: string;
  firstProductName: string;
  firstProductPrice: string;
  adminName: string;
  adminEmail: string;
  adminPassword: string;
  aiAgents: string[];
  aiAutoRespond: boolean;
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
  setDomainChoice: (domain: string) => void;
  setFirstProductName: (name: string) => void;
  setFirstProductPrice: (price: string) => void;
  setAdminName: (name: string) => void;
  setAdminEmail: (email: string) => void;
  setAdminPassword: (password: string) => void;
  setAiAgents: (agents: string[]) => void;
  setAiAutoRespond: (autoRespond: boolean) => void;
  setIsLoading: (loading: boolean) => void;
  setError: (error: string) => void;
  setStartResult: (result: any) => void;
}

let syncTimeout: NodeJS.Timeout | null = null;
const syncWithBackend = async (state: Partial<OnboardingState>) => {
  if (typeof window === 'undefined') return; // Do not run on SSR

  if (syncTimeout) {
    clearTimeout(syncTimeout);
  }

  syncTimeout = setTimeout(async () => {
    const tenantId = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'storefront';
    const userId = localStorage.getItem('user_id') || 'test-user';

    try {
      await fetch('/api/onboarding/state', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'X-Tenant-ID': tenantId,
          'X-User-ID': userId,
        },
        body: JSON.stringify({ wizardState: state }),
      });
    } catch (error) {
      console.error('Failed to sync onboarding state with backend:', error);
    }
  }, 1000);
};

export const useOnboardingStore = create<OnboardingState>()(
  persist(
    (set, get) => ({
      step: 1,
      chatStep: 0,
      businessDescription: '',
      businessName: '',
      whatYouSell: '',
      location: '',
      businessType: 'Online Store',
      categories: [],
      websiteTemplate: 'Modern',
      domainChoice: 'subdomain',
      firstProductName: '',
      firstProductPrice: '',
      adminName: '',
      adminEmail: '',
      adminPassword: '',
      aiAgents: [],
      aiAutoRespond: true,
      isLoading: false,
      error: '',
      startResult: null,
      setStep: (step) => { set({ step }); syncWithBackend(get()); },
      setChatStep: (chatStep) => { set({ chatStep }); syncWithBackend(get()); },
      setBusinessDescription: (businessDescription) => { set({ businessDescription }); syncWithBackend(get()); },
      setBusinessName: (businessName) => { set({ businessName }); syncWithBackend(get()); },
      setWhatYouSell: (whatYouSell) => { set({ whatYouSell }); syncWithBackend(get()); },
      setLocation: (location) => { set({ location }); syncWithBackend(get()); },
      setBusinessType: (businessType) => { set({ businessType }); syncWithBackend(get()); },
      setCategories: (categories) => { set({ categories }); syncWithBackend(get()); },
      setWebsiteTemplate: (websiteTemplate) => { set({ websiteTemplate }); syncWithBackend(get()); },
      setDomainChoice: (domainChoice) => { set({ domainChoice }); syncWithBackend(get()); },
      setFirstProductName: (firstProductName) => { set({ firstProductName }); syncWithBackend(get()); },
      setFirstProductPrice: (firstProductPrice) => { set({ firstProductPrice }); syncWithBackend(get()); },
      setAdminName: (adminName) => { set({ adminName }); syncWithBackend(get()); },
      setAdminEmail: (adminEmail) => { set({ adminEmail }); syncWithBackend(get()); },
      setAdminPassword: (adminPassword) => { set({ adminPassword }); syncWithBackend(get()); },
      setAiAgents: (aiAgents) => { set({ aiAgents }); syncWithBackend(get()); },
      setAiAutoRespond: (aiAutoRespond) => { set({ aiAutoRespond }); syncWithBackend(get()); },
      setIsLoading: (isLoading) => set({ isLoading }),
      setError: (error) => set({ error }),
      setStartResult: (startResult) => set({ startResult }),
    }),
    {
      name: 'onboarding-storage-v3', // Changed name to avoid cache collision with new structure
    }
  )
);
