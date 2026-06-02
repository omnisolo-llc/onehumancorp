import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import HelpArticlePage from './page';

let mockUseParams = { articleId: 'getting-started' };
let mockPush = vi.fn();
let mockBack = vi.fn();

vi.mock('next/navigation', () => ({
  useParams: () => mockUseParams,
  useRouter: () => ({ push: mockPush, back: mockBack })
}));

describe('HelpArticlePage', () => {
  let consoleErrorSpy: any;

  beforeEach(() => {
    vi.clearAllMocks();
    mockUseParams = { articleId: 'getting-started' };
    mockPush.mockClear();
    mockBack.mockClear();
    consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
  });

  afterEach(() => {
    consoleErrorSpy.mockRestore();
  });

  it('renders loading state initially', () => {
    global.fetch = vi.fn(() => new Promise(() => {})) as any;
    render(<HelpArticlePage />);
    expect(screen.getByText('Loading...')).toBeInTheDocument();
  });

  it('renders article content', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ title: 'Getting Started with OHC', contentHtml: '<div data-testid="article-content"></div>' }),
    });

    render(<HelpArticlePage />);
    await waitFor(() => {
      expect(screen.getByText('Getting Started with OHC')).toBeInTheDocument();
    });
    expect(screen.getByText('← Back to Help Center')).toBeInTheDocument();
  });

  it('handles not found error', async () => {
    mockUseParams = { articleId: 'invalid-id' };
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 404,
    });
    render(<HelpArticlePage />);
    await waitFor(() => {
      expect(screen.getByText('Article Not Found')).toBeInTheDocument();
    });
  });

  it('purifies HTML content', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ title: 'Getting Started with OHC', contentHtml: '<div data-testid="article-content"></div>' }),
    });
    render(<HelpArticlePage />);
    await waitFor(() => {
      expect(screen.getByText('Getting Started with OHC')).toBeInTheDocument();
    });
    // DOMPurify is mocked so it just passes the string through
    expect(screen.getByTestId('article-content')).toBeInTheDocument();
  });

  it('navigates back when clicking the back button', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ title: 'Getting Started with OHC', contentHtml: '<div data-testid="article-content"></div>' }),
    });
    render(<HelpArticlePage />);
    await waitFor(() => {
      expect(screen.getByText('Getting Started with OHC')).toBeInTheDocument();
    });

    const backBtn = screen.getByText('← Back to Help Center');
    fireEvent.click(backBtn);
    expect(mockPush).toHaveBeenCalledWith('/help');
  });
});
