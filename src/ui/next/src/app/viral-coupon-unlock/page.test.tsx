import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralCouponUnlockPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

const mockUseProPlan = vi.fn();
vi.mock('../components/useProPlan', () => ({
  useProPlan: () => mockUseProPlan(),
}));

describe('ViralCouponUnlockPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockUseProPlan.mockReturnValue({ hasPro: false, claimTrial: vi.fn(), claimError: null });

    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockResolvedValue(undefined),
      },
    });

    const localStorageMock = {
      getItem: vi.fn((key) => {
        if (key === 'tenant_id' || key === 'tenant') return 'test-tenant';
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

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders correctly', () => {
    render(<ViralCouponUnlockPage />);
    expect(screen.getByText('Share-to-Unlock Coupon 🎁')).toBeDefined();
    expect(screen.getByText('Coupon Settings')).toBeDefined();
    expect(screen.getByText('Preview: Unlock Page')).toBeDefined();
  });

  it('updates form inputs and preview', () => {
    render(<ViralCouponUnlockPage />);

    const headlineInput = screen.getByPlaceholderText('e.g. 20% Off Your First Order');
    fireEvent.change(headlineInput, { target: { value: '50% Off Special' } });

    const couponInput = screen.getByPlaceholderText('e.g. WELCOME20');
    fireEvent.change(couponInput, { target: { value: 'SECRET50' } });

    const sharesInput = screen.getByDisplayValue('3');
    fireEvent.change(sharesInput, { target: { value: '5' } });

    // Check if preview updates
    expect(screen.getByText('50% Off Special')).toBeDefined();

    // The coupon code is displayed in the locked area (with blur class in real CSS)
    const codeDisplay = screen.getByText('SECRET50');
    expect(codeDisplay).toBeDefined();

    // Check if shares update
    expect(screen.getByText('5 Shares')).toBeDefined();
    expect(screen.getByText('1 / 5')).toBeDefined();
  });

  it('copies link to clipboard', async () => {
    render(<ViralCouponUnlockPage />);

    const copyBtn = screen.getByRole('button', { name: 'Copy Link' });
    fireEvent.click(copyBtn);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByRole('button', { name: 'Copied!' })).toBeDefined();
  });

  it('navigates back to dashboard', () => {
    render(<ViralCouponUnlockPage />);

    const backBtn = screen.getByRole('button', { name: /Back to Dashboard/i });
    fireEvent.click(backBtn);

    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });

  it('shows paywall when non-Pro user clicks to remove branding', () => {
    render(<ViralCouponUnlockPage />);

    const removeBrandingCheckbox = screen.getByRole('checkbox', { name: /Remove "Powered by OHC" Badge/i });
    fireEvent.click(removeBrandingCheckbox);

    expect(screen.getByText('Upgrade to Remove Branding')).toBeDefined();
  });

  it('hides Powered by OHC widget when user is Pro', () => {
    mockUseProPlan.mockReturnValue({ hasPro: true, claimTrial: vi.fn(), claimError: null });
    render(<ViralCouponUnlockPage />);

    const removeBrandingCheckbox = screen.getByRole('checkbox', { name: /Remove "Powered by OHC" Badge/i }) as HTMLInputElement;
    expect(removeBrandingCheckbox.checked).toBe(true);

    const poweredBy = screen.queryByTestId('powered-by-ohc');
    expect(poweredBy).toBeNull();
  });
});
