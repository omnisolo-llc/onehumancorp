import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import MyPlanPage from './page';

const mockPush = vi.fn();
const mockRefresh = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush, refresh: mockRefresh }),
}));

vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: any) => <>{children}</>,
}));

describe('MyPlanPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        current_plan: 'Starter',
        ai_actions_used: 10,
        ai_actions_limit: 1000,
        storage_used_bytes: 1024 * 1024,
        storage_limit_bytes: 5 * 1024 * 1024 * 1024,
        next_bill_estimated: 2900,
      }),
    } as any);
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('renders loading state initially', async () => {
    render(<MyPlanPage />);
    expect(screen.getByText('Loading your plan data...')).toBeDefined();
    await waitFor(() => expect(screen.queryByText('Loading your plan data...')).toBeNull());
  });

  it('renders plan data after loading', async () => {
    render(<MyPlanPage />);
    await waitFor(() => expect(screen.queryByText('Loading your plan data...')).toBeNull());
    expect(screen.getByText('Starter')).toBeDefined();
    expect(screen.getByText('$2,900.00')).toBeDefined();
  });

  it('navigates to pricing when Upgrade is clicked', async () => {
    render(<MyPlanPage />);
    await waitFor(() => expect(screen.queryByText('Loading your plan data...')).toBeNull());

    const upgradeBtn = screen.getByRole('button', { name: 'Upgrade' });
    fireEvent.click(upgradeBtn);
    expect(mockPush).toHaveBeenCalledWith('/pricing');
  });

  it('navigates to cost-dashboard when View Detailed Costs is clicked', async () => {
    render(<MyPlanPage />);
    await waitFor(() => expect(screen.queryByText('Loading your plan data...')).toBeNull());

    const viewCostsBtn = screen.getByRole('button', { name: 'View Detailed Costs' });
    fireEvent.click(viewCostsBtn);
    expect(mockPush).toHaveBeenCalledWith('/cost-dashboard');
  });

  it('shows cancel modal and calls cancel API when confirmed', async () => {
    const mockCancelResponse = { ok: true, json: () => Promise.resolve({}) };
    global.fetch = vi.fn().mockImplementation((url) => {
        if (url === '/api/billing/cancel-subscription') {
            return Promise.resolve(mockCancelResponse as any);
        }
        return Promise.resolve({
            ok: true,
            json: () => Promise.resolve({ current_plan: 'Starter' }),
        } as any);
    });

    render(<MyPlanPage />);
    await waitFor(() => expect(screen.queryByText('Loading your plan data...')).toBeNull());

    const openModalBtn = screen.getByRole('button', { name: 'Cancel Subscription' });
    fireEvent.click(openModalBtn);

    const confirmBtn = screen.getByRole('button', { name: 'Confirm Cancel' });
    fireEvent.click(confirmBtn);

    await waitFor(() => {
        expect(global.fetch).toHaveBeenCalledWith('/api/billing/cancel-subscription', expect.objectContaining({ method: 'POST' }));
    });

    await waitFor(() => expect(screen.getByText('Subscription canceled successfully.')).toBeDefined());
  });
});
