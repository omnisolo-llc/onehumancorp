import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface WebsiteBuilderState {
  wizardStep: number | string;
  wizardData: {
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
  };
  setWizardStep: (step: number | string) => void;
  updateWizardData: (data: Partial<WebsiteBuilderState['wizardData']>) => void;
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
      wizardData: {
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
      },
      setWizardStep: (wizardStep) => set({ wizardStep }),
      updateWizardData: (data) => set((state) => ({ wizardData: { ...state.wizardData, ...data } })),
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
