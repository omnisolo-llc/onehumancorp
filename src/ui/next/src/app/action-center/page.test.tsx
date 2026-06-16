import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ActionCenterPage from './page';
import * as React from 'react';

// Mock components
vi.mock('../components/AppShell', () => ({
  AppShell: ({ children, title }: { children: React.ReactNode; title: string }) => (
    <div data-testid="app-shell" data-title={title}>
      {children}
    </div>
  ),
}));

describe('ActionCenterPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        pending_approvals: [
          {
            id: '1',
            department: 'business_advisory',
            description: 'Test Recommendation | Payload: {"context": {"summary": "A test summary", "actionable_suggestion": "Do this action"}}',
            status: 'pending',
            action_risk: 'low'
          },
          {
             id: '2',
             department: 'The Advisor',
             description: 'Pricing Update | Payload: {"context": {"smart_pricing": true, "product_name": "Cake", "new_price": 50, "old_price": 40, "sales_projection": "+20%"}}',
             status: 'pending',
             action_risk: 'medium'
          }
        ]
      })
    } as any);

    Object.defineProperty(window, 'localStorage', {
      value: {
        getItem: vi.fn(() => 'fake-token'),
      },
      writable: true
    });
  });

  it('renders loading state initially', () => {
    // We defer the fetch resolution to ensure we can inspect the loading state
    global.fetch = vi.fn().mockImplementation(() => {
      return new Promise(() => {}); // never resolves to keep it loading
    });

    let container: HTMLElement | undefined;
    React.act(() => {
      const result = render(<ActionCenterPage />);
      container = result.container;
    });

    // We check for loading synchronously before fetch completes
    expect(container?.querySelector('.animate-spin')).not.toBeNull();
  });

  it('renders approvals after fetching', async () => {
    render(<ActionCenterPage />);
    await waitFor(() => {
      expect(screen.getByText('Test Recommendation')).toBeInTheDocument();
      expect(screen.getByText('Pricing Update')).toBeInTheDocument();
    });
  });

  it('displays parsed payload details correctly for promo', async () => {
    render(<ActionCenterPage />);
    await waitFor(() => {
      expect(screen.getByText('A test summary')).toBeInTheDocument();
      expect(screen.getByText('Do this action')).toBeInTheDocument();
    });
  });

  it('displays parsed payload details correctly for smart pricing', async () => {
    render(<ActionCenterPage />);
    await waitFor(() => {
      expect(screen.getByText('Cake')).toBeInTheDocument();
      expect(screen.getByText('$50 (was $40)')).toBeInTheDocument();
      expect(screen.getByText('+20%')).toBeInTheDocument();
    });
  });

  it('handles approve action', async () => {
    render(<ActionCenterPage />);
    await waitFor(() => {
      expect(screen.getByText('Test Recommendation')).toBeDefined();
    });

    const approveButtons = screen.getAllByText('Approve & Send');

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
    } as any);

    fireEvent.click(approveButtons[0]);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/agents/approvals/1', expect.objectContaining({
        method: 'POST',
        body: JSON.stringify({ approved: true })
      }));
      expect(screen.getByText('Action approved and executed..')).toBeDefined();
    });
  });

  it('handles dismiss action', async () => {
    render(<ActionCenterPage />);
    await waitFor(() => {
      expect(screen.getByText('Pricing Update')).toBeDefined();
    });

    const dismissButtons = screen.getAllByText('Dismiss');

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
    } as any);

    fireEvent.click(dismissButtons[1]);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/agents/approvals/2', expect.objectContaining({
        method: 'POST',
        body: JSON.stringify({ approved: false })
      }));
      expect(screen.getByText('Action dismissed.')).toBeDefined();
    });
  });
});
