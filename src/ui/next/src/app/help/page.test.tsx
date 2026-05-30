import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import HelpCenterPage from './page';
import { describe, it, expect, vi, beforeEach } from 'vitest';

global.fetch = vi.fn() as any;

describe('HelpCenterPage', () => {
  beforeEach(() => {
    vi.mocked(global.fetch).mockReset();
  });

  it('renders correctly and fetches articles', async () => {
    vi.mocked(global.fetch).mockResolvedValueOnce({
      ok: true,
      json: async () => [
        { title: "Article 1", desc: "Desc 1", link: "/help/article-1" },
        { title: "Article 2", desc: "Desc 2", link: "/help/article-2" }
      ]
    } as any);

    render(<HelpCenterPage />);

    expect(screen.getByText('Help Center')).toBeInTheDocument();
    expect(screen.getByPlaceholderText('Search for help articles...')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('Article 1')).toBeInTheDocument();
      expect(screen.getByText('Article 2')).toBeInTheDocument();
    });
  });

  it('filters articles based on search query', async () => {
    vi.mocked(global.fetch).mockResolvedValueOnce({
      ok: true,
      json: async () => [
        { title: "Apple", desc: "Red fruit", link: "/help/apple" },
        { title: "Banana", desc: "Yellow fruit", link: "/help/banana" }
      ]
    } as any);

    render(<HelpCenterPage />);

    await waitFor(() => {
      expect(screen.getByText('Apple')).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText('Search for help articles...');
    fireEvent.change(searchInput, { target: { value: 'Apple' } });

    expect(screen.getByText('Apple')).toBeInTheDocument();
    expect(screen.queryByText('Banana')).not.toBeInTheDocument();
  });

  it('shows no articles found message when search yields no results', async () => {
    vi.mocked(global.fetch).mockResolvedValueOnce({
      ok: true,
      json: async () => [
        { title: "Apple", desc: "Red fruit", link: "/help/apple" }
      ]
    } as any);

    render(<HelpCenterPage />);

    await waitFor(() => {
      expect(screen.getByText('Apple')).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText('Search for help articles...');
    fireEvent.change(searchInput, { target: { value: 'Orange' } });

    expect(screen.getByText('No articles found matching "Orange"')).toBeInTheDocument();
  });
});
