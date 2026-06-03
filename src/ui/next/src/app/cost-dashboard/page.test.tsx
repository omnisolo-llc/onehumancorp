import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import CostDashboardPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

const mockFetch = vi.fn();
global.fetch = mockFetch;

describe('CostDashboardPage', () => {
  it('renders loading state initially', () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: () => Promise.resolve({}),
    });
    render(<CostDashboardPage />);
    expect(screen.getByText('Loading...')).toBeDefined();
  });

  it('renders cost details after fetch', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: () => Promise.resolve({
        total_revenue: 150000,
        total_costs: 25000,
        llm_cost: 5000,
        storage_cost: 1500,
        payment_fees: 4350,
        network_cost: 200,
        bandwidth_savings: 50,
        period_start: '2024-05-01',
        period_end: '2024-05-31',
      }),
    });

    render(<CostDashboardPage />);

    await waitFor(() => {
      expect(screen.queryByText('Loading...')).toBeNull();
    });

    expect(screen.getByText('$1500.00')).toBeDefined(); // Revenue
    expect(screen.getByText('$250.00')).toBeDefined(); // Total costs
    expect(screen.getByText('$50.00')).toBeDefined(); // LLM cost
    expect(screen.getByText('-$0.50')).toBeDefined(); // Bandwidth savings
  });
});
