import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import ReferralFabBuilder from './page';

describe('ReferralFabBuilder', () => {
  beforeEach(() => {
    localStorage.clear();
    global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => ({ current_plan: 'free' }) });
  });

  it('renders the builder with default values', () => {
    render(<ReferralFabBuilder />);
    expect(screen.getByText('Referral FAB Builder')).toBeDefined();

    const input = screen.getByDisplayValue('$10');
    expect(input).toBeDefined();
  });

  it('updates the reward value when input changes', () => {
    render(<ReferralFabBuilder />);
    const input = screen.getByDisplayValue('$10');

    fireEvent.change(input, { target: { value: '20% Off' } });

    expect(screen.getByDisplayValue('20% Off')).toBeDefined();
    expect(screen.getByText('Get 20% Off')).toBeDefined(); // Preview title
  });

  it('shows paywall when trying to remove branding on free tier', () => {
    render(<ReferralFabBuilder />);

    const toggle = screen.getByRole('switch');
    fireEvent.click(toggle);

    expect(screen.getByText('Upgrade to Pro')).toBeDefined();
    expect(screen.getByText('Remove the "Powered by OHC" branding and unlock premium widgets by upgrading to our Pro plan.')).toBeDefined();
  });

  it('does not grant branding removal from an upgrade click alone', () => {
    render(<ReferralFabBuilder />);

    // Attempt to toggle
    const toggle = screen.getByRole('switch');
    fireEvent.click(toggle);

    // Upgrade
    const upgradeButton = screen.getByText('Upgrade Now');
    fireEvent.click(upgradeButton);

    expect(localStorage.getItem('has_pro')).toBeNull();
    expect(toggle.getAttribute('aria-checked')).toBe('false');
    expect(screen.getByText('⚡ Powered by OHC')).toBeDefined();
  });

  it('changes theme color', () => {
    render(<ReferralFabBuilder />);

    const redButton = screen.getByLabelText('Select color #dc2626');
    fireEvent.click(redButton);

    // Check if the preview button has the new color
    const getShareLinkBtn = screen.getByText('Get Share Link');
    expect(getShareLinkBtn.style.backgroundColor).toBe('rgb(220, 38, 38)'); // #dc2626
  });
});
