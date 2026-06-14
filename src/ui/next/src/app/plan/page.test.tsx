import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import MyPlanPage from './page';
import { useRouter } from 'next/navigation';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(),
}));

vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</>,
}));

describe('MyPlanPage', () => {
  const mockPush = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    (useRouter as any).mockReturnValue({ push: mockPush });
    global.fetch = vi.fn();
  });

  it('renders loading state initially', () => {
    // We intentionally don't resolve the fetch immediately so the component is stuck in loading
    (global.fetch as any).mockReturnValue(new Promise(() => {}));
    render(<MyPlanPage />);
    expect(screen.getByText('Loading your plan data...')).toBeDefined();
  });

  it('renders plan data after loading', async () => {
    const mockData = {
      current_plan: 'Starter',
      ai_actions_used: 150,
      ai_actions_limit: 1000,
      storage_used_bytes: 1024 * 1024 * 1024 * 1.5, // 1.5 GB
      storage_limit_bytes: 1024 * 1024 * 1024 * 5, // 5 GB
      next_bill_estimated: 29.00
    };

    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: async () => mockData,
    });

    render(<MyPlanPage />);

    await waitFor(() => {
      expect(screen.getByText(/Starter/)).toBeDefined();
      expect(screen.getByText(/\$29.00/)).toBeDefined();
      expect(screen.getByText(/150/)).toBeDefined();
      expect(screen.getByText(/\/ 1000/)).toBeDefined();
      expect(screen.getByText(/1.50 GB/)).toBeDefined();
      expect(screen.getByText(/\/ 5.00 GB/)).toBeDefined();
    });
  });

  it('navigates back to dashboard', async () => {
    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: async () => ({
        current_plan: 'Free',
        ai_actions_used: 0,
        ai_actions_limit: 100,
        storage_used_bytes: 0,
        storage_limit_bytes: 500 * 1024 * 1024,
        next_bill_estimated: 0
      }),
    });

    render(<MyPlanPage />);

    await waitFor(() => {
      expect(screen.getByText('Back')).toBeDefined();
    });

    fireEvent.click(screen.getByText('Back'));
    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });

  it('navigates to pricing page on upgrade', async () => {
    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: async () => ({
        current_plan: 'Free',
        ai_actions_used: 0,
        ai_actions_limit: 100,
        storage_used_bytes: 0,
        storage_limit_bytes: 500 * 1024 * 1024,
        next_bill_estimated: 0
      }),
    });

    render(<MyPlanPage />);

    await waitFor(() => {
      expect(screen.getByText('View Upgrade Plans')).toBeDefined();
    });

    fireEvent.click(screen.getByText('View Upgrade Plans'));
    expect(mockPush).toHaveBeenCalledWith('/pricing');
  });

  it('navigates to cost dashboard', async () => {
    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: async () => ({
        current_plan: 'Free',
        ai_actions_used: 0,
        ai_actions_limit: 100,
        storage_used_bytes: 0,
        storage_limit_bytes: 500 * 1024 * 1024,
        next_bill_estimated: 0
      }),
    });

    render(<MyPlanPage />);

    await waitFor(() => {
      expect(screen.getByText('View Detailed Costs')).toBeDefined();
    });

    fireEvent.click(screen.getByText('View Detailed Costs'));
    expect(mockPush).toHaveBeenCalledWith('/cost-dashboard');
  });
});
