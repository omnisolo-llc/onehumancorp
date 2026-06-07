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
  aiTone: string;
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
  setAiTone: (tone: string) => void;
  blocks: any[];
  status: "idle" | "generating" | "draft" | "live";
  liveUrl: string;
  setBlocks: (blocks: any[]) => void;
  moveBlock: (fromIndex: number, toIndex: number) => void;
  setStatus: (status: "idle" | "generating" | "draft" | "live") => void;
  setLiveUrl: (url: string) => void;
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
      aiTone: 'Friendly',
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
      setAiTone: (aiTone) => set({ aiTone }),
      blocks: [],
      status: "idle",
      liveUrl: "",
      setBlocks: (blocks) => set({ blocks }),
      moveBlock: (fromIndex, toIndex) =>
        set((state) => {
          if (
            toIndex < 0 ||
            toIndex >= state.blocks.length ||
            fromIndex === toIndex
          ) {
            return state;
          }
          const newBlocks = [...state.blocks];
          const [moved] = newBlocks.splice(fromIndex, 1);
          newBlocks.splice(toIndex, 0, moved);

          if (typeof localStorage !== "undefined") {
              localStorage.setItem("ohc_builder_blocks", JSON.stringify(newBlocks));
          }

          return { blocks: newBlocks };
        }),
      setStatus: (status) => set({ status }),
      setLiveUrl: (liveUrl) => set({ liveUrl }),
      loadState: (state) => set(state),
    }),
    {
      name: 'website-builder-storage',
    }
  )
);
