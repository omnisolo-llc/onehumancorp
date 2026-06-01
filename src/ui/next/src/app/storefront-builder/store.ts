import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface StorefrontBuilderState {
  bio: string;
  blocks: any[];
  status: "idle" | "generating" | "draft" | "live";
  liveUrl: string;
  draggedIndex: number | null;
  selectedBlockIndex: number | null;
  tenantId: string;

  setBio: (bio: string) => void;
  setBlocks: (blocks: any[]) => void;
  setStatus: (status: "idle" | "generating" | "draft" | "live") => void;
  setLiveUrl: (url: string) => void;
  setDraggedIndex: (index: number | null) => void;
  setSelectedBlockIndex: (index: number | null) => void;
  setTenantId: (id: string) => void;
  moveBlock: (fromIndex: number, toIndex: number) => void;
}

export const useStorefrontBuilderStore = create<StorefrontBuilderState>()(
  persist(
    (set) => ({
      bio: '',
      blocks: [],
      status: 'idle',
      liveUrl: '',
      draggedIndex: null,
      selectedBlockIndex: null,
      tenantId: 'storefront',

      setBio: (bio) => set({ bio }),
      setBlocks: (blocks) => set({ blocks }),
      setStatus: (status) => set({ status }),
      setLiveUrl: (liveUrl) => set({ liveUrl }),
      setDraggedIndex: (draggedIndex) => set({ draggedIndex }),
      setSelectedBlockIndex: (selectedBlockIndex) => set({ selectedBlockIndex }),
      setTenantId: (tenantId) => set({ tenantId }),

      moveBlock: (fromIndex, toIndex) => set((state) => {
        const blocks = [...state.blocks];
        const [movedBlock] = blocks.splice(fromIndex, 1);
        blocks.splice(toIndex, 0, movedBlock);
        return { blocks };
      }),
    }),
    {
      name: 'storefront-builder-storage',
      partialize: (state) => ({
        bio: state.bio,
        blocks: state.blocks,
        status: state.status,
        liveUrl: state.liveUrl
      }),
    }
  )
);
