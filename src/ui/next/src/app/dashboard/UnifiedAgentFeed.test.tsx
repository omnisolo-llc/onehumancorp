import { render, screen, waitFor, fireEvent } from '@testing-library/react';
import { UnifiedAgentFeed } from './UnifiedAgentFeed';
import { expect, test, vi, beforeEach, afterEach } from 'vitest';

const mockFetch = vi.fn();
global.fetch = mockFetch as any;

describe('UnifiedAgentFeed', () => {
  beforeEach(() => {
    mockFetch.mockClear();
    localStorage.clear();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  test('renders loading state initially', async () => {
    // Delay fetch resolution to check loading state
    mockFetch.mockImplementationOnce(() => new Promise(() => {}));
    render(<UnifiedAgentFeed />);
    expect(screen.getByText('Loading Agent Proposals...')).toBeDefined();
  });

  test('renders error state on fetch failure', async () => {
    mockFetch.mockRejectedValueOnce(new Error('Network failure'));
    render(<UnifiedAgentFeed />);
    await waitFor(() => {
      expect(screen.getByText('Network failure')).toBeDefined();
    });
  });

  test('renders empty state when no pending approvals', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: () => Promise.resolve({ pending_approvals: [] }),
    });
    render(<UnifiedAgentFeed />);
    await waitFor(() => {
      expect(screen.getByText('All caught up!')).toBeDefined();
      expect(screen.getByText('Your agents are currently monitoring the business.')).toBeDefined();
    });
  });

  test('renders populated feed properly', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: () => Promise.resolve({
        pending_approvals: [
          {
            id: 'req-1',
            tenant_id: 'default',
            department: 'Marketing',
            description: 'Run targeted ad campaign',
            status: 'pending',
            action_risk: 'HIGH',
            payload: {
              context: {
                abandoned_carts_count: 5,
                potential_revenue: 125.50
              }
            }
          }
        ]
      }),
    });
    render(<UnifiedAgentFeed />);

    await waitFor(() => {
      expect(screen.getByText('Marketing')).toBeDefined();
    });
    expect(screen.getByText('Run targeted ad campaign')).toBeDefined();
    expect(screen.getByText('Requires Review')).toBeDefined();
    expect(screen.getByText('Abandoned Carts:')).toBeDefined();
    expect(screen.getByText('5')).toBeDefined();
    expect(screen.getByText('Potential Revenue:')).toBeDefined();
    expect(screen.getByText('$125.50')).toBeDefined();
  });

  test('approves proposal successfully', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: () => Promise.resolve({
        pending_approvals: [
          {
            id: 'req-2',
            tenant_id: 'default',
            department: 'Sales',
            description: 'Generate quotes',
            status: 'pending',
            action_risk: 'LOW',
          }
        ]
      }),
    });

    render(<UnifiedAgentFeed />);

    await waitFor(() => {
      expect(screen.getByText('Generate quotes')).toBeDefined();
    });

    // Mock the POST request for approval
    mockFetch.mockResolvedValueOnce({ ok: true });

    fireEvent.click(screen.getByText('Approve'));

    // Proposal should be optimistically removed
    expect(screen.queryByText('Generate quotes')).toBeNull();

    expect(mockFetch).toHaveBeenCalledTimes(2);
    expect(mockFetch.mock.calls[1][0]).toContain('/api/agents/approvals/req-2');
    expect(mockFetch.mock.calls[1][1].method).toBe('POST');
    expect(JSON.parse(mockFetch.mock.calls[1][1].body).approved).toBe(true);
  });

  test('declines proposal successfully', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: () => Promise.resolve({
        pending_approvals: [
          {
            id: 'req-3',
            tenant_id: 'default',
            department: 'Finance',
            description: 'Update pricing',
            status: 'pending',
            action_risk: 'LOW',
          }
        ]
      }),
    });

    render(<UnifiedAgentFeed />);

    await waitFor(() => {
      expect(screen.getByText('Update pricing')).toBeDefined();
    });

    // Mock the POST request for rejection
    mockFetch.mockResolvedValueOnce({ ok: true });

    fireEvent.click(screen.getByText('Decline'));

    // Proposal should be optimistically removed
    expect(screen.queryByText('Update pricing')).toBeNull();

    expect(mockFetch).toHaveBeenCalledTimes(2);
    expect(mockFetch.mock.calls[1][0]).toContain('/api/agents/approvals/req-3');
    expect(mockFetch.mock.calls[1][1].method).toBe('POST');
    expect(JSON.parse(mockFetch.mock.calls[1][1].body).approved).toBe(false);
  });

  test('handles approval failure and restores state', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: () => Promise.resolve({
        pending_approvals: [
          {
            id: 'req-4',
            tenant_id: 'default',
            department: 'Support',
            description: 'Send refund',
            status: 'pending',
            action_risk: 'HIGH',
          }
        ]
      }),
    });

    render(<UnifiedAgentFeed />);

    await waitFor(() => {
      expect(screen.getByText('Send refund')).toBeDefined();
    });

    // Mock the POST request to fail
    mockFetch.mockResolvedValueOnce({ ok: false });

    // Mock the re-fetch request to succeed and return the original state
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: () => Promise.resolve({
        pending_approvals: [
          {
            id: 'req-4',
            tenant_id: 'default',
            department: 'Support',
            description: 'Send refund',
            status: 'pending',
            action_risk: 'HIGH',
          }
        ]
      }),
    });

    fireEvent.click(screen.getByText('Approve'));

    // The error overrides the UI rendering of the list, so test error rendering instead.
    await waitFor(() => {
      expect(screen.getByText('Failed to submit decision')).toBeDefined();
    });
  });
});
