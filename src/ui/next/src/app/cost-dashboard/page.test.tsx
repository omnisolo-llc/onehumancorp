import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { vi } from 'vitest';
import CostDashboardPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn().mockReturnValue({
    push: vi.fn(),
  }),
}));

const mockData = {
  total_revenue: 50000,
  total_costs: 20000,
  llm_cost: 15000,
  storage_cost: 4000,
  payment_fees: 1000,
  period_start: "2024-05-01",
  period_end: "2024-05-31",
};

describe('CostDashboardPage', () => {
  let consoleErrorSpy: any;

  beforeEach(() => {
    vi.resetAllMocks();
    consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
  });

  afterEach(() => {
    consoleErrorSpy.mockRestore();
  });

  it('renders loading initially', () => {
    global.fetch = vi.fn(() => new Promise(() => {})) as any;
    render(<CostDashboardPage />);
    expect(screen.getByText('Loading...')).toBeInTheDocument();
  });

  it('fetches and displays cost data', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockData),
    });

    render(<CostDashboardPage />);

    await waitFor(() => {
        expect(screen.queryByText('Loading...')).not.toBeInTheDocument();
    });

    expect(screen.getByText('Cost Transparency Dashboard')).toBeInTheDocument();
    expect(screen.getByText('Period: 2024-05-01 to 2024-05-31')).toBeInTheDocument();
    expect(screen.getByText('$500.00')).toBeInTheDocument(); // revenue
    expect(screen.getByText('$200.00')).toBeInTheDocument(); // total cost
    expect(screen.getByText('$150.00')).toBeInTheDocument(); // llm
    expect(screen.getByText('$40.00')).toBeInTheDocument(); // storage
    expect(screen.getByText('$10.00')).toBeInTheDocument(); // payment fees
  });

  it('displays fallback data on fetch error', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 500,
    });

    render(<CostDashboardPage />);

    await waitFor(() => {
        expect(screen.queryByText('Loading...')).not.toBeInTheDocument();
    });

    expect(screen.getByText('Cost Transparency Dashboard')).toBeInTheDocument();
    // It should display $0.00 for fallback values
    const zeroValues = screen.getAllByText('$0.00');
    expect(zeroValues.length).toBe(5); // total, revenue, llm, storage, payment
  });
});
