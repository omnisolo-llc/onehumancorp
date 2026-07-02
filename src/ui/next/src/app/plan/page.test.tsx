import React from 'react';
import { render, screen, fireEvent, act, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import MyPlanPage from './page';
import { useRouter } from 'next/navigation';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(),
}));

vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</>,
}));

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('MyPlanPage', () => {
  const mockPush = vi.fn();
  let originalWindowLocation: any;

  beforeEach(() => {
    vi.clearAllMocks();
    (useRouter as any).mockReturnValue({ push: mockPush });
    global.fetch = vi.fn();

    (global.fetch as any).mockImplementation(async (url) => {
      if (url === '/api/billing/my-plan') {
        return {
          ok: true,
          json: async () => ({
            current_plan: 'Starter',
            ai_actions_used: 150,
            ai_actions_limit: 1000,
            storage_used_bytes: 2 * 1024 * 1024, // 2MB
            storage_limit_bytes: 5 * 1024 * 1024 * 1024, // 5GB
            next_bill_estimated: 2900,
            soft_limit_reached: false,
            user_message: "You've reached your Free tier limit of 100 AI actions. Upgrade to unlock more power!",
          }),
        };
      }
      return { ok: true, json: async () => ({}) };
    });

    originalWindowLocation = window.location;
    delete (window as any).location;
    window.location = { ...originalWindowLocation, href: '' } as any;
  });

  afterEach(() => {
    window.location = originalWindowLocation;
  });

  it('renders the plan page', async () => {
    await act(async () => {
      render(<MyPlanPage />);
    });

    expect(screen.getByText('My Plan')).toBeDefined();
    expect(screen.getByText('Starter')).toBeDefined();
    expect(screen.getByText('$29.00')).toBeDefined();
  });

  it('renders soft limit reached message', async () => {
    (global.fetch as any).mockImplementation(async (url) => {
      if (url === '/api/billing/my-plan') {
        return {
          ok: true,
          json: async () => ({
            current_plan: 'Starter',
            ai_actions_used: 1000,
            ai_actions_limit: 1000,
            storage_used_bytes: 2 * 1024 * 1024, // 2MB
            storage_limit_bytes: 5 * 1024 * 1024 * 1024, // 5GB
            next_bill_estimated: 2900,
            soft_limit_reached: true,
            user_message: "You've reached your Starter tier limit of 1000 AI actions. Upgrade to unlock more power!",
          }),
        };
      }
      return { ok: true, json: async () => ({}) };
    });

    await act(async () => {
      render(<MyPlanPage />);
    });

    expect(screen.getByText("You've reached your Starter tier limit of 1000 AI actions. Upgrade to unlock more power!")).toBeDefined();
  });

  it('navigates to pricing on upgrade click', async () => {
    await act(async () => {
      render(<MyPlanPage />);
    });

    const upgradeButton = screen.getByText('Upgrade');
    await act(async () => {
      fireEvent.click(upgradeButton);
    });
    expect(mockPush).toHaveBeenCalledWith('/pricing');
  });

  it('renders unlimited limits properly for 0 limits', async () => {
    (global.fetch as any).mockImplementation(async (url) => {
      if (url === '/api/billing/my-plan') {
        return {
          ok: true,
          json: async () => ({
            current_plan: 'Business',
            ai_actions_used: 150,
            ai_actions_limit: 0,
            storage_used_bytes: 2 * 1024 * 1024, // 2MB
            storage_limit_bytes: 0,
            next_bill_estimated: 29900,
          }),
        };
      }
      return { ok: true, json: async () => ({}) };
    });

    await act(async () => {
      render(<MyPlanPage />);
    });

    // Check if it renders '/ Unlimited' for both limits
    const unlimitedTexts = screen.getAllByText(/\/ Unlimited/);
    expect(unlimitedTexts.length).toBeGreaterThan(0);
  });

  it('navigates to cost-dashboard on details click', async () => {
    await act(async () => {
      render(<MyPlanPage />);
    });

    const detailsButton = screen.getByText('View Detailed Costs');
    await act(async () => {
      fireEvent.click(detailsButton);
    });
    expect(mockPush).toHaveBeenCalledWith('/cost-dashboard');
  });

  it('initiates manage billing flow', async () => {
    const mockPortalUrl = 'https://billing.stripe.com/p/session/test_123';
    (global.fetch as any).mockImplementation(async (url, options) => {
      if (url === '/api/billing/my-plan') {
        return {
          ok: true,
          json: async () => ({ current_plan: 'Starter' }),
        };
      }
      if (url === '/api/billing/create-billing-portal-session' && options?.method === 'POST') {
        return {
          ok: true,
          json: async () => ({ url: mockPortalUrl }),
        };
      }
      return { ok: true, json: async () => ({}) };
    });

    let resolvePortal: any;
    const portalPromise = new Promise((resolve) => {
      resolvePortal = resolve;
    });

    (global.fetch as any).mockImplementation(async (url, options) => {
      if (url === '/api/billing/my-plan') {
        return {
          ok: true,
          json: async () => ({ current_plan: 'Starter' }),
        };
      }
      if (url === '/api/billing/create-billing-portal-session' && options?.method === 'POST') {
        return portalPromise.then(() => ({
          ok: true,
          json: async () => ({ url: mockPortalUrl }),
        }));
      }
      return { ok: true, json: async () => ({}) };
    });

    await act(async () => {
      render(<MyPlanPage />);
    });

    const manageButton = screen.getByText('Manage Billing');

    // Fire click but don't await the full act yet to check intermediate state
    fireEvent.click(manageButton);

    // Verify it changes to loading state
    expect(screen.getByText('Redirecting...')).toBeDefined();

    // Resolve the mock fetch promise
    await act(async () => {
      resolvePortal();
    });

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/billing/create-billing-portal-session', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
      });
      expect(window.location.href).toBe(mockPortalUrl);
    });
  });
});
