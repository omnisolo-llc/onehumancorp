import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import HelpCenterPage from './page';

describe('HelpCenterPage', () => {
  beforeEach(() => {
    vi.stubGlobal('fetch', vi.fn(() =>
      Promise.resolve({
        json: () => Promise.resolve([
          { title: "Getting Started", desc: "Learn how to start.", link: "/help/getting-started" },
          { title: "My Store", desc: "Add products.", link: "/help/my-store" },
        ])
      })
    ));
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders articles and filters by search', async () => {
    render(<HelpCenterPage />);

    // Wait for the fetch to complete and render
    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeDefined();
    });
    expect(screen.getByText('My Store')).toBeDefined();

    const searchInput = screen.getByPlaceholderText('Search for help articles...');
    fireEvent.change(searchInput, { target: { value: 'store' } });

    await waitFor(() => {
      expect(screen.queryByText('Getting Started')).toBeNull();
    });
    expect(screen.getByText('My Store')).toBeDefined();
  });

  it('displays empty state when no articles match', async () => {
    render(<HelpCenterPage />);

    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeDefined();
    });

    const searchInput = screen.getByPlaceholderText('Search for help articles...');
    fireEvent.change(searchInput, { target: { value: 'nonexistent' } });

    await waitFor(() => {
      expect(screen.getByText(/No articles found matching/)).toBeDefined();
    });
    expect(screen.getByText('"nonexistent"')).toBeDefined();

    const clearBtn = screen.getByText('Clear search');
    fireEvent.click(clearBtn);

    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeDefined();
    });
  });
});
