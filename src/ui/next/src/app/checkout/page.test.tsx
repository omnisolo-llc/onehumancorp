import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import CheckoutPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: vi.fn() })
}));

describe('CheckoutPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn();

    const localStorageMock = {
      getItem: vi.fn(),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock
    });
  });

  it('generates dynamic referral link on payment', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({ referral_link: 'https://ohc.app/ref/test-checkout' }),
    });

    render(<CheckoutPage />);

    const payNowBtn = screen.getByText('Pay Now');
    fireEvent.click(payNowBtn);

    await waitFor(() => {
      expect(screen.getByText('Payment Successful!')).toBeInTheDocument();
    });

    const referralInput = screen.getByDisplayValue('https://ohc.app/ref/test-checkout');
    expect(referralInput).toBeInTheDocument();
  });
});
