import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import PricingPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

vi.mock('../../components/TooltipRegistry', () => ({
  WithTooltip: ({ children }: { children: React.ReactNode }) => <>{children}</>,
}));

describe('PricingPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
    // Reset window.open mock
    window.open = vi.fn();
  });

  it('renders the pricing tiers and the growth banner', () => {
    render(<PricingPage />);
    expect(screen.getByText('Pricing Plans')).toBeDefined();
    expect(screen.getByText('Free')).toBeDefined();
    expect(screen.getByText('Starter')).toBeDefined();
    expect(screen.getByText('Pro')).toBeDefined();

    // Growth banner
    expect(screen.getByText('Not ready to upgrade?')).toBeDefined();
    expect(screen.getByText(/Share on X to Unlock/i)).toBeDefined();
  });

  it('unlocks trial when sharing on X from the banner', async () => {
    render(<PricingPage />);

    const shareBtn = screen.getByText(/Share on X to Unlock/i);
    fireEvent.click(shareBtn);

    // Assert localStorage was updated
    expect(localStorage.getItem('ohc_pro_trial_unlocked')).toBe('true');
    expect(window.open).toHaveBeenCalledWith(
      expect.stringContaining('twitter.com/intent/tweet'),
      '_blank'
    );

    // The UI should update to show Trial Unlocked
    await waitFor(() => {
      expect(screen.getByText('✅ Trial Unlocked!')).toBeDefined();
    });
  });
});
