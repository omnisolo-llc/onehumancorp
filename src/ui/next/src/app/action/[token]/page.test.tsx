import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { expect, test, vi, describe, beforeEach } from 'vitest';
import ActionApprovalPage from './page';

// Mock useRouter
const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
}));

describe('ActionApprovalPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn() as any;
  });

  test('renders loading initially and then shows mock action', async () => {
    render(<ActionApprovalPage params={{ token: 'valid-token' }} />);

    // Initially should be loading
    expect(screen.queryByText(/Sales Agent/i)).not.toBeInTheDocument();

    // After 1 second mock load, should show data
    await waitFor(() => {
      expect(screen.getByText(/Sales Agent/i)).toBeInTheDocument();
      expect(screen.getByText(/Quote Approval/i)).toBeInTheDocument();
      expect(screen.getByText(/Leaking pipe repair/i)).toBeInTheDocument();
    }, { timeout: 1500 });
  });

  test('handles approve action correctly', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ message: 'Approved successfully' })
    });

    render(<ActionApprovalPage params={{ token: 'valid-token' }} />);

    // Wait for load
    await waitFor(() => {
      expect(screen.getByText(/Approve & Send/i)).toBeInTheDocument();
    }, { timeout: 1500 });

    const approveBtn = screen.getByText(/Approve & Send/i);
    fireEvent.click(approveBtn);

    // Verify fetch
    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/agents/action', expect.any(Object));
      expect(screen.getByText(/Done/i)).toBeInTheDocument();
      expect(screen.getByText(/Approved successfully/i)).toBeInTheDocument();
    });

    // Test Dashboard redirect
    const dashBtn = screen.getByText(/Go to Dashboard/i);
    fireEvent.click(dashBtn);
    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });

  test('shows error state for invalid token', async () => {
    render(<ActionApprovalPage params={{ token: '' }} />);

    await waitFor(() => {
      expect(screen.getByText(/Error/i)).toBeInTheDocument();
      expect(screen.getByText(/Invalid or expired action token/i)).toBeInTheDocument();
    }, { timeout: 1500 });
  });
});
