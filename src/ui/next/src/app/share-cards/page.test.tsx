import { render, screen, act } from '@testing-library/react';
import ShareCardsPage from './page';
import { describe, it, expect, vi } from 'vitest';

vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: vi.fn() })
}));

describe('ShareCardsPage', () => {
  it('renders direct social share links', () => {
    render(<ShareCardsPage />);

    const twitterLink = screen.getByText(/Share on X/i);
    expect(twitterLink).toBeInTheDocument();
    expect(twitterLink.closest('a')).toHaveAttribute('href', expect.stringContaining('twitter.com/intent/tweet'));

    const facebookLink = screen.getByText(/Share on Facebook/i);
    expect(facebookLink).toBeInTheDocument();
    expect(facebookLink.closest('a')).toHaveAttribute('href', expect.stringContaining('facebook.com/sharer/sharer.php'));
  });

  it('renders Powered by OHC branding in preview', () => {
    render(<ShareCardsPage />);
    const brandingElements = screen.getAllByText(/Powered by OHC/i);
    expect(brandingElements.length).toBeGreaterThan(0);
  });

  it('shows soft paywall when trying to remove branding without Pro', () => {
    window.localStorage.setItem('has_pro', 'false');
    render(<ShareCardsPage />);
    const toggle = screen.getByRole('checkbox');
    act(() => {
        toggle.click();
    });
    expect(screen.getAllByText('Upgrade to Pro').length).toBeGreaterThan(0);
  });

  it('allows removing branding with Pro', () => {
    window.localStorage.setItem('has_pro', 'true');
    render(<ShareCardsPage />);
    const toggle = screen.getByRole('checkbox');
    act(() => {
        toggle.click();
    });
    expect(screen.queryByText('Powered by OHC')).toBeNull();
  });
});
