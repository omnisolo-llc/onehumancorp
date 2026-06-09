import { render, screen, act } from '@testing-library/react';
import ReferralWidgetPage from './page';
import { describe, it, expect, vi } from 'vitest';

vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: vi.fn() })
}));

describe('ReferralWidgetPage', () => {
  it('renders referral widget configuration', () => {
    render(<ReferralWidgetPage />);
    expect(screen.getByText(/Referral Widget Builder/i)).toBeInTheDocument();

    // Using a more robust selector since the text could be split
    expect(screen.getByText(/Give 10%, Get 10%/i)).toBeInTheDocument();
  });

  it('renders Powered by OHC branding in preview by default', () => {
    render(<ReferralWidgetPage />);
    const brandingElements = screen.getAllByText(/Powered by OHC/i);
    expect(brandingElements.length).toBeGreaterThan(0);
  });

  it('shows soft paywall when trying to remove branding without Pro', () => {
    window.localStorage.setItem('has_pro', 'false');
    render(<ReferralWidgetPage />);
    const toggle = screen.getByRole('checkbox', { name: /Remove "Powered by OHC"/i });
    act(() => {
        toggle.click();
    });
    expect(screen.getAllByText('Upgrade to Pro').length).toBeGreaterThan(0);
  });

  it('allows removing branding with Pro', () => {
    window.localStorage.setItem('has_pro', 'true');
    render(<ReferralWidgetPage />);
    const toggle = screen.getByRole('checkbox', { name: /Remove "Powered by OHC"/i });
    act(() => {
        toggle.click();
    });
    expect(screen.queryByText('⚡ Powered by OHC')).toBeNull();
  });
});
