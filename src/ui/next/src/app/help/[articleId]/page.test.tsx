import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import HelpArticlePage from './page';
import * as navigation from 'next/navigation';

vi.mock('next/navigation', () => ({
  useParams: vi.fn(),
}));

describe('HelpArticlePage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders loading state initially', () => {
    vi.mocked(navigation.useParams).mockReturnValue({ articleId: 'getting-started-1' });
    global.fetch = vi.fn(() => new Promise<Response>(() => {})); // Never resolves

    render(<HelpArticlePage />);
    expect(screen.getByText('Loading article...')).toBeInTheDocument();
  });

  it('renders article content on successful fetch', async () => {
    vi.mocked(navigation.useParams).mockReturnValue({ articleId: 'getting-started-1' });
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        title: 'Getting Started',
        contentHtml: '<p>Welcome to OneHumanCorp!</p>'
      })
    });

    render(<HelpArticlePage />);

    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeInTheDocument();
    });

    expect(screen.getByText('Welcome to OneHumanCorp!')).toBeInTheDocument();
  });

  it('renders error state on failed fetch', async () => {
    vi.mocked(navigation.useParams).mockReturnValue({ articleId: 'invalid-id' });
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
    });

    render(<HelpArticlePage />);

    await waitFor(() => {
      expect(screen.getByText('Oops!')).toBeInTheDocument();
    });

    expect(screen.getByText('Article not found')).toBeInTheDocument();
  });
});
