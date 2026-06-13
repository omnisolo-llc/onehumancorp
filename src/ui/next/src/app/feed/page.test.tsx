import { describe, it, expect, vi } from 'vitest';
import { render, screen, waitFor, act } from '@testing-library/react';
import AgentWorkFeedPage from './page';

// Mock fetch to return some dummy items for testing
global.fetch = vi.fn().mockImplementation((url) => {
  if (url === '/api/agent-feed') {
    return Promise.resolve({
      ok: true,
      json: () => Promise.resolve({
        items: [
          {
            id: 'inquiry1',
            tenant_id: 't1',
            event_source: 'CUSTOMER_INQUIRY',
            context_payload: { customerName: 'Maya', message: 'Hi, do you have any vegan options?' },
            lifecycle_state: 'PENDING',
            created_at: new Date().toISOString(),
          },
          {
            id: 'approval1',
            tenant_id: 't1',
            event_source: 'QUOTE_APPROVAL',
            proposed_action: { title: 'Kitchen Sink Repair' },
            context_payload: { customerName: 'Carlos', amount: 500 },
            lifecycle_state: 'PENDING',
            created_at: new Date().toISOString(),
          },
          {
            id: 'summary1',
            tenant_id: 't1',
            event_source: 'DAILY_SUMMARY',
            context_payload: { summary: 'Sales were up 12% yesterday.' },
            lifecycle_state: 'PENDING',
            created_at: new Date().toISOString(),
          }
        ]
      })
    });
  }
  return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
});

// Mock alert
vi.stubGlobal('alert', vi.fn());

describe('AgentWorkFeedPage', () => {
  it('renders the feed header and bottom nav', async () => {
    await act(async () => {
      render(<AgentWorkFeedPage />);
    });
    expect(screen.getByText('Assistant')).toBeInTheDocument();
    expect(screen.getByText('Feed')).toBeInTheDocument();
    expect(screen.getByText('Customers')).toBeInTheDocument();
    expect(screen.getByText('Ops')).toBeInTheDocument();
    expect(screen.getByText('Money')).toBeInTheDocument();
  });

  it('renders the agent message and cards from mock backend', async () => {
    await act(async () => {
      render(<AgentWorkFeedPage />);
    });

    // Wait for fetch to complete
    await waitFor(() => {
      expect(screen.queryByText('Loading feed...')).not.toBeInTheDocument();
    });

    // Check generic message
    expect(screen.getByText(/Good morning! You have new updates/)).toBeInTheDocument();

    // Check specific cards
    expect(screen.getByText('Maya')).toBeInTheDocument();
    expect(screen.getByText(/"Hi, do you have any vegan options\?"/)).toBeInTheDocument();

    expect(screen.getByText(/Carlos/)).toBeInTheDocument();
    expect(screen.getByText('Kitchen Sink Repair')).toBeInTheDocument();
    expect(screen.getByText('$500.00')).toBeInTheDocument();

    expect(screen.getByText('Sales were up 12% yesterday.')).toBeInTheDocument();
  });
});
