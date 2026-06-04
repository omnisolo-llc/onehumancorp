import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';

export interface OfflineMutation {
  id: string;
  type: string;
  payload: any;
  timestamp: string;
  idempotency_key: string;
  status: 'pending' | 'syncing' | 'failed' | 'completed';
}

interface SyncState {
  queue: OfflineMutation[];
  isOnline: boolean;
  isSyncing: boolean;
  addMutation: (mutation: Omit<OfflineMutation, 'id' | 'timestamp' | 'status' | 'idempotency_key'>) => void;
  removeMutation: (id: string) => void;
  updateMutationStatus: (id: string, status: OfflineMutation['status']) => void;
  setOnlineStatus: (isOnline: boolean) => void;
  setSyncingStatus: (isSyncing: boolean) => void;
  clearQueue: () => void;
}

export const useSyncStore = create<SyncState>()(
  persist(
    (set, get) => ({
      queue: [],
      isOnline: typeof navigator !== 'undefined' ? navigator.onLine : true,
      isSyncing: false,

      addMutation: (mutation) => {
        const newMutation: OfflineMutation = {
          ...mutation,
          id: `mut_${Date.now()}_${Math.random().toString(36).substring(7)}`,
          timestamp: new Date().toISOString(),
          idempotency_key: `idemp_${Date.now()}_${Math.random().toString(36).substring(7)}`,
          status: 'pending',
        };
        set((state) => ({ queue: [...state.queue, newMutation] }));
      },

      removeMutation: (id) => {
        set((state) => ({ queue: state.queue.filter((m) => m.id !== id) }));
      },

      updateMutationStatus: (id, status) => {
        set((state) => ({
          queue: state.queue.map((m) =>
            m.id === id ? { ...m, status } : m
          ),
        }));
      },

      setOnlineStatus: (isOnline) => set({ isOnline }),
      setSyncingStatus: (isSyncing) => set({ isSyncing }),
      clearQueue: () => set({ queue: [] }),
    }),
    {
      name: 'ohc-offline-sync-queue',
      storage: createJSONStorage(() => localStorage),
    }
  )
);
