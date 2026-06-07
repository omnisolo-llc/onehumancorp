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
    const mockCostData = {
      total_revenue: 150000,
      total_costs: 51000,
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

    const mockPlanData = {
      current_plan: "Starter",
      ai_actions_used: 150,
      ai_actions_limit: 1000,
      storage_used_bytes: 2 * 1024 * 1024,
      storage_limit_bytes: 5 * 1024 * 1024 * 1024,
      next_bill_estimated: 2900,
    };

    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url.includes('cost-dashboard')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve(mockCostData)
        });
      } else if (url.includes('my-plan')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve(mockPlanData)
        });
      }
      return Promise.reject(new Error('not found'));
    }) as any;

    render(<CostDashboardPage />);

    await waitFor(() => {
      expect(screen.queryByText('Loading...')).toBeNull();
    });

    // My Plan assertions
    expect(screen.getByText('My Plan')).toBeDefined();
    expect(screen.getByText('Starter')).toBeDefined();
    // AI actions used: 150 / 1000. Text split.
    expect(screen.getAllByText(/150/)[0]).toBeDefined();
    expect(screen.getAllByText(/\/ 1000/)[0]).toBeDefined();
    // Storage used
    expect(screen.getByText(/2.0 MB/)).toBeDefined();
    expect(screen.getByText(/\/ 5120 MB/)).toBeDefined();
    // Next bill estimated
    expect(screen.getByText('$29.00')).toBeDefined();

    expect(screen.getByText('Cost Transparency')).toBeDefined();
    expect(screen.getByText('Period: 2023-10-01 to 2023-10-31')).toBeDefined();

    // total revenue
    expect(screen.getByText('$1500.00')).toBeDefined();

    // total cost
    expect(screen.getByText('$510.00')).toBeDefined();

    // Specific cost breakdowns
    expect(screen.getByText('$200.00')).toBeDefined(); // llm
    expect(screen.getByText('Efficiency: 85.5% cache hit rate, $0.0015/1k tokens')).toBeDefined(); // llm efficiency
    expect(screen.getByText('$100.00')).toBeDefined(); // storage
    expect(screen.getAllByText('$50.00').length).toBeGreaterThan(0); // payment fees
    expect(screen.getByText('$160.00')).toBeDefined(); // network
    expect(screen.getAllByText('-$50.00').length).toBeGreaterThan(0); // bandwidth savings

    // 7-Day Trend
    expect(screen.getByText('7-Day Trend')).toBeDefined();
    expect(screen.getByText('2023-10-01')).toBeDefined();
    expect(screen.getByText('$10.00')).toBeDefined(); // 1000 cents
    expect(screen.getByText('2023-10-02')).toBeDefined();
    expect(screen.getByText('$15.00')).toBeDefined(); // 1500 cents

    expect(screen.getByText('Department Tier Usage')).toBeDefined();
    expect(screen.getByText('marketing')).toBeDefined();
    expect(screen.getByText('12 / 20 actions')).toBeDefined();
    expect(screen.getByText('operations')).toBeDefined();
    expect(screen.getByText('Tier limit reached')).toBeDefined();
  });

  test('renders / Unlimited when limits are null', async () => {
    const mockCostData = {
      total_revenue: 150000,
      total_costs: 51000,
      llm_cost: 20000,
      storage_cost: 10000,
      payment_fees: 5000,
      network_cost: 16000,
      bandwidth_savings: 5000,
      cache_hit_rate: 85.5,
      cost_per_1k_tokens: 0.0015,
      period_start: "2023-10-01",
      period_end: "2023-10-31",
    };

    const mockPlanData = {
      current_plan: "Pro",
      ai_actions_used: 150,
      ai_actions_limit: null,
      storage_used_bytes: 2 * 1024 * 1024,
      storage_limit_bytes: null,
      next_bill_estimated: 7900,
    };

    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url.includes('cost-dashboard')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve(mockCostData)
        });
      } else if (url.includes('my-plan')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve(mockPlanData)
        });
      }
      return Promise.reject(new Error('not found'));
    }) as any;

    render(<CostDashboardPage />);

    await waitFor(() => {
      expect(screen.queryByText('Loading...')).toBeNull();
    });

    const unlimitedElements = screen.getAllByText('/ Unlimited');
    expect(unlimitedElements.length).toBe(2);
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
      expect(screen.queryByText('Loading...')).toBeNull();
    });

    expect(consoleSpy).toHaveBeenCalledWith("Failed to fetch cost data:", 500);

    // Data is null, formatting should return $0.00
    const zeroElements = screen.getAllByText('$0.00');
    expect(zeroElements.length).toBeGreaterThan(0);
  });
});
