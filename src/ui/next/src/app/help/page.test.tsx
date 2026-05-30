import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import HelpCenterPage from './page';

global.fetch = vi.fn() as any;

describe('HelpCenterPage', () => {
  beforeEach(() => {
    (global.fetch as any).mockImplementation((url: string) => {
      if (url === '/api/help') {
        return Promise.resolve({
          json: () => Promise.resolve([
            { title: "Getting Started", desc: "Start here", link: "/getting-started" },
            { title: "Payments", desc: "Get paid", link: "/payments" }
          ])
        });
      }
      if (url === '/api/videos') {
        return Promise.resolve({
          json: () => Promise.resolve([
            { id: 1, title: "Video 1", duration: "1:00" },
            { id: 2, title: "Video 2", duration: "2:00" }
          ])
        });
      }
      return Promise.resolve({ json: () => Promise.resolve([]) });
    });
  });

  it('renders articles and filters them', async () => {
    render(<HelpCenterPage />);

    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeInTheDocument();
      expect(screen.getByText('Payments')).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText('Search help articles...');
    fireEvent.change(searchInput, { target: { value: 'payment' } });

    expect(screen.queryByText('Getting Started')).not.toBeInTheDocument();
    expect(screen.getByText('Payments')).toBeInTheDocument();
  });

  it('renders video tutorials', async () => {
    render(<HelpCenterPage />);

    await waitFor(() => {
      expect(screen.getByText('Video 1')).toBeInTheDocument();
      expect(screen.getByText('Video 2')).toBeInTheDocument();
    });
  });
});
