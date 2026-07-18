import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import CustomerMemoryGraph from './page';

// Mock the next/navigation hooks
vi.mock('next/navigation', () => ({
  useSearchParams: () => new URLSearchParams('customerId=test-customer&tenantId=test-tenant'),
}));

// Mock the PoweredByOHC component since we're focused on CustomerMemoryGraph
vi.mock('@/app/components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('CustomerMemoryGraph Component', () => {
  it('renders loading state initially', () => {
    render(<CustomerMemoryGraph />);
    expect(screen.getByText('Loading customer history...')).toBeInTheDocument();
  });

  it('renders error state on fetch failure', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
    });

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

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockData),
    });

    render(<CustomerMemoryGraph />);

    await waitFor(() => {
      // Check for main headers
      expect(screen.getByText('Assistant\'s Memory')).toBeInTheDocument();
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
      expect(screen.getByText('Draft Reply')).toBeInTheDocument();
      expect(screen.getByText('Issue Refund')).toBeInTheDocument();
    });
  });

  it('renders empty state when no events exist', async () => {
    const mockData = {
      total_interactions: 0,
      segments: [],
      events: [],
    };

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockData),
    });

    render(<CustomerMemoryGraph />);

    await waitFor(() => {
      expect(screen.getByText('No interaction history found.')).toBeInTheDocument();
    });
  });
});
