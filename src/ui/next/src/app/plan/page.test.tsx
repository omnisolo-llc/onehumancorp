import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import MyPlanPage from './page';

const mockPush = vi.fn();

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
}));

describe('MyPlanPage', () => {
  beforeEach(() => {
    mockPush.mockClear();
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        current_plan: 'Starter',
        ai_actions_used: 100,
        ai_actions_limit: 1000,
        storage_used_bytes: 1024 * 1024 * 50,
        storage_limit_bytes: 1024 * 1024 * 1024 * 5,
        next_bill_estimated: 29
      })
    } as any);
  });

  it('renders loading state initially', () => {
    render(<MyPlanPage />);
    expect(screen.getByText('Loading...')).toBeDefined();
  });

  it('renders data correctly after fetch', async () => {
    render(<MyPlanPage />);

    await waitFor(() => {
      expect(screen.queryByText('Loading...')).toBeNull();
    });

    expect(screen.getByText('My Plan')).toBeDefined();
    expect(screen.getByText('Starter')).toBeDefined();
    expect(screen.getByText('$29.00')).toBeDefined();

    // Usage data
    expect(screen.getByText('100 / 1000')).toBeDefined();
    expect(screen.getByText('50.0 MB / 5.00 GB')).toBeDefined();
  });

  it('handles navigation', async () => {
    render(<MyPlanPage />);

    await waitFor(() => {
      expect(screen.queryByText('Loading...')).toBeNull();
    });

    const costDetailsBtn = screen.getByRole('heading', { name: 'View Cost Details' });
    fireEvent.click(costDetailsBtn);
    expect(mockPush).toHaveBeenCalledWith('/cost-dashboard');

    const changePlanBtn = screen.getByRole('heading', { name: 'Change Plan' });
    fireEvent.click(changePlanBtn);
    expect(mockPush).toHaveBeenCalledWith('/pricing');
  });
});
