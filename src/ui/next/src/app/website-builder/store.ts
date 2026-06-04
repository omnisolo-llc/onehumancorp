import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface WebsiteBuilderState {
  wizardStep: number | string;
  businessName: string;
  businessType: string;
  hasPhysicalProducts: boolean;
  hasDigitalProducts: boolean;
  productName: string;
  productPrice: string;
  paymentMethod: string;
  userName: string;
  userEmail: string;
  userPassword: string;
  template: string;
  bio: string;
  domainChoice: string;
  aiAgents: string[];
  aiAutoRespond: boolean;
  setWizardStep: (step: number | string) => void;
  setBusinessName: (name: string) => void;
  setBusinessType: (type: string) => void;
  setHasPhysicalProducts: (has: boolean) => void;
  setHasDigitalProducts: (has: boolean) => void;
  setProductName: (name: string) => void;
  setProductPrice: (price: string) => void;
  setPaymentMethod: (method: string) => void;
  setUserName: (name: string) => void;
  setUserEmail: (email: string) => void;
  setUserPassword: (password: string) => void;
  setTemplate: (template: string) => void;
  setBio: (bio: string) => void;
  setDomainChoice: (domain: string) => void;
  setAiAgents: (agents: string[]) => void;
  setAiAutoRespond: (autoRespond: boolean) => void;
  loadState?: (state: Partial<WebsiteBuilderState>) => void;
}

export const useWebsiteBuilderStore = create<WebsiteBuilderState>()(
  persist(
    (set) => ({
      wizardStep: 0,
      businessName: '',
      businessType: '',
      hasPhysicalProducts: false,
      hasDigitalProducts: false,
      productName: '',
      productPrice: '',
      paymentMethod: '',
      userName: '',
      userEmail: '',
      userPassword: '',
      template: '',
      bio: '',
      domainChoice: 'subdomain',
      aiAgents: [],
      aiAutoRespond: false,
      setWizardStep: (wizardStep) => set({ wizardStep }),
      setBusinessName: (businessName) => set({ businessName }),
      setBusinessType: (businessType) => set({ businessType }),
      setHasPhysicalProducts: (hasPhysicalProducts) => set({ hasPhysicalProducts }),
      setHasDigitalProducts: (hasDigitalProducts) => set({ hasDigitalProducts }),
      setProductName: (productName) => set({ productName }),
      setProductPrice: (productPrice) => set({ productPrice }),
      setPaymentMethod: (paymentMethod) => set({ paymentMethod }),
      setUserName: (userName) => set({ userName }),
      setUserEmail: (userEmail) => set({ userEmail }),
      setUserPassword: (userPassword) => set({ userPassword }),
      setTemplate: (template) => set({ template }),
      setBio: (bio) => set({ bio }),
      setDomainChoice: (domainChoice) => set({ domainChoice }),
      setAiAgents: (aiAgents) => set({ aiAgents }),
      setAiAutoRespond: (aiAutoRespond) => set({ aiAutoRespond }),
      loadState: (state) => set(state),
    }),
    {
      name: 'website-builder-storage',
    }
  )
);
