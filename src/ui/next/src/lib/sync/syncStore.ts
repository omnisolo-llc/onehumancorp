import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';

export interface QueuedMutation {
  id: string;
  type: string;
  payload: any;
  timestamp: string;
  idempotencyKey: string;
  status: 'pending' | 'syncing' | 'failed' | 'completed';
  retryCount: number;
  lastError?: string;
  endpoint?: string;
}

interface SyncState {
  queue: QueuedMutation[];
  isOnline: boolean;
  isSyncing: boolean;
  enqueueMutation: (mutation: Omit<QueuedMutation, 'id' | 'timestamp' | 'idempotencyKey' | 'status' | 'retryCount'>) => QueuedMutation;
  updateMutationStatus: (id: string, status: QueuedMutation['status'], error?: string) => void;
  removeMutation: (id: string) => void;
  clearQueue: () => void;
  setOnlineStatus: (status: boolean) => void;
  setSyncingStatus: (status: boolean) => void;
  incrementRetry: (id: string) => void;
  getPendingMutations: () => QueuedMutation[];
}

const generateId = () => {
  return 'txn_' + Date.now() + '_' + Math.random().toString(36).substring(2, 9);
};

export const useSyncStore = create<SyncState>()(
  persist(
    (set, get) => ({
      queue: [],
      isOnline: typeof navigator !== 'undefined' ? navigator.onLine : true,
      isSyncing: false,

      enqueueMutation: (mutationInput) => {
        const newMutation: QueuedMutation = {
          ...mutationInput,
          id: generateId(),
          timestamp: new Date().toISOString(),
          idempotencyKey: 'idempotency_' + Date.now() + '_' + Math.random().toString(36).substring(2, 9),
          status: 'pending',
          retryCount: 0,
        };

        set((state) => ({
          queue: [...state.queue, newMutation]
        }));

        return newMutation;
      },

      updateMutationStatus: (id, status, error) => {
        set((state) => ({
          queue: state.queue.map(m =>
            m.id === id
              ? { ...m, status, lastError: error }
              : m
          )
        }));
      },

      removeMutation: (id) => {
        set((state) => ({
          queue: state.queue.filter(m => m.id !== id)
        }));
      },

      clearQueue: () => {
        set({ queue: [] });
      },

      setOnlineStatus: (status) => {
        set({ isOnline: status });
      },

      setSyncingStatus: (status) => {
        set({ isSyncing: status });
      },

      incrementRetry: (id) => {
        set((state) => ({
          queue: state.queue.map(m =>
            m.id === id
              ? { ...m, retryCount: m.retryCount + 1 }
              : m
          )
        }));
      },

      getPendingMutations: () => {
        return get().queue.filter(m => m.status === 'pending' || m.status === 'failed');
      }
    }),
    {
      name: 'ohc-offline-queue-storage',
      storage: createJSONStorage(() => localStorage),
    }
  )
);
