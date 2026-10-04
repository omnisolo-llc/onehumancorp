import React from 'react';
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, expect, it, vi } from 'vitest';
import UnifiedFeed from '../app/unified-feed/page';
import FeedPage from '../app/feed/page';
import ActionCenter from '../app/action-center/page';
import { notifyQueueIdentityChange } from './sync/queueIdentity';
import { installOnboardingLocks } from '../app/onboarding/testLocks';

vi.mock('../app/components/AppShell', () => ({ AppShell: ({ children }: { children: React.ReactNode }) => <main>{children}</main> }));
vi.mock('../hooks/useAuthenticatedPolling', () => ({ useAuthenticatedPolling: vi.fn() }));
vi.mock('../app/utils/offlineQueue', () => ({ getActions: async () => [] }));
afterEach(() => { cleanup(); notifyQueueIdentityChange(); vi.unstubAllGlobals(); });

const item = { id: 'feed-owned', tenant_id: 'owner-tenant', event_source: 'Operations', context_payload: { summary: 'Review the real pending action' }, proposed_action: { action_type: 'proposal', summary: 'Real pending proposal' }, lifecycle_state: 'PENDING_APPROVAL', created_at: '2026-10-03T12:00:00Z' };
for (const [name, Component, label, body] of [
  ['unified feed', UnifiedFeed, 'Reject', { items: [item] }],
  ['daily feed', FeedPage, 'Dismiss', { items: [item] }],
  ['action center', ActionCenter, 'Dismiss', { pending_approvals: [{ id: 'approval-owned', department: 'operations', description: 'Review the real pending action', status: 'pending' }] }],
] as const) {
  it(`${name} completes the actual mutation response before discarding it`, async () => {
    localStorage.clear(); notifyQueueIdentityChange(); installOnboardingLocks();
    const receipt = Response.json(name === 'unified feed' ? { ...item, lifecycle_state: 'DISMISSED' } : { success: true });
    vi.stubGlobal('fetch', vi.fn(async (url, options) => {
      if (String(url).endsWith('/session-identity')) return Response.json({ userId: 'owner-user', tenantId: 'owner-tenant', expiresAt: Date.now() + 60_000 });
      if (options?.method === 'PUT' || options?.method === 'POST') return receipt;
      return Response.json(body);
    }));
    render(<Component />);
    fireEvent.click(await screen.findByRole('button', { name: label }));
    await waitFor(() => expect(receipt.bodyUsed).toBe(true));
    await waitFor(() => expect(screen.queryByRole('button', { name: label })).toBeNull());
  });
}
