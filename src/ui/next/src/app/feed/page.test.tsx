import React from 'react';
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import FeedPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: vi.fn() }),
}));

vi.mock('../components/AppShell', () => ({
  AppShell: ({ children }: { children: React.ReactNode }) => <main>{children}</main>,
}));

vi.mock('../../hooks/useAuthenticatedPolling', () => ({
  useAuthenticatedPolling: vi.fn(),
}));

describe('FeedPage', () => {
  const bookingFeedItem = {
    id: 'feed-booking-1',
    tenant_id: 'tenant-1',
    event_source: 'booking_request',
    context_payload: {
      booking_id: 'booking-1',
      customer_name: 'Taylor Customer',
      description: 'New booking request',
    },
    proposed_action: {
      action_type: 'approve_booking',
      booking_id: 'booking-1',
    },
    lifecycle_state: 'PENDING_APPROVAL',
    created_at: '2026-08-09T12:00:00Z',
    updated_at: '2026-08-09T12:00:00Z',
  };

  beforeEach(() => {
    vi.stubGlobal('fetch', vi.fn()
      .mockResolvedValueOnce(new Response(JSON.stringify({ items: [bookingFeedItem] }), { status: 200 }))
      .mockResolvedValueOnce(new Response(JSON.stringify({ item: { ...bookingFeedItem, lifecycle_state: 'APPROVED' } }), { status: 200 })));
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    vi.clearAllMocks();
  });

  it('renders actions for a pending booking and removes it after approval', async () => {
    render(<FeedPage />);

    expect(await screen.findByText('Review Required')).toBeInTheDocument();
    expect(screen.getByText('New booking request')).toBeInTheDocument();
    const approveButton = screen.getByRole('button', { name: 'Approve' });
    fireEvent.click(approveButton);

    await waitFor(() => {
      expect(fetch).toHaveBeenLastCalledWith('/api/v1/agent-feed/feed-booking-1/state', expect.objectContaining({
        method: 'PUT',
      }));
      expect(screen.queryByText('Review Required')).not.toBeInTheDocument();
    });
  });

  it('lets the owner edit a generic proposed action before approval', async () => {
    render(<FeedPage />);

    await screen.findByText('New booking request');
    fireEvent.click(screen.getByRole('button', { name: 'Edit' }));

    const editor = screen.getByRole('textbox', { name: 'Edit proposed action' });
    expect(editor).toHaveValue('New booking request');
    fireEvent.change(editor, { target: { value: 'Confirm after calling the customer' } });
    fireEvent.click(screen.getByRole('button', { name: 'Save changes' }));

    await waitFor(() => {
      const request = vi.mocked(fetch).mock.calls[1]?.[1];
      const payload = JSON.parse(String(request?.body));
      expect(payload.state).toBe('PENDING_APPROVAL');
      expect(payload.context_payload.summary).toBe('Confirm after calling the customer');
    });
  });
  it('saves and approves the edited Ambassador draft when the owner chooses Save & Send', async () => {
    const ambassador = {
      ...bookingFeedItem,
      id: 'ambassador-edit-1', event_source: 'Ambassador',
      context_payload: { feature_type: 'ambassador_reply', original_message: 'Can you help?', source: 'SMS' },
      proposed_action: { feature_type: 'ambassador_reply', generated_response: 'Original reply' },
    };
    vi.stubGlobal('fetch', vi.fn()
      .mockResolvedValueOnce(new Response(JSON.stringify({ items: [ambassador] }), { status: 200 }))
      .mockResolvedValueOnce(new Response('{}', { status: 200 })));
    render(<FeedPage />);
    await screen.findByTestId('ambassador-reply-card');
    fireEvent.click(screen.getByTestId('feed-edit-btn'));
    fireEvent.change(screen.getByTestId('feed-edit-input'), { target: { value: 'Owner-reviewed reply' } });
    fireEvent.click(screen.getByTestId('feed-save-edit-btn'));
    await waitFor(() => {
      expect(screen.queryByTestId('ambassador-reply-card')).toBeNull();
    });
    const payload = JSON.parse(String(vi.mocked(fetch).mock.calls[1]?.[1]?.body));
    expect(payload.state).toBe('APPROVED');
    expect(payload.proposed_action.feature_type).toBe('ambassador_reply');
    expect(payload.proposed_action.generated_response).toBe('Owner-reviewed reply');
  });

});
