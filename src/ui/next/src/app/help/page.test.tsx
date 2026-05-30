import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import HelpCenterPage from './page';

// Mock fetch for the API
global.fetch = vi.fn();

const mockArticles = [
  { title: "Getting Started", desc: "Learn how to setup.", link: "/help/getting-started" },
  { title: "Payments", desc: "Learn about payments.", link: "/help/payments" },
];

describe('HelpCenterPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    (global.fetch as any).mockResolvedValue({
      json: async () => mockArticles,
    });
  });

  it('renders correctly and fetches articles', async () => {
    render(<HelpCenterPage />);

    // Wait for articles to load
    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeInTheDocument();
      expect(screen.getByText('Payments')).toBeInTheDocument();
    });
  });

  it('filters articles based on search query', async () => {
    render(<HelpCenterPage />);

    // Wait for initial render
    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeInTheDocument();
      expect(screen.getByText('Payments')).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText('Search help articles...');

    // Search for "payment"
    fireEvent.change(searchInput, { target: { value: 'payment' } });

    // "Payments" should be visible, "Getting Started" should not
    expect(screen.getByText('Payments')).toBeInTheDocument();
    expect(screen.queryByText('Getting Started')).not.toBeInTheDocument();

    // Search for "setup"
    fireEvent.change(searchInput, { target: { value: 'setup' } });

    expect(screen.getByText('Getting Started')).toBeInTheDocument();
    expect(screen.queryByText('Payments')).not.toBeInTheDocument();

    // Search for something that doesn't exist
    fireEvent.change(searchInput, { target: { value: 'nonexistent' } });
    expect(screen.getByText('No articles found matching your search.')).toBeInTheDocument();
  });
});
