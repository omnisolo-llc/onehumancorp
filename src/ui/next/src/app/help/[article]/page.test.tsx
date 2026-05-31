import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import HelpArticlePage from './page';
import * as navigation from 'next/navigation';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(),
  useParams: vi.fn()
}));

describe('HelpArticlePage', () => {
  it('renders getting-started article correctly', () => {
    vi.spyOn(navigation, 'useParams').mockReturnValue({ article: 'getting-started' });
    vi.spyOn(navigation, 'useRouter').mockReturnValue({ push: vi.fn() } as any);

    render(<HelpArticlePage />);

    expect(screen.getByText('Getting Started with Your Store')).toBeInTheDocument();
    expect(screen.getByText('Step 1: Tell us about your business')).toBeInTheDocument();
  });

  it('renders "Article Not Found" for an unknown article', () => {
    vi.spyOn(navigation, 'useParams').mockReturnValue({ article: 'unknown-article' });
    vi.spyOn(navigation, 'useRouter').mockReturnValue({ push: vi.fn() } as any);

    render(<HelpArticlePage />);

    expect(screen.getByText('Article Not Found')).toBeInTheDocument();
    expect(screen.getByText("We couldn't find the article you're looking for.")).toBeInTheDocument();
  });
});
