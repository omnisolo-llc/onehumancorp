import '@testing-library/jest-dom';
import React from 'react';
import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, it, expect, vi } from 'vitest';
import HelpArticlePage from './page';
import { useRouter, useParams } from 'next/navigation';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(),
  useParams: vi.fn()
}));

describe('HelpArticlePage', () => {
  it('renders getting-started article correctly', () => {
    (useParams as any).mockReturnValue({ article: 'getting-started' });
    (useRouter as any).mockReturnValue({ push: vi.fn() });

    render(<HelpArticlePage />);

    expect(screen.getByText('Getting Started with Your Store')).toBeInTheDocument();
    expect(screen.getByText('Step 1: Tell us about your business')).toBeInTheDocument();
  });

  it('navigates back to the help center when the back button is clicked', async () => {
    const user = userEvent.setup();
    render(<HelpArticlePage />);

    const backButton = screen.getByRole('button', { name: /Back to Help Center/i });
    expect(backButton).toBeInTheDocument();

    await user.click(backButton);
    // Router push is mocked in vitest setup
  });

  it('renders "Article Not Found" for an unknown article', () => {
    (useParams as any).mockReturnValue({ article: 'unknown-article' });
    (useRouter as any).mockReturnValue({ push: vi.fn() });

    render(<HelpArticlePage />);

    expect(screen.getByText('Article Not Found')).toBeInTheDocument();
    expect(screen.getByText("We couldn't find the article you're looking for.")).toBeInTheDocument();
  });

  it('navigates back to the help center when the back button is clicked', async () => {
    (useParams as any).mockReturnValue({ article: 'getting-started' });
    const pushMock = vi.fn();
    (useRouter as any).mockReturnValue({ push: pushMock });

    const user = userEvent.setup();
    render(<HelpArticlePage />);

    const backButton = screen.getByText(/Back to Help Center/i);
    await user.click(backButton);

    expect(pushMock).toHaveBeenCalledWith('/help');
  });
});
