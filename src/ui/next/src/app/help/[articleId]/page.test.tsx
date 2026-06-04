import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import HelpArticlePage from './page';
import userEvent from '@testing-library/user-event';

// Mock next/navigation
const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
  useParams: () => ({
    articleId: 'getting-started'
  })
}));

describe('HelpArticlePage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        title: "Getting Started with Your Store",
        contentHtml: "<p>Welcome to OneHumanCorp!</p>"
      })
    });
  });

  it('renders loading state initially', async () => {
    let resolvePromise: any;
    const promise = new Promise(resolve => { resolvePromise = resolve; });
    global.fetch = vi.fn().mockImplementation(() => promise);

    render(<HelpArticlePage />);
    expect(screen.getByText('Loading...')).toBeInTheDocument();

    await act(async () => {
      resolvePromise({
        ok: true,
        json: () => Promise.resolve({
            title: "Getting Started with Your Store",
            contentHtml: "<p>Welcome to OneHumanCorp!</p>"
        })
      });
    });
  });

  it('renders article loaded from API', async () => {
    render(<HelpArticlePage />);

    await waitFor(() => {
      expect(screen.getByText('Getting Started with Your Store')).toBeInTheDocument();
      expect(screen.getByText('Welcome to OneHumanCorp!')).toBeInTheDocument();
    });
  });

  it('navigates back when clicking the back button', async () => {
    const user = userEvent.setup();
    render(<HelpArticlePage />);

    await waitFor(() => {
      expect(screen.getByText('Getting Started with Your Store')).toBeInTheDocument();
    });

    const backButton = screen.getByRole('button', { name: /Back to Help Center/i });
    await user.click(backButton);

    expect(mockPush).toHaveBeenCalledWith('/help');
  });

  it('handles not found error', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
    });

    render(<HelpArticlePage />);

    await waitFor(() => {
      expect(screen.getByText('Article Not Found')).toBeInTheDocument();
      expect(screen.getByText("We couldn't find the article you're looking for.")).toBeInTheDocument();
    });
  });
});
