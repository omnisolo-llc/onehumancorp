import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor, fireEvent } from '@testing-library/react';
import HelpCenterPage from './page';
import { describe, it, expect, vi } from 'vitest';

global.fetch = vi.fn() as any;

describe('HelpCenterPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders articles and videos when fetched', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/help') {
        return Promise.resolve({
          ok: true,
          json: async () => [{ title: 'Art 1', desc: 'Desc 1', link: '/help/1' }]
        });
      }
      if (url === '/api/videos') {
        return Promise.resolve({
          ok: true,
          json: async () => [{ id: 1, title: 'Vid 1', duration: '1:00' }]
        });
      }
      return Promise.resolve({ ok: true, json: async () => ([]) });
    });

    render(<HelpCenterPage />);

    await waitFor(() => {
      expect(screen.getByText('Art 1')).toBeInTheDocument();
      expect(screen.getByText('Vid 1')).toBeInTheDocument();
    });
  });

  it('filters results with search', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/help') {
        return Promise.resolve({
          ok: true,
          json: async () => [
            { title: 'Art 1', desc: 'Desc 1', link: '/help/1' },
            { title: 'Other', desc: 'Other desc', link: '/help/2' }
          ]
        });
      }
      if (url === '/api/videos') {
        return Promise.resolve({
          ok: true,
          json: async () => [{ id: 1, title: 'Vid 1', duration: '1:00' }]
        });
      }
      return Promise.resolve({ ok: true, json: async () => ([]) });
    });

    render(<HelpCenterPage />);

    await waitFor(() => {
      expect(screen.getByText('Other')).toBeInTheDocument();
    });

    const input = screen.getByPlaceholderText(/search/i);
    fireEvent.change(input, { target: { value: 'Art' } });

    expect(screen.queryByText('Other')).not.toBeInTheDocument();
    expect(screen.getByText('Art 1')).toBeInTheDocument();
  });

  it('shows empty state when no results match', async () => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/help') {
        return Promise.resolve({
          ok: true,
          json: async () => [{ title: 'Art 1', desc: 'Desc 1', link: '/help/1' }]
        });
      }
      if (url === '/api/videos') {
        return Promise.resolve({
          ok: true,
          json: async () => [{ id: 1, title: 'Vid 1', duration: '1:00' }]
        });
      }
      return Promise.resolve({ ok: true, json: async () => ([]) });
    });

    render(<HelpCenterPage />);

    await waitFor(() => {
      expect(screen.getByText('Art 1')).toBeInTheDocument();
    });

    const input = screen.getByPlaceholderText(/search/i);
    fireEvent.change(input, { target: { value: 'Nonexistent' } });

    expect(screen.getByText(/No results found matching/i)).toBeInTheDocument();
  });
});
