import React from 'react';
import { render, screen } from '@testing-library/react';
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

  it('renders "Article Not Found" for an unknown article', () => {
    (useParams as any).mockReturnValue({ article: 'unknown-article' });
    (useRouter as any).mockReturnValue({ push: vi.fn() });

    render(<HelpArticlePage />);

    expect(screen.getByText('Article Not Found')).toBeInTheDocument();
    expect(screen.getByText("We couldn't find the article you're looking for.")).toBeInTheDocument();
  });
});
