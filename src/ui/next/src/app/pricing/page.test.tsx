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

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

vi.mock('../components/ViralTrialExtensionWidget', () => ({
  ViralTrialExtensionWidget: () => <div data-testid="viral-trial-extension-widget" />,
}));

describe('PricingPage', () => {
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
          json: async () => ({ current_plan: 'Free' }),
        };
      }
      return { ok: true, json: async () => ({}) };
    });

    // Mock window.location.href
    originalWindowLocation = window.location;
    delete (window as any).location;
    window.location = { ...originalWindowLocation, href: '' } as any;
  });

  afterEach(() => {
    window.location = originalWindowLocation;
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

  it('initiates checkout session when upgrading to Starter', async () => {
    const mockCheckoutUrl = 'https://checkout.stripe.com/pay/test_session_123';
    (global.fetch as any).mockImplementation(async (url, options) => {
      if (url === '/api/billing/my-plan') {
        return {
          ok: true,
          json: async () => ({ current_plan: 'Free' }),
        };
      }
      if (url === '/api/billing/create-checkout-session' && options?.method === 'POST') {
        return {
          ok: true,
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
      expect(global.fetch).toHaveBeenCalledWith('/api/billing/create-checkout-session', {
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
    (global.fetch as any).mockImplementation(async (url, options) => {
      if (url === '/api/billing/my-plan') {
        return {
          ok: true,
          json: async () => ({ current_plan: 'Free' }),
        };
      }
      if (url === '/api/billing/create-checkout-session' && options?.method === 'POST') {
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
      expect(consoleSpy).toHaveBeenCalled();
      expect(alertMock).toHaveBeenCalledWith('Failed to initiate upgrade. Please try again.');
    });

    alertMock.mockRestore();
    consoleSpy.mockRestore();
  });

  it('renders the PoweredByOHC component', async () => {
    await act(async () => {
      render(<PricingPage />);
    });
    expect(screen.getByTestId('powered-by-ohc')).toBeDefined();
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
      expect(global.fetch).toHaveBeenCalledWith('/api/billing/create-billing-portal-session', {
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
    (global.fetch as any).mockImplementation(() => new Promise((resolve) => {
        resolveFetch = resolve;
    }));

    render(<PricingPage />);

    // Test that the loading button appears
    expect(screen.getAllByText('Loading...').length).toBeGreaterThan(0);
    expect(screen.getAllByRole('button', { name: /loading/i }).length).toBe(4);

    resolveFetch({ ok: true, json: async () => ({ current_plan: 'Free' }) });
  });

  it('handles manage billing portal errors gracefully', async () => {
    (global.fetch as any).mockImplementation(async (url, options) => {
      if (url === '/api/billing/my-plan') {
        return {
          ok: true,
          json: async () => ({ current_plan: 'Starter' }),
        };
      }
      if (url === '/api/billing/create-billing-portal-session' && options?.method === 'POST') {
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
    (global.fetch as any).mockImplementation(async (url, options) => {
      if (url === '/api/billing/my-plan') {
        return {
          ok: true,
          json: async () => ({ current_plan: 'Business' }),
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

    // Check we get current plan status for Business
    expect(screen.getByText('My Plan: Business')).toBeDefined();
  });

  it('renders business plan handleUpgrade state', async () => {
    const mockCheckoutUrl = 'https://checkout.stripe.com/pay/test_session_123';
    (global.fetch as any).mockImplementation(async (url, options) => {
      if (url === '/api/billing/my-plan') {
        return {
          ok: true,
          json: async () => ({ current_plan: 'Free' }),
        };
      }
      if (url === '/api/billing/create-checkout-session' && options?.method === 'POST') {
        return {
          ok: true,
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
      expect(global.fetch).toHaveBeenCalledWith('/api/billing/create-checkout-session', {
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
    (global.fetch as any).mockImplementation(async (url, options) => {
      if (url === '/api/billing/my-plan') {
        return {
          ok: true,
          json: async () => ({ current_plan: 'Free' }),
        };
      }
      if (url === '/api/billing/create-checkout-session' && options?.method === 'POST') {
        return {
          ok: true,
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
      expect(global.fetch).toHaveBeenCalledWith('/api/billing/create-checkout-session', {
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
    (global.fetch as any).mockImplementation(async (url, options) => {
      if (url === '/api/billing/my-plan') {
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
    (global.fetch as any).mockImplementation(async (url, options) => {
      if (url === '/api/billing/my-plan') {
        return {
          ok: true,
          json: async () => ({ current_plan: 'Starter' }),
        };
      }
      if (url === '/api/billing/create-billing-portal-session' && options?.method === 'POST') {
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
    (global.fetch as any).mockImplementation(async (url, options) => {
      if (url === '/api/billing/my-plan') {
        return {
          ok: true,
          json: async () => ({ current_plan: 'Free' }),
        };
      }
      if (url === '/api/billing/create-checkout-session' && options?.method === 'POST') {
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
      expect(consoleSpy).toHaveBeenCalled();
      expect(alertMock).toHaveBeenCalledWith('Failed to initiate upgrade. Please try again.');
    });

    alertMock.mockRestore();
    consoleSpy.mockRestore();
  });

  it('updates the price when annual billing is toggled', async () => {
    (global.fetch as any).mockImplementation(async (url: string) => {
      if (url === '/api/billing/my-plan') {
        return {
          ok: true,
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
