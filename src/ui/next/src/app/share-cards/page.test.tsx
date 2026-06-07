import '@testing-library/jest-dom';
import { render, screen } from '@testing-library/react';
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
    const branding = screen.getByText(/Powered by OHC/i);
    expect(branding).toBeInTheDocument();
  });
});
