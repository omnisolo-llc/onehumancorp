import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import { v4 as uuidv4 } from 'uuid';

interface MutationEvent {
  id: string;
  type: string;
  payload: any;
  timestamp: string;
  endpoint: string;
  method: 'POST' | 'PUT' | 'PATCH' | 'DELETE';
  retryCount: number;
}

interface OfflineSyncState {
  queue: MutationEvent[];
  isSyncing: boolean;
  enqueueMutation: (endpoint: string, method: MutationEvent['method'], type: string, payload: any) => void;
  dequeueMutation: (id: string) => void;
  incrementRetry: (id: string) => void;
  setSyncing: (isSyncing: boolean) => void;
  clearQueue: () => void;
  getQueue: () => MutationEvent[];
}

export const useOfflineSyncStore = create<OfflineSyncState>()(
  persist(
    (set, get) => ({
      queue: [],
      isSyncing: false,
      enqueueMutation: (endpoint, method, type, payload) => {
        const newEvent: MutationEvent = {
          id: uuidv4(),
          type,
          payload,
          timestamp: new Date().toISOString(),
          endpoint,
          method,
          retryCount: 0,
        };
        set((state) => ({ queue: [...state.queue, newEvent] }));
      },
      dequeueMutation: (id) => {
        set((state) => ({ queue: state.queue.filter((event) => event.id !== id) }));
      },
      incrementRetry: (id) => {
        set((state) => ({
          queue: state.queue.map((event) =>
            event.id === id ? { ...event, retryCount: event.retryCount + 1 } : event
          ),
        }));
      },
      setSyncing: (isSyncing) => set({ isSyncing }),
      clearQueue: () => set({ queue: [] }),
      getQueue: () => get().queue,
    }),
    {
      name: 'ohc-offline-sync-queue',
      storage: createJSONStorage(() => localStorage),
    }
  )
);
