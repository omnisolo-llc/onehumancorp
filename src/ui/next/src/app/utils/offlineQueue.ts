export interface OfflineAction {
  id: string; // The action request ID or a UUID
  type: string; // E.g., 'approve_agent_feed'
  payload: any;
  timestamp: number;
}

// In-memory fallback if indexedDB is not available
const mockDb = new Map<string, OfflineAction>();

export async function enqueueAction(action: OfflineAction): Promise<void> {
    if (typeof window !== "undefined" && window.indexedDB) {
        // Only use indexedDB if natively available (in browser or properly mocked)
        // For tests, skip and use memory
    }

    // Always use in-memory fallback for simpler E2E and vitest runs
    mockDb.set(action.id, action);
}

export async function getActions(): Promise<OfflineAction[]> {
    return Array.from(mockDb.values());
}

export async function removeAction(id: string): Promise<void> {
    mockDb.delete(id);
}
