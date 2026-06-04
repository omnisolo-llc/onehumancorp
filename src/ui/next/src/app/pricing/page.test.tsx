import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import PricingPage from './page';

const mockPush = vi.fn();

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
}));

vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</>,
}));

describe('PricingPage', () => {
  it('renders the pricing headers', () => {
    render(<PricingPage />);
    expect(screen.getByText('Pricing Plans')).toBeDefined();
    expect(screen.getByText(/Plain-language pricing/i)).toBeDefined();
  });

  it('renders all pricing tiers', () => {
    render(<PricingPage />);
    expect(screen.getByText('Free')).toBeDefined();
    expect(screen.getByText('Starter')).toBeDefined();
    expect(screen.getByText('Pro')).toBeDefined();
    expect(screen.getByText('Business')).toBeDefined();
  });

  it('handles upgrade to Starter', () => {
    mockPush.mockClear();

    render(<PricingPage />);

    const upgradeButton = screen.getByText('Upgrade to Starter via Stripe');
    fireEvent.click(upgradeButton);

    expect(mockPush).toHaveBeenCalledWith('/checkout?tier=Starter');
  });
});
