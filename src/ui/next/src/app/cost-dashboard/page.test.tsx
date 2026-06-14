import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { expect, test, vi, describe, beforeEach } from 'vitest';
import CostDashboardPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('CostDashboardPage', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  test('renders loading state initially', () => {
    // Mock fetch to not resolve immediately
    global.fetch = vi.fn(() => new Promise(() => {})) as any;

    render(<CostDashboardPage />);
    expect(screen.getByTestId('cost-dashboard-loading')).toBeDefined();
  });

  test('renders cost data after fetch', async () => {
    const mockCostData = {
      total_revenue: 150000,
      total_costs: 51000,
      projected_monthly_cost: 218571,
      llm_cost: 20000,
      storage_cost: 10000,
      payment_fees: 5000,
      network_cost: 16000,
      bandwidth_savings: 5000,
      cache_hit_rate: 85.5,
      cost_per_1k_tokens: 0.0015,
      period_start: "2023-10-01",
      period_end: "2023-10-31",
      trend: [
        { date: "2023-10-01", total_cost: 1000, llm_cost: 500, storage_cost: 200, network_cost: 100, compute_cost: 200 },
        { date: "2023-10-02", total_cost: 1500, llm_cost: 800, storage_cost: 200, network_cost: 200, compute_cost: 300 }
      ],
      agent_costs: [
        { agent_id: "marketing_agent", cost_cents: 1200 },
        { agent_id: "sales_agent", cost_cents: 800 }
      ],
      department_tier_usage: {
        current_plan: "Free",
        period: "2023-10",
        departments: [
          {
            id: "dept-marketing",
            department_type: "marketing",
            agent_id: "marketing_agent",
            actions_used: 12,
            action_limit: 20,
            usage_percent: 60,
            soft_limit_reached: false,
          },
          {
            id: "dept-ops",
            department_type: "operations",
            agent_id: "operations_agent",
            actions_used: 21,
            action_limit: 20,
            usage_percent: 100,
            soft_limit_reached: true,
          },
        ],
      },
    };

    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url.includes('cost-dashboard')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve(mockCostData)
        });
      }
      return Promise.reject(new Error('not found'));
    }) as any;

    render(<CostDashboardPage />);

    await waitFor(() => {
      expect(screen.queryByTestId('cost-dashboard-loading')).toBeNull();
    });

    expect(screen.getByText('Cost Transparency')).toBeDefined();

    // Using a more flexible matcher since the text is broken up by elements in page.tsx:
    // <span id="cost-dashboard-period" className="text-sm text-gray-500 font-medium">Period: {data?.period_start} to {data?.period_end}</span>
    expect(screen.getByText((content, element) => {
        return element?.textContent === 'Period: 2023-10-01 to 2023-10-31';
    })).toBeDefined();

    // total revenue
    expect(screen.getByText('$1,500.00')).toBeDefined();

    // total cost
    expect(screen.getByText('$510.00')).toBeDefined();

    // Budget Alert
    expect(screen.queryByText('Budget Health Warning')).not.toBeNull(); // Operations department usage reaches 100%

    // projected monthly cost
    expect(screen.getByText('$2,185.71')).toBeDefined();

    // Specific cost breakdowns
    expect(screen.getByText('$200.00')).toBeDefined(); // llm
    expect(screen.getByText('Efficiency: 85.5% cache hit rate, $0.0015/1k tokens')).toBeDefined(); // llm efficiency
    expect(screen.getByText('$100.00')).toBeDefined(); // storage
    expect(screen.getAllByText('$50.00').length).toBeGreaterThan(0); // payment fees
    expect(screen.getByText('$160.00')).toBeDefined(); // network
    expect(screen.getAllByText('-$50.00').length).toBeGreaterThan(0); // bandwidth savings

    // Agent & Feature Costs
    expect(screen.getByText('Agent & Feature Costs')).toBeDefined();
    expect(screen.getByText('marketing agent')).toBeDefined();
    expect(screen.getByText('$12.00')).toBeDefined(); // 1200 cents
    expect(screen.getByText('sales agent')).toBeDefined();
    expect(screen.getAllByText('$8.00')[0]).toBeDefined(); // 800 cents

    // 7-Day Trend
    expect(screen.getByText('7-Day Trend')).toBeDefined();
    expect(screen.getByText('10/01')).toBeDefined();
    expect(screen.getByText('$10.00')).toBeDefined(); // 1000 cents
    expect(screen.getByText('10/02')).toBeDefined();
    expect(screen.getByText('$15.00')).toBeDefined(); // 1500 cents

    expect(screen.getByText('Department Tier Usage')).toBeDefined();
    expect(screen.getByText('marketing')).toBeDefined();
    expect(screen.getByText('12 / 20 actions')).toBeDefined();
    expect(screen.getByText('operations')).toBeDefined();
    expect(screen.getByText('Tier limit reached')).toBeDefined();
  });

  test('handles fetch error gracefully', async () => {
    global.fetch = vi.fn().mockImplementation(() => {
      return Promise.resolve({
        ok: false,
        status: 500
      });
    }) as any;

    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    render(<CostDashboardPage />);

    await waitFor(() => {
      expect(screen.queryByTestId('cost-dashboard-loading')).toBeNull();
    });

    expect(consoleSpy).toHaveBeenCalledWith("Failed to fetch cost data:", 500);

    // Data is null, formatting should return $0.00
    const zeroElements = screen.getAllByText('$0.00');
    expect(zeroElements.length).toBeGreaterThan(0);
  });
});
