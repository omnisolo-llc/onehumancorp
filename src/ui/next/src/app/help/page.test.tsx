import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import HelpCenterPage from './page';
import { describe, it, expect, vi, beforeEach } from 'vitest';

describe('HelpCenterPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockImplementation(() =>
      Promise.resolve({
        json: () => Promise.resolve([
          { title: "Getting Started", desc: "Learn how to start.", link: "/help/getting-started" },
          { title: "My Store", desc: "Add products.", link: "/help/my-store" }
        ])
      })
    );
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders the help center and fetches articles', async () => {
    render(<HelpCenterPage />);
    expect(screen.getByText('Help Center')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeInTheDocument();
      expect(screen.getByText('My Store')).toBeInTheDocument();
    });
  });

  it('filters articles based on search query', async () => {
    render(<HelpCenterPage />);

    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText('Search for help articles...');
    fireEvent.change(searchInput, { target: { value: 'Started' } });

    expect(screen.getByText('Getting Started')).toBeInTheDocument();
    expect(screen.queryByText('My Store')).not.toBeInTheDocument();
  });

  it('shows no articles found message when query does not match', async () => {
    render(<HelpCenterPage />);

    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText('Search for help articles...');
    fireEvent.change(searchInput, { target: { value: 'XYZ123' } });

    expect(screen.getByText('No articles found matching "XYZ123"')).toBeInTheDocument();
  });
});
