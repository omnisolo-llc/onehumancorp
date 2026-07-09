import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import React from 'react';
import CostDashboardPage from './page';
import { useRouter } from 'next/navigation';
import { expect, test, vi, describe, beforeEach, afterEach } from 'vitest';

// Mock next/navigation
let originalWindowLocation: any;
beforeEach(() => { originalWindowLocation = window.location; delete (window as any).location; window.location = { ...originalWindowLocation, href: '' } as any; });
afterEach(() => { window.location = originalWindowLocation; });
vi.mock('next/navigation', () => ({
  useRouter: vi.fn(),
  usePathname: () => '/cost-dashboard',
}));

vi.mock('../../components/TooltipRegistry', () => ({
  TooltipProvider: ({ children }: any) => children,
  WithTooltip: ({ children }: any) => children,
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
    vi.clearAllMocks();
    vi.restoreAllMocks();
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
      budget_health_alert: true,
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
      expect(screen.queryByTestId('cost-dashboard-loading')).toBeNull();
    });

    // My Plan assertions
    expect(screen.getByText('My Plan')).toBeDefined();
    expect(screen.getByText('Back to My Plan')).toBeDefined();
    expect(screen.getByText('Starter')).toBeDefined();
    // AI Actions Used: 150 / 1000. Text split.
    expect(screen.getAllByText(/150/)[0]).toBeDefined();
    expect(screen.getAllByText(/\/ 1000/)[0]).toBeDefined();
    // Storage Used
    expect(screen.getAllByText(/2 MB/)[0]).toBeDefined();
    // Next bill estimated
    expect(screen.getByText('$29.00')).toBeDefined(); // Since Next bill estimated uses formatCurrency which divides by 100

    expect(screen.getAllByText('Cost Transparency Dashboard')[0]).toBeDefined();
    expect(screen.getByText('Period: 2023-10-01 to 2023-10-31')).toBeDefined();

    // total revenue
    expect(screen.getByText('$1500.00')).toBeDefined();

    // total cost
    expect(screen.getByText('$510.00')).toBeDefined();

    // Budget Alert
    expect(screen.queryAllByText('Budget Alert').length).toBeGreaterThan(0);

    // projected monthly cost
    expect(screen.getByText('$2185.71')).toBeDefined();

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
    expect(screen.getByText('$8.00')).toBeDefined(); // 800 cents

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

  test('renders Budget Alert when threshold is crossed', async () => {
    const mockCostData = {
      total_revenue: 150000,
      budget_health_alert: true,
      total_costs: 200000,
      projected_monthly_cost: 200000,
      llm_cost: 180000,
      storage_cost: 0,
      payment_fees: 0,
      network_cost: 0,
      bandwidth_savings: 0,
      cache_hit_rate: 0,
      cost_per_1k_tokens: 0,
      period_start: "2023-10-01",
      period_end: "2023-10-31",
      trend: [],
      agent_costs: [],
      department_tier_usage: {
        departments: [
          {
            id: "dept-ops",
            department_type: "operations",
            agent_id: "operations_agent",
            actions_used: 16,
            action_limit: 20, // 16 / 20 = 0.8 (>= 0.8 threshold)
            usage_percent: 80,
            soft_limit_reached: false,
          }
        ],
      },
    };

    const mockPlanData = {
      current_plan: "Starter",
      ai_actions_used: 150,
      ai_actions_limit: 1000,
      storage_used_bytes: 0,
      storage_limit_bytes: 0,
      next_bill_estimated: 0,
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
      expect(screen.queryByTestId('cost-dashboard-loading')).toBeNull();
    });

    expect(screen.getAllByText('Budget Alert').length).toBeGreaterThan(0);
  });

  test('renders 0 limits properly', async () => {
    const mockCostData = {
      cost_per_1k_tokens: 0.0015,
      projected_monthly_cost: 0,
      trend: [],
      department_tier_usage: {
        departments: [],
      },
    };

    const mockPlanData = {
      current_plan: "Starter",
      ai_actions_used: 0,
      ai_actions_limit: 0,
      storage_used_bytes: 0,
      storage_limit_bytes: 0,
      next_bill_estimated: 0,
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
      expect(screen.queryByTestId('cost-dashboard-loading')).toBeNull();
    });

    expect(screen.getAllByText(/\/ Unlimited/)[0]).toBeDefined();
    expect(screen.getAllByText(/\/ Unlimited/)[1]).toBeDefined();
  });

  test('renders unlimited limits properly', async () => {
    const mockCostData = {
      cost_per_1k_tokens: 0.0015,
      projected_monthly_cost: 0,
      trend: [],
      department_tier_usage: {
        departments: [],
      },
    };

    const mockPlanData = {
      current_plan: "Pro",
      ai_actions_used: 10,
      ai_actions_limit: null,
      storage_used_bytes: 1000,
      storage_limit_bytes: null,
      next_bill_estimated: 0,
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
      expect(screen.queryByTestId('cost-dashboard-loading')).toBeNull();
    });

    expect(screen.getAllByText(/\/ Unlimited/)[0]).toBeDefined();
    expect(screen.getAllByText(/\/ Unlimited/).length).toBe(2);
  });

  test('handles manage billing portal correctly', async () => {
    const mockCostData = { cost_per_1k_tokens: 0, trend: [] };
    const mockPlanData = { current_plan: 'Starter' };
    const mockPortalUrl = 'https://billing.stripe.com/p/session/test_123';

    global.fetch = vi.fn().mockImplementation((url: string, options: any) => {
      if (url.includes('cost-dashboard')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockCostData) });
      if (url.includes('my-plan')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockPlanData) });
      if (url === '/api/billing/create-billing-portal-session' && options?.method === 'POST') {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ url: mockPortalUrl }) });
      }
      return Promise.reject(new Error('not found'));
    }) as any;

    render(<CostDashboardPage />);

    await waitFor(() => {
      expect(screen.queryByTestId('cost-dashboard-loading')).toBeNull();
    });

    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    const manageButton = screen.getByText('Manage Billing');
    await act(async () => {
      fireEvent.click(manageButton);
    });

    await waitFor(() => {
      expect(window.location.href).toBe(mockPortalUrl);
    });
  });

  test('handles manage billing portal error', async () => {
    const mockCostData = { cost_per_1k_tokens: 0, trend: [] };
    const mockPlanData = { current_plan: 'Starter' };

    global.fetch = vi.fn().mockImplementation((url: string, options: any) => {
      if (url.includes('cost-dashboard')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockCostData) });
      if (url.includes('my-plan')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockPlanData) });
      if (url === '/api/billing/create-billing-portal-session' && options?.method === 'POST') {
        return Promise.resolve({ ok: false, json: () => Promise.resolve({}) });
      }
      return Promise.reject(new Error('not found'));
    }) as any;

    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    render(<CostDashboardPage />);

    await waitFor(() => {
      expect(screen.queryByTestId('cost-dashboard-loading')).toBeNull();
    });

    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    const manageButton = screen.getByText('Manage Billing');
    await act(async () => {
      fireEvent.click(manageButton);
    });

    await waitFor(() => {
      expect(screen.getByText('Failed to initiate billing portal. Please try again.')).toBeDefined();
      consoleErrorSpy.mockRestore();

    });

    consoleSpy.mockRestore();
  });

  test('handles cancel subscription correctly', async () => {
    const mockCostData = { cost_per_1k_tokens: 0, trend: [] };
    const mockPlanData = { current_plan: 'Starter' };

    global.fetch = vi.fn().mockImplementation((url: string, options: any) => {
      if (url.includes('cost-dashboard')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockCostData) });
      if (url.includes('my-plan')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockPlanData) });
      if (url === '/api/billing/cancel-subscription' && options?.method === 'POST') {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({}) });
      }
      return Promise.reject(new Error('not found'));
    }) as any;

    const confirmSpy = vi.spyOn(window, 'confirm').mockImplementation(() => true);

    render(<CostDashboardPage />);

    await waitFor(() => {
      expect(screen.queryByTestId('cost-dashboard-loading')).toBeNull();
    });

    const cancelButton = screen.getByText('Cancel Subscription');
    await act(async () => {
      fireEvent.click(cancelButton);
    });

    await waitFor(() => {
      expect(screen.getByText('Subscription canceled successfully.')).toBeDefined();
    });

    confirmSpy.mockRestore();
  });

  test('handles cancel subscription error', async () => {
    const mockCostData = { cost_per_1k_tokens: 0, trend: [] };
    const mockPlanData = { current_plan: 'Starter' };

    global.fetch = vi.fn().mockImplementation((url: string, options: any) => {
      if (url.includes('cost-dashboard')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockCostData) });
      if (url.includes('my-plan')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockPlanData) });
      if (url === '/api/billing/cancel-subscription' && options?.method === 'POST') {
        return Promise.resolve({ ok: false, json: () => Promise.resolve({}) });
      }
      return Promise.reject(new Error('not found'));
    }) as any;

    const confirmSpy = vi.spyOn(window, 'confirm').mockImplementation(() => true);

    render(<CostDashboardPage />);

    await waitFor(() => {
      expect(screen.queryByTestId('cost-dashboard-loading')).toBeNull();
    });

    const cancelButton = screen.getByText('Cancel Subscription');
    await act(async () => {
      fireEvent.click(cancelButton);
    });

    await waitFor(() => {
      expect(screen.getByText('Failed to cancel subscription.')).toBeDefined();
    });

    confirmSpy.mockRestore();
  });

  test('handles cancel subscription catch error', async () => {
    const mockCostData = { cost_per_1k_tokens: 0, trend: [] };
    const mockPlanData = { current_plan: 'Starter' };

    global.fetch = vi.fn().mockImplementation((url: string, options: any) => {
      if (url.includes('cost-dashboard')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockCostData) });
      if (url.includes('my-plan')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockPlanData) });
      if (url === '/api/billing/cancel-subscription' && options?.method === 'POST') {
        return Promise.reject(new Error('Network err'));
      }
      return Promise.reject(new Error('not found'));
    }) as any;

    const confirmSpy = vi.spyOn(window, 'confirm').mockImplementation(() => true);

    render(<CostDashboardPage />);

    await waitFor(() => {
      expect(screen.queryByTestId('cost-dashboard-loading')).toBeNull();
    });

    const cancelButton = screen.getByText('Cancel Subscription');
    await act(async () => {
      fireEvent.click(cancelButton);
    });

    await waitFor(() => {
      expect(screen.getByText('Error canceling subscription.')).toBeDefined();
    });

    confirmSpy.mockRestore();
  });


  test('handles back to plan routing', async () => {
    const mockPush = vi.fn();
    (useRouter as any).mockReturnValue({ push: mockPush });

    const mockCostData = { cost_per_1k_tokens: 0, trend: [] };
    const mockPlanData = { current_plan: 'Starter' };

    global.fetch = vi.fn().mockImplementation((url: string, options: any) => {
      if (url.includes('cost-dashboard')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockCostData) });
      if (url.includes('my-plan')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockPlanData) });
      return Promise.reject(new Error('not found'));
    }) as any;

    render(<CostDashboardPage />);

    await waitFor(() => {
      expect(screen.queryByTestId('cost-dashboard-loading')).toBeNull();
    });

    const backButton = screen.getByRole('link', { name: /Back to My Plan/i });
    expect(backButton.getAttribute('href')).toBe('/plan');
  });

  test('handles download invoice correctly', async () => {
    const mockCostData = { cost_per_1k_tokens: 0, trend: [] };
    const mockPlanData = { current_plan: 'Starter' };
    const mockInvoiceUrl = 'https://billing.stripe.com/invoice/test_123';

    global.fetch = vi.fn().mockImplementation((url: string, options: any) => {
      if (url.includes('cost-dashboard')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockCostData) });
      if (url.includes('my-plan')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockPlanData) });
      if (url === '/api/billing/download-invoice' && options?.method === 'POST') {
        return Promise.resolve({ ok: true, json: () => Promise.resolve({ url: mockInvoiceUrl }) });
      }
      return Promise.reject(new Error('not found'));
    }) as any;

    const openSpy = vi.spyOn(window, 'open').mockImplementation(() => null);

    render(<CostDashboardPage />);

    await waitFor(() => {
      expect(screen.queryByTestId('cost-dashboard-loading')).toBeNull();
    });

    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    const invoiceButton = screen.getByText('Download Invoice');
    await act(async () => {
      fireEvent.click(invoiceButton);
    });

    await waitFor(() => {
      expect(openSpy).toHaveBeenCalledWith(mockInvoiceUrl, '_blank');
      expect(screen.getByText('Invoice download is ready for your current billing period.')).toBeDefined();
    });

    openSpy.mockRestore();
  });

  test('handles download invoice error', async () => {
    const mockCostData = { cost_per_1k_tokens: 0, trend: [] };
    const mockPlanData = { current_plan: 'Starter' };

    global.fetch = vi.fn().mockImplementation((url: string, options: any) => {
      if (url.includes('cost-dashboard')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockCostData) });
      if (url.includes('my-plan')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockPlanData) });
      if (url === '/api/billing/download-invoice' && options?.method === 'POST') {
        return Promise.resolve({ ok: false, json: () => Promise.resolve({}) });
      }
      return Promise.reject(new Error('not found'));
    }) as any;

    render(<CostDashboardPage />);

    await waitFor(() => {
      expect(screen.queryByTestId('cost-dashboard-loading')).toBeNull();
    });

    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    const invoiceButton = screen.getByText('Download Invoice');
    await act(async () => {
      fireEvent.click(invoiceButton);
    });

    await waitFor(() => {
      expect(screen.getByText('Failed to download invoice.')).toBeDefined();
      consoleErrorSpy.mockRestore();
    });
  });

  test('handles download invoice catch error', async () => {
    const mockCostData = { cost_per_1k_tokens: 0, trend: [] };
    const mockPlanData = { current_plan: 'Starter' };
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    global.fetch = vi.fn().mockImplementation((url: string, options: any) => {
      if (url.includes('cost-dashboard')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockCostData) });
      if (url.includes('my-plan')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockPlanData) });
      if (url === '/api/billing/download-invoice' && options?.method === 'POST') {
        return Promise.reject(new Error('Network err'));
      }
      return Promise.reject(new Error('not found'));
    }) as any;

    render(<CostDashboardPage />);

    await waitFor(() => {
      expect(screen.queryByTestId('cost-dashboard-loading')).toBeNull();
    });

    const invoiceButton = screen.getByText('Download Invoice');
    await act(async () => {
      fireEvent.click(invoiceButton);
    });

    await waitFor(() => {
      expect(screen.getByText('Error downloading invoice.')).toBeDefined();
    });
  });
  test('handles cost data fetch catch error', async () => {
    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url.includes('cost-dashboard')) return Promise.reject(new Error('Network err'));
      if (url.includes('my-plan')) return Promise.resolve({ ok: true, json: () => Promise.resolve({ current_plan: 'Starter' }) });
      return Promise.reject(new Error('not found'));
    }) as any;

    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    render(<CostDashboardPage />);

    await waitFor(() => {
      expect(screen.queryByTestId('cost-dashboard-loading')).toBeNull();
    });

    expect(consoleSpy).toHaveBeenCalledWith("Error fetching cost data", expect.any(Error));
    consoleSpy.mockRestore();
  });
  test('handles upgrade routing', async () => {
    const mockPush = vi.fn();
    (useRouter as any).mockReturnValue({ push: mockPush });

    const mockCostData = { cost_per_1k_tokens: 0, trend: [] };
    const mockPlanData = { current_plan: 'Starter' };
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    global.fetch = vi.fn().mockImplementation((url: string, options: any) => {
      if (url.includes('cost-dashboard')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockCostData) });
      if (url.includes('my-plan')) return Promise.resolve({ ok: true, json: () => Promise.resolve(mockPlanData) });
      return Promise.reject(new Error('not found'));
    }) as any;

    render(<CostDashboardPage />);

    await waitFor(() => {
      expect(screen.queryByTestId('cost-dashboard-loading')).toBeNull();
    });

    const upgradeButton = screen.getByRole('button', { name: /Upgrade/i });
    await act(async () => {
      fireEvent.click(upgradeButton);
    });

    expect(mockPush).toHaveBeenCalledWith('/pricing');
  });
});
