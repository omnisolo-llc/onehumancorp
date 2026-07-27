import { render, screen, act, waitFor } from '@testing-library/react';
import ShareCardsPage from './page';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import '@testing-library/jest-dom/vitest';

vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: vi.fn() })
}));

describe('ShareCardsPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => ({ current_plan: 'free' }) });
  });
  it('renders direct social share links', () => {
    act(() => { render(<ShareCardsPage />); });

    const twitterLink = screen.getByText(/Share on X/i);
    expect(twitterLink).toBeInTheDocument();
    expect(twitterLink.closest('a')).toHaveAttribute('href', expect.stringContaining('twitter.com/intent/tweet'));

    const facebookLink = screen.getByText(/Share on Facebook/i);
    expect(facebookLink).toBeInTheDocument();
    expect(facebookLink.closest('a')).toHaveAttribute('href', expect.stringContaining('facebook.com/sharer/sharer.php'));
  });

  it('renders Powered by OHC branding in preview', () => {
    act(() => { render(<ShareCardsPage />); });
    const brandingElements = screen.getAllByText(/Powered by OHC/i);
    expect(brandingElements.length).toBeGreaterThan(0);
  });

  it('shows soft paywall when trying to remove branding without Pro', () => {
    act(() => { render(<ShareCardsPage />); });
    const toggle = screen.getByRole('checkbox');
    act(() => {
        toggle.click();
    });
    expect(screen.getAllByText('Upgrade to Pro').length).toBeGreaterThan(0);
  });

  it('allows removing branding when the plan API reports Pro', async () => {
    global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => ({ current_plan: 'pro' }) });
    act(() => { render(<ShareCardsPage />); });
    await waitFor(() => expect(global.fetch).toHaveBeenCalledWith('/api/v1/billing/my-plan'));
    const toggle = screen.getByRole('checkbox');
    act(() => {
        toggle.click();
    });
    expect(screen.queryByText('Powered by OHC')).toBeNull();
  });
});
