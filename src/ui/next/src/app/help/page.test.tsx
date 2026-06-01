import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import HelpCenterPage from './page';
import userEvent from '@testing-library/user-event';

describe('HelpCenterPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve([
        { title: "Getting Started", desc: "Learn how to easily set up your store and accept your first payment.", link: "/help/getting-started" },
        { title: "My Store", desc: "Add products, track what's in stock, and change how your store looks.", link: "/help/my-store" }
      ])
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('renders articles loaded from API', async () => {
    render(<HelpCenterPage />);

    expect(screen.getByText('Help Center')).toBeTruthy();

    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeTruthy();
      expect(screen.getByText('My Store')).toBeTruthy();
    });
  });

  it('filters articles based on search query', async () => {
    const user = userEvent.setup();
    render(<HelpCenterPage />);

    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeTruthy();
    });

    const searchInput = screen.getByPlaceholderText('Search for help articles...');
    await user.type(searchInput, 'products');

    await waitFor(() => {
      expect(screen.queryByText('Getting Started')).not.toBeTruthy();
      expect(screen.getByText('My Store')).toBeTruthy();
    });
  });

  it('displays no matching articles message when search fails', async () => {
    const user = userEvent.setup();
    render(<HelpCenterPage />);

    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeTruthy();
    });

    const searchInput = screen.getByPlaceholderText('Search for help articles...');
    await user.type(searchInput, 'nonexistentxyz123');

    await waitFor(() => {
      expect(screen.getByText('No articles found matching "nonexistentxyz123"')).toBeTruthy();
    });
  });
});
