import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { expect, test, vi, describe, beforeEach } from 'vitest';
import MyPlanPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('MyPlanPage', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  test('renders loading state initially', () => {
    // Mock fetch to not resolve immediately
    global.fetch = vi.fn(() => new Promise(() => {})) as any;

    render(<MyPlanPage />);
    expect(screen.getByTestId('plan-loading')).toBeDefined();
  });

  test('renders plan data after fetch', async () => {
    const mockPlanData = {
      current_plan: "Starter",
      ai_actions_used: 150,
      ai_actions_limit: 1000,
      storage_used_bytes: 2 * 1024 * 1024,
      storage_limit_bytes: 5 * 1024 * 1024 * 1024,
      next_bill_estimated: 2900,
    };

    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url.includes('my-plan')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve(mockPlanData)
        });
      }
      return Promise.reject(new Error('not found'));
    }) as any;

    render(<MyPlanPage />);

    await waitFor(() => {
      expect(screen.queryByTestId('plan-loading')).toBeNull();
    });

    expect(screen.getByText('My Plan')).toBeDefined();
    expect(screen.getByText('Starter')).toBeDefined();

    // AI actions used: 150 / 1000. Text split.
    expect(screen.getAllByText(/150/)[0]).toBeDefined();
    expect(screen.getAllByText(/\/ 1000/)[0]).toBeDefined();

    // Storage used
    expect(screen.getByText(/2 MB/)).toBeDefined();

    // Next bill estimated
    expect(screen.getByText('$29.00')).toBeDefined();
  });

  test('handles fetch error gracefully', async () => {
    global.fetch = vi.fn().mockImplementation(() => {
      return Promise.resolve({
        ok: false,
        status: 500
      });
    }) as any;

    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    render(<MyPlanPage />);

    await waitFor(() => {
      expect(screen.queryByTestId('plan-loading')).toBeNull();
    });

    expect(consoleSpy).toHaveBeenCalledWith("Failed to fetch plan data:", 500);

    expect(screen.getByText('Free')).toBeDefined();
    expect(screen.getByText('$0.00')).toBeDefined();
  });
});
