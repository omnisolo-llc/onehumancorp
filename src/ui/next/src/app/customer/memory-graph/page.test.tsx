import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import { act } from 'react';
import CustomerMemoryGraph from './page';

// Mock the next/navigation hooks
vi.mock('next/navigation', () => ({
  useSearchParams: () => new URLSearchParams('customerId=test-customer&tenantId=test-tenant'),
}));

// Mock the PoweredByOHC component since we're focused on CustomerMemoryGraph
vi.mock('../../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('CustomerMemoryGraph Component', () => {
  it('renders loading state initially', () => {
    act(() => { render(<CustomerMemoryGraph />); });
    expect(screen.getByText('Loading customer history...')).toBeDefined();
  });

  it('renders error state on fetch failure', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
    });

    act(() => { render(<CustomerMemoryGraph />); });

    await waitFor(() => {
      expect(screen.getByText('Failed to fetch customer history.')).toBeDefined();
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

    act(() => { render(<CustomerMemoryGraph />); });

    await waitFor(() => {
      // Check for main headers
      expect(screen.getByText('Customer Context')).toBeDefined();
      expect(screen.getByText('Timeline')).toBeDefined();

      // Check for AI insights segments
      expect(screen.getByText('VIP')).toBeDefined();
      expect(screen.getByText('Frequent Buyer')).toBeDefined();
      expect(screen.getByText('2 total interactions recorded.')).toBeDefined();

      // Check for specific events
      expect(screen.getByText('Bought in store: Summer Dress')).toBeDefined();
      expect(screen.getByText('Sent DM: Do you have vegan cakes?')).toBeDefined();

      // Check for channels
      expect(screen.getByText('pos')).toBeDefined();
      expect(screen.getByText('instagram')).toBeDefined();

      // Check for action buttons
      expect(screen.getByText('Draft Reply')).toBeDefined();
      expect(screen.getByText('Issue Refund')).toBeDefined();
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

    act(() => { render(<CustomerMemoryGraph />); });

    await waitFor(() => {
      expect(screen.getByText('No interaction history found.')).toBeDefined();
    });
  });
});
