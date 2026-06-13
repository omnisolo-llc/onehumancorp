import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
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

describe('PricingPage', () => {
  const mockPush = vi.fn();

  let originalWindowLocation: any;

  beforeEach(() => {
    vi.clearAllMocks();
    (useRouter as any).mockReturnValue({ push: mockPush });
    global.fetch = vi.fn();

    // Mock window.location.href
    originalWindowLocation = window.location;
    delete (window as any).location;
    window.location = { ...originalWindowLocation, href: '' } as any;
  });

  afterEach(() => {
    window.location = originalWindowLocation;
  });

  it('renders the pricing page', () => {
    render(<PricingPage />);
    expect(screen.getByText('Pricing Plans')).toBeDefined();
    expect(screen.getByText('Free')).toBeDefined();
    expect(screen.getByText('Starter')).toBeDefined();
    expect(screen.getByText('Pro')).toBeDefined();
    expect(screen.getByText('Business')).toBeDefined();
  });

  it('initiates checkout session when upgrading to Starter', async () => {
    const mockCheckoutUrl = 'https://checkout.stripe.com/pay/test_session_123';
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ checkout_url: mockCheckoutUrl }),
    });

    render(<PricingPage />);
    const upgradeButton = screen.getByText('Upgrade to Starter via Stripe');
    fireEvent.click(upgradeButton);

    // Wait for the async logic to finish
    await vi.waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/billing/create-checkout-session', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ tier: 'Starter' }),
      });
      expect(window.location.href).toBe(mockCheckoutUrl);
    });
  });

  it('handles upgrade errors gracefully', async () => {
    (global.fetch as any).mockRejectedValueOnce(new Error('Network error'));
    const alertMock = vi.spyOn(window, 'alert').mockImplementation(() => {});
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    render(<PricingPage />);
    const upgradeButton = screen.getByText('Upgrade to Starter via Stripe');
    fireEvent.click(upgradeButton);

    await vi.waitFor(() => {
      expect(consoleSpy).toHaveBeenCalled();
      expect(alertMock).toHaveBeenCalledWith('Failed to initiate upgrade. Please try again.');
    });

    alertMock.mockRestore();
    consoleSpy.mockRestore();
  });

  it('renders the PoweredByOHC component', () => {
    render(<PricingPage />);
    expect(screen.getByTestId('powered-by-ohc')).toBeDefined();
  });

  it('renders the FAQ section with Stripe Billing integration info', () => {
    render(<PricingPage />);
    expect(screen.getByText(/Stripe Billing for self-serve plan upgrades, downgrades, and cancellation/)).toBeDefined();
  });
});
