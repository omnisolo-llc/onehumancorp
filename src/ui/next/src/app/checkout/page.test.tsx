import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import CheckoutPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</>,
}));

describe('CheckoutPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders the checkout page', () => {
    render(<CheckoutPage />);
    expect(screen.getByText('Checkout')).toBeDefined();
    expect(screen.getByText('Pay Now')).toBeDefined();
  });

  it('handles payment click', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve({ referral_link: 'http://test.link' })
    } as any);

    render(<CheckoutPage />);

    const payButton = screen.getByText('Pay Now');
    fireEvent.click(payButton);

    expect(payButton.textContent).toBe('Processing...');

    await waitFor(() => {
      expect(screen.getByText('Payment Successful!')).toBeDefined();
    });
  });

  describe('Powered by OHC Viral Badge', () => {
    it('renders the viral badge', () => {
      render(<CheckoutPage />);
      expect(screen.getByText('⚡ Powered by')).toBeDefined();
      expect(screen.getByText('OHC')).toBeDefined();
    });

    it('has the correct referral link', () => {
      render(<CheckoutPage />);
      const badgeLink = screen.getByText('⚡ Powered by').closest('a');
      expect(badgeLink?.getAttribute('href')).toBe('ohc://join?ref=checkout-viral');
    });

    it('has the correct styling classes for a fixed footer', () => {
      render(<CheckoutPage />);
      const badgeLink = screen.getByText('⚡ Powered by').closest('a');
      const badgeContainer = badgeLink?.parentElement;
      expect(badgeContainer?.className).toContain('fixed bottom-4 left-1/2');
    });

    it('has glassmorphism and transition classes', () => {
      render(<CheckoutPage />);
      const badgeLink = screen.getByText('⚡ Powered by').closest('a');
      expect(badgeLink?.className).toContain('backdrop-blur-xl');
      expect(badgeLink?.className).toContain('transition-all');
      expect(badgeLink?.className).toContain('hover:scale-105');
    });

    it('does not disappear when payment is successful', async () => {
      global.fetch = vi.fn().mockResolvedValue({
        json: () => Promise.resolve({ referral_link: 'http://test.link' })
      } as any);

      render(<CheckoutPage />);

      const payButton = screen.getByText('Pay Now');
      fireEvent.click(payButton);

      await waitFor(() => {
        expect(screen.getByText('Payment Successful!')).toBeDefined();
      });

      // The badge should still be there after the modal shows
      expect(screen.getByText('⚡ Powered by')).toBeDefined();
      expect(screen.getByText('OHC')).toBeDefined();
    });
  });
});
