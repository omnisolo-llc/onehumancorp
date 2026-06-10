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

  beforeEach(() => {
    vi.clearAllMocks();
    (useRouter as any).mockReturnValue({ push: mockPush });
  });

  it('renders the pricing page', () => {
    render(<PricingPage />);
    expect(screen.getByText('Pricing Plans')).toBeDefined();
    expect(screen.getByText('Free')).toBeDefined();
    expect(screen.getByText('Starter')).toBeDefined();
    expect(screen.getByText('Pro')).toBeDefined();
    expect(screen.getByText('Business')).toBeDefined();
  });

  it('navigates to checkout when upgrading to Starter', () => {
    render(<PricingPage />);
    const upgradeButton = screen.getByText('Upgrade to Starter via Stripe');
    fireEvent.click(upgradeButton);
    expect(mockPush).toHaveBeenCalledWith('/checkout?tier=Starter');
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
