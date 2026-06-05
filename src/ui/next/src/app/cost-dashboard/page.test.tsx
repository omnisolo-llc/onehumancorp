import { render, screen, waitFor } from '@testing-library/react';
import React from 'react';
import CostDashboardPage from './page';
import { useRouter } from 'next/navigation';
import { expect, test, vi, describe, beforeEach, afterEach } from 'vitest';

// Mock next/navigation
vi.mock('next/navigation', () => ({
  useRouter: vi.fn()
}));

const mockPush = vi.fn();

describe('CostDashboardPage', () => {
  beforeEach(() => {
    (useRouter as any).mockReturnValue({
      push: mockPush,
    });

    // Clear mocks
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  test('renders loading state initially', () => {
    // Mock fetch to not resolve immediately
    global.fetch = vi.fn(() => new Promise(() => {})) as any;

    render(<CostDashboardPage />);
    expect(screen.getByText('Loading...')).toBeDefined();
  });

  test('renders cost data after fetch', async () => {
    const mockData = {
      total_revenue: 150000,
      total_costs: 51000,
      llm_cost: 20000,
      storage_cost: 10000,
      payment_fees: 5000,
      network_cost: 16000,
      bandwidth_savings: 5000,
      period_start: "2023-10-01",
      period_end: "2023-10-31"
    };

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: vi.fn().mockResolvedValue(mockData)
    }) as any;

    render(<CostDashboardPage />);

    await waitFor(() => {
      expect(screen.queryByText('Loading...')).toBeNull();
    });

    expect(screen.getByText('Cost Transparency')).toBeDefined();
    expect(screen.getByText('Period: 2023-10-01 to 2023-10-31')).toBeDefined();

    // total revenue
    expect(screen.getByText('$1500.00')).toBeDefined();

    // total cost
    expect(screen.getByText('$510.00')).toBeDefined();

    // Specific cost breakdowns
    expect(screen.getByText('$200.00')).toBeDefined(); // llm
    expect(screen.getByText('$100.00')).toBeDefined(); // storage
    expect(screen.getByText('$50.00')).toBeDefined(); // payment fees
    expect(screen.getByText('$160.00')).toBeDefined(); // network
    expect(screen.getByText('-$50.00')).toBeDefined(); // bandwidth savings
  });

  test('handles fetch error gracefully', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 500
    }) as any;

    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    render(<CostDashboardPage />);

    await waitFor(() => {
      expect(screen.queryByText('Loading...')).toBeNull();
    });

    expect(consoleSpy).toHaveBeenCalledWith("Failed to fetch cost data:", 500);

    // Data is null, formatting should return $0.00
    const zeroElements = screen.getAllByText('$0.00');
    expect(zeroElements.length).toBeGreaterThan(0);
  });
});
