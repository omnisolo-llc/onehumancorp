import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import CustomerMemoryGraph from './page';

// Mock the next/navigation hooks
vi.mock('next/navigation', () => ({
  useSearchParams: () => new URLSearchParams('customerId=test-customer&tenantId=test-tenant'),
}));

// Mock the PoweredByOmniSolo component since we're focused on CustomerMemoryGraph
vi.mock('@/app/components/PoweredByOmniSolo', () => ({
  PoweredByOmniSolo: () => <div data-testid="powered-by-omnisolo" />,
}));

describe('CustomerMemoryGraph Component', () => {
  it.each([null, { events: {} }, { customer_name: 42 }, { events: [{ raw_content: {} }] }])(
    'rejects malformed history without inventing customer facts: %j', async (payload) => {
      vi.stubGlobal('fetch', vi.fn<typeof fetch>(async () => Response.json(payload)));
      render(<CustomerMemoryGraph />);
      expect(await screen.findByText('Failed to fetch customer history.')).toBeVisible();
      expect(screen.queryByText('High Intent')).not.toBeInTheDocument();
    },
  );

  it('renders loading state initially', () => {
    vi.stubGlobal('fetch', vi.fn<typeof fetch>(() => new Promise<Response>(() => {})));
    const view = render(<CustomerMemoryGraph />);
    expect(screen.getByText('Loading customer history...')).toBeInTheDocument();
    view.unmount();
  });

  it('renders error state on fetch failure', async () => {
    vi.stubGlobal('fetch', vi.fn<typeof fetch>(async () => new Response(null, { status: 503 })));

    render(<CustomerMemoryGraph />);

    await waitFor(() => {
      expect(screen.getByText('Failed to fetch customer history.')).toBeInTheDocument();
    });
  });

  it('renders correctly with interaction events data', async () => {
    const mockData = {
      total_interactions: 2,
      segments: ['VIP', 'Frequent Buyer'],
      events: [
        {
          id: '1',
          channel: 'pos',
          raw_content: 'Bought in store: Summer Dress',
          created_at: new Date('2023-01-01T10:00:00Z').toISOString(),
        },
        {
          id: '2',
          channel: 'instagram',
          raw_content: 'Sent DM: Do you have vegan cakes?',
          created_at: new Date('2023-01-02T15:30:00Z').toISOString(),
        },
      ],
    };

    vi.stubGlobal('fetch', vi.fn<typeof fetch>(async () => Response.json(mockData)));

    render(<CustomerMemoryGraph />);

    await waitFor(() => {
      // Check for main headers
      expect(screen.getByText('Customer Context')).toBeInTheDocument();
      expect(screen.getByText('Timeline')).toBeInTheDocument();

      // Check for AI insights segments
      expect(screen.getByText('VIP')).toBeInTheDocument();
      expect(screen.getByText('Frequent Buyer')).toBeInTheDocument();
      expect(screen.getByText('2 total interactions recorded.')).toBeInTheDocument();

      // Check for specific events
      expect(screen.getByText('Bought in store: Summer Dress')).toBeInTheDocument();
      expect(screen.getByText('Sent DM: Do you have vegan cakes?')).toBeInTheDocument();

      // Check for channels
      expect(screen.getByText('pos')).toBeInTheDocument();
      expect(screen.getByText('instagram')).toBeInTheDocument();

      // Check for action buttons
      expect(screen.getByRole('button', { name: 'Draft Reply' })).toBeDisabled();
      expect(screen.getByRole('button', { name: 'Issue Refund' })).toBeDisabled();
      expect(screen.queryByText('High Intent')).not.toBeInTheDocument();
    });
  });

  it('renders empty state when no events exist', async () => {
    const mockData = {
      total_interactions: 0,
      segments: [],
      events: [],
    };

    vi.stubGlobal('fetch', vi.fn<typeof fetch>(async () => Response.json(mockData)));

    render(<CustomerMemoryGraph />);

    await waitFor(() => {
      expect(screen.getByText('No interaction history found.')).toBeInTheDocument();
    });
  });
});
