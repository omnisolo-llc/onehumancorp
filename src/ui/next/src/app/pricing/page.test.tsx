import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
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

describe('PricingPage', () => {
  const mockPush = vi.fn();
  const mockFetch = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    (useRouter as any).mockReturnValue({ push: mockPush });
    global.fetch = mockFetch;
    // Mock default free tier for most tests to avoid pending fetch
    mockFetch.mockResolvedValue({
      ok: true,
      json: async () => ({ current_plan: 'Free' })
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders the pricing page', async () => {
    render(<PricingPage />);
    expect(screen.getByText('Pricing Plans')).toBeDefined();
    expect(screen.getByText('Free')).toBeDefined();
    expect(screen.getByText('Starter')).toBeDefined();
    expect(screen.getByText('Pro')).toBeDefined();
    expect(screen.getByText('Business')).toBeDefined();
    await waitFor(() => {
        expect(screen.getAllByText('Current Plan')).toBeDefined();
    });
  });

  it('navigates to checkout when upgrading to Starter', async () => {
    render(<PricingPage />);
    await waitFor(() => {
      const upgradeButton = screen.getByText('Upgrade to Starter via Stripe');
      fireEvent.click(upgradeButton);
      expect(mockPush).toHaveBeenCalledWith('/checkout?tier=Starter');
    });
  });

  it('renders the PoweredByOHC component', async () => {
    render(<PricingPage />);
    await waitFor(() => {
        expect(screen.getByTestId('powered-by-ohc')).toBeDefined();
    });
  });

  it('renders the FAQ section with Stripe Billing integration info', async () => {
    render(<PricingPage />);
    await waitFor(() => {
        expect(screen.getByText(/Stripe Billing for self-serve plan upgrades, downgrades, and cancellation/)).toBeDefined();
    });
  });

  it('dynamically disables the Current Plan button for Starter', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ current_plan: 'Starter' })
    });

    render(<PricingPage />);

    await waitFor(() => {
        const currentButtons = screen.getAllByText('Current Plan');
        expect(currentButtons.length).toBe(1);
        const currentButton = currentButtons[0] as HTMLButtonElement;
        expect(currentButton.disabled).toBe(true);

        const freeButton = screen.getByText('Downgrade to Free via Stripe');
        expect(freeButton).toBeDefined();
    });
  });
});
