import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralCouponUnlockPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(() => ({
    push: vi.fn(),
  })),
}));

describe('ViralCouponUnlockPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn(() => Promise.resolve()),
      },
    });

    const localStorageMock = {
      getItem: vi.fn((key) => {
        if (key === 'tenant') return 'test-tenant';
        return null;
      }),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock,
      writable: true
    });
  });

  it('renders correctly', () => {
    render(<ViralCouponUnlockPage />);
    expect(screen.getByText('Share-to-Unlock Coupon 🎁')).toBeDefined();
    expect(screen.getByText('Coupon Settings')).toBeDefined();
  });

  it('updates offer name when typed', () => {
    render(<ViralCouponUnlockPage />);
    const inputs = screen.getAllByRole('textbox');
    const offerInput = inputs[0]; // Assuming first textbox is Offer Headline
    fireEvent.change(offerInput, { target: { value: 'New Test Offer' } });
    expect(screen.getByText('New Test Offer')).toBeDefined();
  });

  it('copies link to clipboard', async () => {
    render(<ViralCouponUnlockPage />);
    const copyButton = screen.getByRole('button', { name: /Copy Link/i });
    fireEvent.click(copyButton);
    await waitFor(() => {
      expect(navigator.clipboard.writeText).toHaveBeenCalled();
      expect(screen.getByText('Copied!')).toBeDefined();
    });
  });
});
