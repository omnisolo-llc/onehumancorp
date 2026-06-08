vi.mock("next/link", () => ({ default: (props: any) => <a href={props.href}>{props.children}</a> }));
import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import HelpCenterPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';
import userEvent from '@testing-library/user-event';

describe('HelpCenterPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url === '/api/help') {
        return Promise.resolve({
          json: () => Promise.resolve([
            { title: "Getting Started", desc: "Learn how to easily set up your store and accept your first payment.", link: "/help/getting-started-1" },
            { title: "My Store", desc: "Add products, track what's in stock, and change how your store looks.", link: "/help/my-store" }
          ])
        });
      }
      if (typeof url === 'string' && url.includes('/api/help/search')) {
        const urlObj = new URL('http://localhost' + url);
        const q = urlObj.searchParams.get('q')?.toLowerCase() || '';
        const allArticles = [
            { title: "Getting Started", desc: "Learn how to easily set up your store and accept your first payment.", link: "/help/getting-started-1" },
            { title: "My Store", desc: "Add products, track what's in stock, and change how your store looks.", link: "/help/my-store" }
        ];
        const results = allArticles.filter(a =>
          a.title.toLowerCase().includes(q) ||
          a.desc.toLowerCase().includes(q)
        );
        return Promise.resolve({
          json: () => Promise.resolve(results)
        });
      }
      if (url === '/api/videos') {
        return Promise.resolve({
          json: () => Promise.resolve([
            { id: 1, title: "How to set up your first store easily", duration: "1:20" },
            { id: 2, title: "Linking your own website name", duration: "0:45" }
          ])
        });
      }
      return Promise.resolve({
        json: () => Promise.resolve([])
      });
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('renders articles loaded from API', async () => {
    render(<TooltipProvider><HelpCenterPage /></TooltipProvider>);

    expect(screen.getByText('Help Center')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeInTheDocument();
      expect(screen.getByText('My Store')).toBeInTheDocument();
    });
  });

  it('filters articles based on search query', async () => {
    const user = userEvent.setup();
    render(<TooltipProvider><HelpCenterPage /></TooltipProvider>);

    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText('Search for help articles and videos...');
    await user.type(searchInput, 'products');

    await waitFor(() => {
      expect(screen.queryByText('Getting Started')).not.toBeInTheDocument();
      expect(screen.getByText('My Store')).toBeInTheDocument();
    });
  });

  it('displays no matching articles message when search fails', async () => {
    const user = userEvent.setup();
    render(<TooltipProvider><HelpCenterPage /></TooltipProvider>);

    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText('Search for help articles and videos...');
    await user.type(searchInput, 'nonexistentxyz123');

    await waitFor(() => {
      expect(screen.getByText('No results found matching "nonexistentxyz123"')).toBeInTheDocument();
    });
  });

  it('renders video tutorials loaded from API', async () => {
    render(<TooltipProvider><HelpCenterPage /></TooltipProvider>);

    await waitFor(() => {
      expect(screen.getByText('How to set up your first store easily')).toBeInTheDocument();
      expect(screen.getByText('Linking your own website name')).toBeInTheDocument();
    });
  });

  it('opens and closes the video modal when a video is clicked', async () => {
    const user = userEvent.setup();
    render(<TooltipProvider><HelpCenterPage /></TooltipProvider>);

    // Wait for the video to be rendered
    await waitFor(() => {
      expect(screen.getByText('How to set up your first store easily')).toBeInTheDocument();
    });

    // Verify modal is not currently open
    expect(screen.queryByLabelText('Close video')).not.toBeInTheDocument();

    // Click on the video
    const videoCard = screen.getByText('How to set up your first store easily').closest('div.aspect-\\[9\\/16\\]');
    if (videoCard) {
      await user.click(videoCard);
    }

    // Verify modal is open
    await waitFor(() => {
      expect(screen.getByLabelText('Close video')).toBeInTheDocument();
      // Test that the modal contains the video title
      const openVideoTitles = screen.getAllByText('How to set up your first store easily');
      expect(openVideoTitles.length).toBeGreaterThan(1); // One in the list, one in the modal
    });

    // Click on the close button
    const closeBtn = screen.getByLabelText('Close video');
    await user.click(closeBtn);

    // Verify modal is closed
    await waitFor(() => {
      expect(screen.queryByLabelText('Close video')).not.toBeInTheDocument();
    });
  });
});
