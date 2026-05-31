import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import HelpCenterPage from './page';
import userEvent from '@testing-library/user-event';

vi.mock('next/link', () => {
  return {
    default: ({ children, href }: any) => <a href={href}>{children}</a>
  };
});

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

    expect(screen.getByText('Help Center')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeInTheDocument();
      expect(screen.getByText('My Store')).toBeInTheDocument();
    });
  });

  it('filters articles based on search query', async () => {
    const user = userEvent.setup();
    render(<HelpCenterPage />);

    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText('Search for help articles...');
    await user.type(searchInput, 'products');

    await waitFor(() => {
      expect(screen.queryByText('Getting Started')).not.toBeInTheDocument();
      expect(screen.getByText('My Store')).toBeInTheDocument();
    });
  });
});
