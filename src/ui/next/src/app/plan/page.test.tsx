import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import MyPlanPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

const mockFetch = vi.fn();
global.fetch = mockFetch;

describe('MyPlanPage', () => {
  it('renders loading state initially', () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: () => Promise.resolve({
        current_plan: 'Free',
        ai_actions_used: 50,
        ai_actions_limit: 100,
        storage_used_bytes: 1024 * 1024 * 10,
        storage_limit_bytes: 1024 * 1024 * 500,
        next_bill_estimated: 0,
      }),
    });
    render(<MyPlanPage />);
    expect(screen.getByText('Loading...')).toBeDefined();
  });

  it('renders plan details after fetch', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: () => Promise.resolve({
        current_plan: 'Starter',
        ai_actions_used: 150,
        ai_actions_limit: 1000,
        storage_used_bytes: 1024 * 1024 * 500, // 500MB
        storage_limit_bytes: 1024 * 1024 * 5000, // 5GB (technically 4.88GB in base 1024)
        next_bill_estimated: 29,
      }),
    });

    render(<MyPlanPage />);

    await waitFor(() => {
      expect(screen.queryByText('Loading...')).toBeNull();
    });

    expect(screen.getByText('Starter')).toBeDefined();
    expect(screen.getByText('$29.00')).toBeDefined();
    expect(screen.getByText('150 / 1000')).toBeDefined();
    expect(screen.getByText('500.0 MB / 4.88 GB')).toBeDefined();
  });
});
