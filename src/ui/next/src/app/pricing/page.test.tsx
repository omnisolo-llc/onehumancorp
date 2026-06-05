import { render, screen, fireEvent } from '@testing-library/react';
import React from 'react';
import PricingPage from './page';
import { useRouter } from 'next/navigation';
import { expect, test, vi, describe, beforeEach, afterEach } from 'vitest';

// Mock next/navigation
vi.mock('next/navigation', () => ({
  useRouter: vi.fn()
}));

// Mock TooltipRegistry to avoid context errors
vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</>
}));

const mockPush = vi.fn();

describe('PricingPage', () => {
  beforeEach(() => {
    (useRouter as any).mockReturnValue({
      push: mockPush,
    });

    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  test('renders pricing tiers correctly', () => {
    render(<PricingPage />);

    expect(screen.getByText('Pricing Plans')).toBeDefined();
    expect(screen.getByText('Free')).toBeDefined();
    expect(screen.getByText('Starter')).toBeDefined();
    expect(screen.getByText('Pro')).toBeDefined();
    expect(screen.getByText('Business')).toBeDefined();
  });

  test('Back to Dashboard button works', () => {
    render(<PricingPage />);

    const backBtn = screen.getByText('Back to Dashboard');
    fireEvent.click(backBtn);

    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });

  test('Upgrade to Starter via Stripe button works', () => {
    render(<PricingPage />);

    const upgradeBtn = screen.getByText('Upgrade to Starter via Stripe');
    fireEvent.click(upgradeBtn);

    expect(mockPush).toHaveBeenCalledWith('/checkout?tier=Starter');
  });
});
