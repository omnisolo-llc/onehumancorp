import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import HelpArticlePage from './page';
import userEvent from '@testing-library/user-event';

// Mock next/navigation


describe('HelpArticlePage', () => {
  let consoleErrorMock: any;
  beforeAll(() => {
    consoleErrorMock = vi.spyOn(console, 'error').mockImplementation(() => {});
  });
  afterAll(() => {
    consoleErrorMock.mockRestore();
  });
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

    const { unmount } = render(<HelpArticlePage />);
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
    await waitFor(() => {
        expect(screen.getByText('Getting Started with Your Store')).toBeInTheDocument();
    });
    unmount();
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


describe('HelpArticlePage Error Cases', () => {
  let consoleErrorMock: any;
  beforeAll(() => {
    consoleErrorMock = vi.spyOn(console, 'error').mockImplementation(() => {});
  });
  afterAll(() => {
    consoleErrorMock.mockRestore();
  });
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('handles json parse error', async () => {
    let resolvePromise: any;
    const promise = new Promise(resolve => { resolvePromise = resolve; });
    global.fetch = vi.fn().mockImplementation(() => promise);

    const { unmount } = render(<HelpArticlePage />);
    expect(screen.getByText('Loading...')).toBeInTheDocument();

    await act(async () => {
      resolvePromise({
        ok: true,
        json: () => Promise.reject(new Error('json parsing failed'))
      });
    });

    await waitFor(() => {
      expect(screen.getByText('Article Not Found')).toBeInTheDocument();
    });
    unmount();
  });
});


describe('HelpArticlePage error state back button', () => {
  let consoleErrorMock: any;
  beforeAll(() => {
    consoleErrorMock = vi.spyOn(console, 'error').mockImplementation(() => {});
  });
  afterAll(() => {
    consoleErrorMock.mockRestore();
  });
  it('navigates back when clicking the back button in error state', async () => {
    let resolvePromise: any;
    const promise = new Promise(resolve => { resolvePromise = resolve; });
    global.fetch = vi.fn().mockImplementation(() => promise);

    const user = userEvent.setup();
    render(<HelpArticlePage />);

    await act(async () => {
      resolvePromise({
        ok: false,
        json: () => Promise.resolve({})
      });
    });

    await waitFor(() => {
      expect(screen.getByText('Article Not Found')).toBeInTheDocument();
    });

    const backButtons = screen.getAllByRole('button', { name: /Back to Help Center/i });
    await user.click(backButtons[0]);

    expect(mockPush).toHaveBeenCalledWith('/help');
  });
});



const mockPush = vi.fn();
const mockUseParams = vi.fn(() => ({ articleId: 'getting-started' }));
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
  useParams: () => mockUseParams()
}));

describe('HelpArticlePage without articleId', () => {
  it('renders loading state when articleId is missing', () => {
    mockUseParams.mockReturnValueOnce({});
    render(<HelpArticlePage />);
    expect(screen.getByText('Loading...')).toBeInTheDocument();
  });
});
