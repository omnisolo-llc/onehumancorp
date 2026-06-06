import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { AgentFeedWidget } from './AgentFeedWidget';

const mockFetch = jest.fn();
global.fetch = mockFetch;

describe('AgentFeedWidget', () => {
  beforeEach(() => {
    mockFetch.mockClear();
    // Mock localStorage
    Storage.prototype.getItem = jest.fn(() => 'test-tenant');
  });

  it('renders loading state initially', async () => {
    mockFetch.mockImplementationOnce(() => new Promise(() => {})); // Never resolves
    render(<AgentFeedWidget />);
    expect(screen.getByText('Loading Agent Feed...')).toBeInTheDocument();
  });

  it('renders empty state when no actions', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ pending_actions: [] }),
    });

    render(<AgentFeedWidget />);

    await waitFor(() => {
      expect(screen.getByText('All caught up!')).toBeInTheDocument();
    });
  });

  it('renders action cards and handles dismissal', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        pending_actions: [
          {
            id: '1',
            tenant_id: 'test-tenant',
            agent_id: 'agent-1',
            action_type: 'draft_email',
            payload: { description: 'Drafted an email for 3 abandoned carts' },
            status: 'pending'
          }
        ],
      }),
    });

    render(<AgentFeedWidget />);

    await waitFor(() => {
      expect(screen.getByText('Drafted an email for 3 abandoned carts')).toBeInTheDocument();
    });

    const dismissBtn = screen.getByText('Dismiss');
    fireEvent.click(dismissBtn);

    await waitFor(() => {
      expect(screen.queryByText('Drafted an email for 3 abandoned carts')).not.toBeInTheDocument();
    });
  });
});
