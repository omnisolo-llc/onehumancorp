import React from 'react';
import { render, screen, fireEvent, act, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import PricingPage from './page';
import { useRouter } from 'next/navigation';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(),
}));

vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</>,
}));

vi.mock('../components/PoweredByOmniSolo', () => ({
  PoweredByOmniSolo: () => <div data-testid="powered-by-omnisolo" />,
}));

vi.mock('../components/ViralTrialExtensionWidget', () => ({
  ViralTrialExtensionWidget: () => <div data-testid="viral-trial-extension-widget" />,
}));

describe('PricingPage', () => {
  const mockPush = vi.fn();

  let originalWindowLocation: Location;

  beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(useRouter, { partial: true }).mockReturnValue({ push: mockPush });
    global.fetch = vi.fn();

    vi.mocked(global.fetch, { partial: true }).mockImplementation(async (url) => {
      if (url === '/api/v1/billing/my-plan') {
        return {
          ok: true,
          status: 200,
          json: async () => ({ current_plan: 'Free' }),
        };
      }
      return { ok: true, json: async () => ({}) };
    });

    // Mock window.location.href
    originalWindowLocation = window.location;
    Object.defineProperty(window, 'location', {
      configurable: true, writable: true, value: { ...originalWindowLocation, href: '' },
    });
  });

  afterEach(() => {
    Object.defineProperty(window, 'location', {
      configurable: true, writable: true, value: originalWindowLocation,
    });
  });

  it('renders the pricing page', async () => {
    await act(async () => {
      render(<PricingPage />);
    });
    expect(screen.getByText('Pricing Plans')).toBeDefined();
    expect(screen.getByText('Free')).toBeDefined();
    expect(screen.getByText('Starter')).toBeDefined();
    expect(screen.getByText('Pro')).toBeDefined();
    expect(screen.getByText('Business')).toBeDefined();
  });

  it('does not invent usage, unlimited limits or a bill when those fields are absent', async () => {
    await act(async () => { render(<PricingPage />); });
    expect(screen.getByText('AI Actions Used').parentElement).toHaveTextContent('Unknown / Unknown');
    expect(screen.getByText('Storage Used').parentElement).toHaveTextContent('Unknown / Unknown');
    expect(screen.getByText('Estimated Next Bill').parentElement).toHaveTextContent('Unknown');
    expect(screen.getByText('Estimated Next Bill').parentElement).not.toHaveTextContent('$0.00');
  });

  it('preserves explicit zero usage and limits rather than converting zero into unlimited', async () => {
    vi.mocked(fetch).mockResolvedValue(Response.json({ current_plan: 'Free', ai_actions_used: 0, ai_actions_limit: 0,
      storage_used_bytes: 0, storage_limit_bytes: 0, next_bill_estimated: 0 }));
    await act(async () => { render(<PricingPage />); });
    expect(screen.getByText('AI Actions Used').parentElement).toHaveTextContent('0 / 0');
    expect(screen.getByText('Storage Used').parentElement).toHaveTextContent('0.0 MB / 0 MB');
    expect(screen.getByText('Estimated Next Bill').parentElement).toHaveTextContent('$0.00');
  });

  it('shows unlimited only for explicit null limits from a verified plan response', async () => {
    vi.mocked(fetch).mockResolvedValue(Response.json({ current_plan: 'Pro', ai_actions_used: 2, ai_actions_limit: null,
      storage_used_bytes: 1048576, storage_limit_bytes: null, next_bill_estimated: 7900 }));
    await act(async () => { render(<PricingPage />); });
    expect(screen.getByText('AI Actions Used').parentElement).toHaveTextContent('2 / Unlimited');
    expect(screen.getByText('Storage Used').parentElement).toHaveTextContent('1.0 MB / Unlimited');
    expect(screen.getByText('Estimated Next Bill').parentElement).toHaveTextContent('$79.00');
  });

  it.each([{}, { current_plan: 'Unknown' }, { current_plan: 'Pro', success: false }, { current_plan: 'Pro', error: 'unavailable' }])(
    'does not assume a Free or paid plan from an invalid response: %j', async body => {
      vi.mocked(fetch).mockResolvedValue(Response.json(body));
      await act(async () => { render(<PricingPage />); });
      expect(screen.getByRole('heading', { name: 'My Plan: Unavailable' })).toBeVisible();
      expect(screen.getByRole('button', { name: 'Manage Plan & Billing' })).toBeDisabled();
      expect(screen.getAllByRole('button', { name: 'Plan unavailable' })).toHaveLength(4);
      expect(screen.getByRole('link', { name: 'Retry plan lookup' })).toHaveAttribute('href', '/pricing');
    },
  );

  it('initiates checkout session when upgrading to Starter', async () => {
    const mockCheckoutUrl = 'https://checkout.stripe.com/pay/test_session_123';
    vi.mocked(global.fetch, { partial: true }).mockImplementation(async (url: string, options?: RequestInit) => {
      if (url === '/api/v1/billing/my-plan') {
        return {
          ok: true,
          status: 200,
          json: async () => ({ current_plan: 'Free' }),
        };
      }
      if (url.includes('/api/v1/billing/create-checkout-session') && options?.method === 'POST') {
        return {
          ok: true,
          status: 200,
          json: async () => ({ checkout_url: mockCheckoutUrl }),
        };
      }
      return { ok: true, json: async () => ({}) };
    });


    await act(async () => {
      render(<PricingPage />);
    });

    let upgradeButton;
    await waitFor(() => {
       upgradeButton = screen.getByText('Upgrade to Starter via Stripe');
    });

    await act(async () => {
       fireEvent.click(upgradeButton!);
    });

    // Wait for the async logic to finish
    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/v1/billing/create-checkout-session', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ tier: 'Starter', is_subscription: true, subscription_interval: 'month' }),
      });
      expect(window.location.href).toBe(mockCheckoutUrl);
    });
  });

  it('handles upgrade errors gracefully', async () => {
    vi.mocked(global.fetch, { partial: true }).mockImplementation(async (url: string, options?: RequestInit) => {
      if (url === '/api/v1/billing/my-plan') {
        return {
          ok: true,
          status: 200,
          json: async () => ({ current_plan: 'Free' }),
        };
      }
      if (url.includes('/api/v1/billing/create-checkout-session') && options?.method === 'POST') {
        throw new Error('Network error');
      }
      return { ok: true, json: async () => ({}) };
    });

    const alertMock = vi.spyOn(window, 'alert').mockImplementation(() => {});
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    await act(async () => {
      render(<PricingPage />);
    });

    let upgradeButton;
    await waitFor(() => {
       upgradeButton = screen.getByText('Upgrade to Starter via Stripe');
    });

    await act(async () => {
       fireEvent.click(upgradeButton!);
    });

    await waitFor(() => {
      expect(screen.getByRole('alert')).toHaveTextContent('Checkout is unavailable. Your plan has not changed. Please try again.');
      expect(window.location.href).toBe('');
      expect(alertMock).not.toHaveBeenCalled();
    });

    alertMock.mockRestore();
    consoleSpy.mockRestore();
  });

  it('renders the PoweredByOmniSolo component', async () => {
    await act(async () => {
      render(<PricingPage />);
    });
    expect(screen.getByTestId('powered-by-omnisolo')).toBeDefined();
  });

  it('renders the ViralTrialExtensionWidget when plan is Free', async () => {
    await act(async () => {
      render(<PricingPage />);
    });
    expect(screen.getByTestId('viral-trial-extension-widget')).toBeDefined();
  });

  it('renders the FAQ section with Stripe Billing integration info', async () => {
    await act(async () => {
      render(<PricingPage />);
    });
    expect(screen.getByText(/Stripe Billing for self-serve plan upgrades, downgrades, and cancellation/)).toBeDefined();
  });

  it('initiates billing portal session for manage billing', async () => {
    const mockPortalUrl = 'https://billing.stripe.com/p/session/test_123';
    vi.mocked(global.fetch, { partial: true }).mockImplementation(async (url: string, options?: RequestInit) => {
      if (url === '/api/v1/billing/my-plan') {
        return {
          ok: true,
          status: 200,
          json: async () => ({ current_plan: 'Starter' }),
        };
      }
      if (url.includes('/api/v1/billing/create-billing-portal-session') && options?.method === 'POST') {
        return {
          ok: true,
          status: 200,
          json: async () => ({ url: mockPortalUrl }),
        };
      }
      return { ok: true, json: async () => ({}) };
    });

    await act(async () => {
      render(<PricingPage />);
    });

    let manageButton;
    await waitFor(() => {
       manageButton = screen.getAllByText('Manage Plan')[0];
    });

    await act(async () => {
       fireEvent.click(manageButton!);
    });

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/v1/billing/create-billing-portal-session', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
      });
      expect(window.location.href).toBe(mockPortalUrl);
    });
  });

  it('renders loading state correctly', async () => {
    // Keep fetch promise pending to test loading state
    let resolveFetch;
    vi.mocked(global.fetch, { partial: true }).mockImplementation(() => new Promise((resolve) => {
        resolveFetch = resolve;
    }));

    render(<PricingPage />);

    // Test that the loading button appears
    expect(screen.getAllByText('Loading...').length).toBeGreaterThan(0);
    expect(screen.getAllByRole('button', { name: /loading/i }).length).toBe(4);

    resolveFetch({ ok: true, json: async () => ({ current_plan: 'Free' }) });
  });

  it('handles manage billing portal errors gracefully', async () => {
    vi.mocked(global.fetch, { partial: true }).mockImplementation(async (url: string, options?: RequestInit) => {
      if (url === '/api/v1/billing/my-plan') {
        return {
          ok: true,
          status: 200,
          json: async () => ({ current_plan: 'Starter' }),
        };
      }
      if (url.includes('/api/v1/billing/create-billing-portal-session') && options?.method === 'POST') {
        throw new Error('Network error');
      }
      return { ok: true, json: async () => ({}) };
    });

    const alertMock = vi.spyOn(window, 'alert').mockImplementation(() => {});
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    await act(async () => {
      render(<PricingPage />);
    });

    let manageButton;
    await waitFor(() => {
       manageButton = screen.getAllByText('Manage Plan')[0];
    });

    await act(async () => {
       fireEvent.click(manageButton!);
    });

    await waitFor(() => {
      expect(consoleSpy).toHaveBeenCalled();
      expect(alertMock).toHaveBeenCalledWith('Failed to initiate billing portal. Please try again.');
    });

    alertMock.mockRestore();
    consoleSpy.mockRestore();
  });

  it('renders business plan upgrade states', async () => {
    vi.mocked(global.fetch, { partial: true }).mockImplementation(async (url: string) => {
      if (url === '/api/v1/billing/my-plan') {
        return {
          ok: true,
          status: 200,
          json: async () => ({ current_plan: 'Business' }),
        };
      }
      return { ok: true, json: async () => ({}) };
    });

    await act(async () => {
      render(<PricingPage />);
    });

    await waitFor(() => {
       expect(screen.getAllByText('Manage Plan')[0]).toBeVisible();
    });

    // Check we get current plan status for Business
    expect(screen.getByText('My Plan: Business')).toBeDefined();
  });

  it('renders business plan handleUpgrade state', async () => {
    const mockCheckoutUrl = 'https://checkout.stripe.com/pay/test_session_123';
    vi.mocked(global.fetch, { partial: true }).mockImplementation(async (url: string, options?: RequestInit) => {
      if (url === '/api/v1/billing/my-plan') {
        return {
          ok: true,
          status: 200,
          json: async () => ({ current_plan: 'Free' }),
        };
      }
      if (url.includes('/api/v1/billing/create-checkout-session') && options?.method === 'POST') {
        return {
          ok: true,
          status: 200,
          json: async () => ({ checkout_url: mockCheckoutUrl }),
        };
      }
      return { ok: true, json: async () => ({}) };
    });


    await act(async () => {
      render(<PricingPage />);
    });

    let upgradeButton;
    await waitFor(() => {
       upgradeButton = screen.getByText('Upgrade to Business via Stripe');
    });

    await act(async () => {
       fireEvent.click(upgradeButton!);
    });

    // Wait for the async logic to finish
    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/v1/billing/create-checkout-session', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ tier: 'Business', is_subscription: true, subscription_interval: 'month' }),
      });
      expect(window.location.href).toBe(mockCheckoutUrl);
    });
  });

  it('renders pro plan handleUpgrade state', async () => {
    const mockCheckoutUrl = 'https://checkout.stripe.com/pay/test_session_123';
    vi.mocked(global.fetch, { partial: true }).mockImplementation(async (url: string, options?: RequestInit) => {
      if (url === '/api/v1/billing/my-plan') {
        return {
          ok: true,
          status: 200,
          json: async () => ({ current_plan: 'Free' }),
        };
      }
      if (url.includes('/api/v1/billing/create-checkout-session') && options?.method === 'POST') {
        return {
          ok: true,
          status: 200,
          json: async () => ({ checkout_url: mockCheckoutUrl }),
        };
      }
      return { ok: true, json: async () => ({}) };
    });


    await act(async () => {
      render(<PricingPage />);
    });

    let upgradeButton;
    await waitFor(() => {
       upgradeButton = screen.getByText('Upgrade to Pro via Stripe');
    });

    await act(async () => {
       fireEvent.click(upgradeButton!);
    });

    // Wait for the async logic to finish
    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/v1/billing/create-checkout-session', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ tier: 'Pro', is_subscription: true, subscription_interval: 'month' }),
      });
      expect(window.location.href).toBe(mockCheckoutUrl);
    });
  });

  it('handles plan fetch errors gracefully', async () => {
    vi.mocked(global.fetch, { partial: true }).mockImplementation(async (url: string) => {
      if (url === '/api/v1/billing/my-plan') {
         throw new Error('Network plan error');
      }
      return { ok: true, json: async () => ({}) };
    });

    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    await act(async () => {
      render(<PricingPage />);
    });

    await waitFor(() => {
      expect(consoleSpy).toHaveBeenCalled();
    });

    consoleSpy.mockRestore();
  });

  it('handles manage billing portal not ok gracefully', async () => {
    vi.mocked(global.fetch, { partial: true }).mockImplementation(async (url: string, options?: RequestInit) => {
      if (url === '/api/v1/billing/my-plan') {
        return {
          ok: true,
          status: 200,
          json: async () => ({ current_plan: 'Starter' }),
        };
      }
      if (url.includes('/api/v1/billing/create-billing-portal-session') && options?.method === 'POST') {
        return {
          ok: false,
          json: async () => ({})
        };
      }
      return { ok: true, json: async () => ({}) };
    });

    const alertMock = vi.spyOn(window, 'alert').mockImplementation(() => {});
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    await act(async () => {
      render(<PricingPage />);
    });

    let manageButton;
    await waitFor(() => {
       manageButton = screen.getAllByText('Manage Plan')[0];
    });

    await act(async () => {
       fireEvent.click(manageButton!);
    });

    await waitFor(() => {
      expect(consoleSpy).toHaveBeenCalled();
      expect(alertMock).toHaveBeenCalledWith('Failed to initiate billing portal. Please try again.');
    });

    alertMock.mockRestore();
    consoleSpy.mockRestore();
  });

  it('handles upgrade not ok gracefully', async () => {
    vi.mocked(global.fetch, { partial: true }).mockImplementation(async (url: string, options?: RequestInit) => {
      if (url === '/api/v1/billing/my-plan') {
        return {
          ok: true,
          status: 200,
          json: async () => ({ current_plan: 'Free' }),
        };
      }
      if (url.includes('/api/v1/billing/create-checkout-session') && options?.method === 'POST') {
        return {
          ok: false,
          json: async () => ({})
        };
      }
      return { ok: true, json: async () => ({}) };
    });

    const alertMock = vi.spyOn(window, 'alert').mockImplementation(() => {});
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    await act(async () => {
      render(<PricingPage />);
    });

    let upgradeButton;
    await waitFor(() => {
       upgradeButton = screen.getByText('Upgrade to Pro via Stripe');
    });

    await act(async () => {
       fireEvent.click(upgradeButton!);
    });

    await waitFor(() => {
      expect(screen.getByRole('alert')).toHaveTextContent('Checkout is unavailable. Your plan has not changed. Please try again.');
      expect(window.location.href).toBe('');
      expect(alertMock).not.toHaveBeenCalled();
    });

    alertMock.mockRestore();
    consoleSpy.mockRestore();
  });

  it('updates the price when annual billing is toggled', async () => {
    vi.mocked(global.fetch, { partial: true }).mockImplementation(async (url: string) => {
      if (url === '/api/v1/billing/my-plan') {
        return {
          ok: true,
          status: 200,
          json: async () => ({ current_plan: 'Free' }),
        };
      }
      return { ok: true, json: async () => ({}) };
    });

    await act(async () => {
      render(<PricingPage />);
    });

    // Verify initial monthly pricing
    expect(screen.getByText('$29')).toBeDefined();

    let toggle: HTMLElement;
    await waitFor(() => {
      toggle = screen.getByRole('checkbox');
    });

    // Toggle annual billing
    await act(async () => {
      fireEvent.click(toggle!);
    });

    // Verify annual pricing with dollar symbol
    await waitFor(() => {
      expect(screen.getByText('$23')).toBeDefined();
    });
  });
});
