vi.mock("next/link", () => ({ default: (props: any) => <a href={props.href}>{props.children}</a> }));
import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import HelpCenterPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';
import userEvent from '@testing-library/user-event';

describe('HelpCenterPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockImplementation(async (url) => {
      console.log("Mock fetch called with:", url);
      // Simulate network delay to prevent React state batching issues during test
      await new Promise(r => setTimeout(r, 0));
      if (url === '/api/help') {
        return {
          ok: true,
          json: async () => [
            { title: "Getting Started", desc: "Learn how to easily set up your store and accept your first payment.", link: "/help/getting-started-1", category: "Getting Started" },
            { title: "Adding Products", desc: "Add products, track what's in stock, and change how your store looks.", link: "/help/my-store", category: "My Store" }
          ]
        };
      }
      if (typeof url === 'string' && url.includes('/api/help/search')) {
        let q = '';
        try {
          const urlObj = new URL(url.startsWith('http') ? url : 'http://localhost' + url);
          q = urlObj.searchParams.get('q')?.toLowerCase() || '';
        } catch (e) {
          const match = url.match(/q=([^&]*)/);
          q = match ? decodeURIComponent(match[1]).toLowerCase() : '';
        }

        const allArticles = [
            { title: "Getting Started", desc: "Learn how to easily set up your store and accept your first payment.", link: "/help/getting-started-1", category: "Getting Started" },
            { title: "Adding Products", desc: "Add products, track what's in stock, and change how your store looks.", link: "/help/my-store", category: "My Store" }
        ];
        const results = allArticles.filter(a =>
          a.title.toLowerCase().includes(q) ||
          a.desc.toLowerCase().includes(q)
        );
        return {
          ok: true,
          json: async () => results
        };
      }
      if (url === '/api/videos') {
        return {
          ok: true,
          json: async () => [
            { id: 1, title: "How to set up your first store easily", duration: "1:20" },
            { id: 2, title: "Linking your own website name", duration: "0:45" }
          ]
        };
      }
      return {
        ok: true,
        json: async () => []
      };
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('renders articles loaded from API', async () => {
    render(<TooltipProvider><HelpCenterPage /></TooltipProvider>);

    expect(screen.getByText('Help Center')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('Getting Started', { selector: 'h3' })).toBeInTheDocument();
      expect(screen.getByText('Adding Products', { selector: 'h3' })).toBeInTheDocument();
    }, { timeout: 3000 });
  });

  it('filters articles based on search query', async () => {
    const user = userEvent.setup();
    render(<TooltipProvider><HelpCenterPage /></TooltipProvider>);

    await waitFor(() => {
      expect(screen.getByText('Getting Started', { selector: 'h3' })).toBeInTheDocument();
    }, { timeout: 3000 });

    const searchInput = screen.getByPlaceholderText('Search for help articles and videos...');
    await user.type(searchInput, 'products');

    await waitFor(() => {
      expect(screen.queryByText('Getting Started', { selector: 'h3' })).not.toBeInTheDocument();
      expect(screen.getByText('Adding Products', { selector: 'h3' })).toBeInTheDocument();
    }, { timeout: 3000 });
  });

  it('displays no matching articles message when search fails', async () => {
    const user = userEvent.setup();
    render(<TooltipProvider><HelpCenterPage /></TooltipProvider>);

    await waitFor(() => {
      expect(screen.getByText('Getting Started', { selector: 'h3' })).toBeInTheDocument();
    }, { timeout: 3000 });

    const searchInput = screen.getByPlaceholderText('Search for help articles and videos...');
    await user.type(searchInput, 'nonexistentxyz123');

    await waitFor(() => {
      expect(screen.getByText(/No results found matching/)).toBeInTheDocument();
      expect(screen.getByText(/"nonexistentxyz123"/)).toBeInTheDocument();
    }, { timeout: 3000 });
  });

  it('renders video tutorials loaded from API', async () => {
    render(<TooltipProvider><HelpCenterPage /></TooltipProvider>);

    await waitFor(() => {
      expect(screen.getByText('How to set up your first store easily')).toBeInTheDocument();
      expect(screen.getByText('Linking your own website name')).toBeInTheDocument();
    }, { timeout: 3000 });
  });

  it('opens and closes the video modal', async () => {
    const user = userEvent.setup();
    render(<TooltipProvider><HelpCenterPage /></TooltipProvider>);
    await waitFor(() => {
      expect(screen.getByText('How to set up your first store easily')).toBeInTheDocument();
    }, { timeout: 3000 });
    // The video card container has changed in VideoTutorialList
    const videoTitle = screen.getByText('How to set up your first store easily');
    const videoCard = videoTitle.closest('.app-card');
    if (videoCard) {
      await user.click(videoCard);
    }
    await waitFor(() => {
      expect(screen.getByLabelText('Close video')).toBeInTheDocument();
    });
    const closeBtn = screen.getByLabelText('Close video');
    await user.click(closeBtn);
    await waitFor(() => {
      expect(screen.queryByLabelText('Close video')).not.toBeInTheDocument();
    });
  });

  it('renders correctly when there are no matching results at all', async () => {
    const user = userEvent.setup();
    render(<TooltipProvider><HelpCenterPage /></TooltipProvider>);
    await waitFor(() => {
      expect(screen.getByText('Getting Started', { selector: 'h3' })).toBeInTheDocument();
    }, { timeout: 3000 });

    const searchInput = screen.getByPlaceholderText('Search for help articles and videos...');
    await user.type(searchInput, 'nonexistentxyz123');

    await waitFor(() => {
      expect(screen.getByText(/No results found matching/)).toBeInTheDocument();
      expect(screen.queryByText('Video Tutorials')).not.toBeInTheDocument();
    }, { timeout: 3000 });
  });
});
