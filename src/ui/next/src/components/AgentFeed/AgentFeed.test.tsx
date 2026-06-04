import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { AgentFeed } from './AgentFeed';
import { vi, describe, it, expect, beforeEach } from 'vitest';

// Mock fetch
const mockFetch = vi.fn();
global.fetch = mockFetch;

describe('AgentFeed', () => {
  beforeEach(() => {
    mockFetch.mockClear();
    vi.spyOn(Storage.prototype, 'getItem').mockImplementation((key) => {
      if (key === 'tenant_id') return 'test-tenant';
      return null;
    });
  });

  it('renders loading state initially', () => {
    mockFetch.mockReturnValue(new Promise(() => {})); // Never resolves
    render(<AgentFeed />);
    expect(screen.getByText(/Synchronizing with Agents/i)).toBeInTheDocument();
  });

  it('renders empty state when no proposals', async () => {
    mockFetch.mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ pending_approvals: [] }),
    });

    render(<AgentFeed />);

    const emptyMsg = await screen.findByText(/All Clear/i);
    expect(emptyMsg).toBeInTheDocument();
  });

  it('renders proposals when available', async () => {
    mockFetch.mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        pending_approvals: [
          {
            id: '1',
            department: 'Marketing',
            description: 'Test Proposal',
            action_risk: 'LOW',
            status: 'PendingApproval',
          }
        ]
      }),
    });

    render(<AgentFeed />);

    const proposal = await screen.findByText('Test Proposal');
    expect(proposal).toBeInTheDocument();
    expect(screen.getByText('Marketing')).toBeInTheDocument();
  });

  it('handles approval action', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: () => Promise.resolve({
        pending_approvals: [
          {
            id: '1',
            department: 'Marketing',
            description: 'Test Proposal',
            action_risk: 'LOW',
            status: 'PendingApproval',
          }
        ]
      }),
    }).mockResolvedValueOnce({
      ok: true,
      json: () => Promise.resolve({ success: true }),
    });

    render(<AgentFeed />);

    const approveBtn = await screen.findByRole('button', { name: /Approve proposal/i });
    fireEvent.click(approveBtn);

    // Should optimismistically remove
    await waitFor(() => {
        expect(screen.queryByText('Test Proposal')).not.toBeInTheDocument();
    });

    expect(mockFetch).toHaveBeenCalledWith(
      expect.stringContaining('/api/agents/approvals/1'),
      expect.objectContaining({ method: 'POST' })
    );
  });

  it('handles decline action', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: () => Promise.resolve({
        pending_approvals: [
          {
            id: '1',
            department: 'Operations',
            description: 'Decline me',
            action_risk: 'LOW',
            status: 'PendingApproval',
          }
        ]
      }),
    }).mockResolvedValueOnce({
      ok: true,
      json: () => Promise.resolve({ success: true }),
    });

    render(<AgentFeed />);

    const declineBtn = await screen.findByRole('button', { name: /Decline proposal/i });
    fireEvent.click(declineBtn);

    await waitFor(() => {
        expect(screen.queryByText('Decline me')).not.toBeInTheDocument();
    });

    expect(mockFetch).toHaveBeenCalledWith(
      expect.stringContaining('/api/agents/approvals/1'),
      expect.objectContaining({
          method: 'POST',
          body: JSON.stringify({ approved: false })
      })
    );
  });

  it('renders multiple proposals and maintains count', async () => {
    mockFetch.mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        pending_approvals: [
          { id: '1', department: 'Marketing', description: 'P1', action_risk: 'LOW', status: 'PendingApproval' },
          { id: '2', department: 'Finance', description: 'P2', action_risk: 'HIGH', status: 'PendingApproval' }
        ]
      }),
    });

    render(<AgentFeed />);

    expect(await screen.findByText('P1')).toBeInTheDocument();
    expect(screen.getByText('P2')).toBeInTheDocument();
    expect(screen.getByText('2 Items')).toBeInTheDocument();
  });

  it('renders error state on fetch failure', async () => {
    mockFetch.mockResolvedValue({
      ok: false,
      status: 500
    });

    render(<AgentFeed />);

    const errorMsg = await screen.findByText(/Failed to load agent proposals/i);
    expect(errorMsg).toBeInTheDocument();
  });
});
