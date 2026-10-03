import React from 'react';
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, expect, it, vi } from 'vitest';
import UnifiedFeed from '../app/unified-feed/page';
import FeedPage from '../app/feed/page';
import ActionCenter from '../app/action-center/page';

vi.mock('../app/components/AppShell', () => ({ AppShell: ({ children }: { children: React.ReactNode }) => <main>{children}</main> }));
vi.mock('../hooks/useAuthenticatedPolling', () => ({ useAuthenticatedPolling: vi.fn() }));
vi.mock('../app/utils/offlineQueue', () => ({ getActions: async () => [] }));
afterEach(() => { cleanup(); vi.unstubAllGlobals(); });

const item = { id: 'feed-owned', event_source: 'Operations', context_payload: { summary: 'Review the real pending action' }, proposed_action: { action_type: 'proposal', summary: 'Real pending proposal' }, lifecycle_state: 'PENDING_APPROVAL', created_at: '2026-10-03T12:00:00Z' };
for (const [name, Component, label, body] of [
  ['unified feed', UnifiedFeed, 'Reject', { items: [item] }],
  ['daily feed', FeedPage, 'Dismiss', { items: [item] }],
  ['action center', ActionCenter, 'Dismiss', { pending_approvals: [{ id: 'approval-owned', department: 'operations', description: 'Review the real pending action', status: 'pending' }] }],
] as const) {
  it(`${name} completes the actual mutation response before discarding it`, async () => {
    const receipt = Response.json({ success: true });
    vi.stubGlobal('fetch', vi.fn().mockResolvedValueOnce(Response.json(body)).mockResolvedValueOnce(receipt));
    render(<Component />);
    fireEvent.click(await screen.findByRole('button', { name: label }));
    await waitFor(() => expect(receipt.bodyUsed).toBe(true));
    await waitFor(() => expect(screen.queryByRole('button', { name: label })).toBeNull());
  });
}
