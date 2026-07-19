import { describe, it, expect, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import ReferralFabBuilder from './page';

describe('ReferralFabBuilder', () => {
  beforeEach(() => {
    localStorage.clear();
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

  it('allows removing branding after upgrading', () => {
    render(<ReferralFabBuilder />);

    // Attempt to toggle
    const toggle = screen.getByRole('switch');
    fireEvent.click(toggle);

    // Upgrade
    const upgradeButton = screen.getByText('Upgrade Now');
    fireEvent.click(upgradeButton);

    expect(localStorage.getItem('has_pro')).toBe('true');
    expect(toggle.getAttribute('aria-checked')).toBe('true');
    expect(screen.queryByText('⚡ Powered by OHC')).toBeNull(); // Should be removed from preview
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