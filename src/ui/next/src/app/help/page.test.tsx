import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
vi.mock("next/link", () => ({ default: (props: any) => React.createElement("a", { href: props.href }, props.children) }));
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import HelpCenterPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';
import userEvent from '@testing-library/user-event';

describe('HelpCenterPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url === '/api/tooltips') {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ "test-id": "Tooltip text" })
        });
      }
      if (url === '/api/help') {
        return Promise.resolve({
          json: () => Promise.resolve([
            { title: "Getting Started", desc: "Learn how to easily set up your store and accept your first payment.", link: "/help/getting-started-1", category: "General" },
            { title: "Adding Products", desc: "Add products, track what's in stock, and change how your store looks.", link: "/help/my-store", category: "General" },
            { title: "API Documentation", desc: "Advanced.", link: "/api-docs", category: "Advanced" }
          ])
        });
      }
      if (typeof url === 'string' && url.includes('/api/help/search')) {
        const urlObj = new URL('http://localhost' + url);
        const q = urlObj.searchParams.get('q')?.toLowerCase() || '';
        const allArticles = [
            { title: "Getting Started", desc: "Learn how to easily set up your store and accept your first payment.", link: "/help/getting-started-1" },
            { title: "Adding Products", desc: "Add products, track what's in stock, and change how your store looks.", link: "/help/my-store" }
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

    expect(screen.getByText('In-App Help Center')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('Getting Started')).toBeInTheDocument();
      expect(screen.getByText('Adding Products')).toBeInTheDocument();
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
      expect(screen.getByText('Adding Products')).toBeInTheDocument();
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
      expect(screen.getByText(/No results found matching/)).toBeInTheDocument();
      expect(screen.getByText(/"nonexistentxyz123"/)).toBeInTheDocument();
    });
  });

  it('renders video tutorials loaded from API', async () => {
    render(<TooltipProvider><HelpCenterPage /></TooltipProvider>);

    await waitFor(() => {
      expect(screen.getByText('How to set up your first store easily')).toBeInTheDocument();
      expect(screen.getByText('Linking your own website name')).toBeInTheDocument();
    });
  });

  it('opens and closes the video modal', async () => {
    const user = userEvent.setup();
    render(<TooltipProvider><HelpCenterPage /></TooltipProvider>);
    await waitFor(() => {
      expect(screen.getByText('How to set up your first store easily')).toBeInTheDocument();
    });
    // The video card container has changed in VideoTutorialList
    const videoTitle = screen.getByText('How to set up your first store easily');
    const videoCard = videoTitle.parentElement?.parentElement;
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
      expect(screen.getByText('Getting Started')).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText('Search for help articles and videos...');
    await user.type(searchInput, 'nonexistentxyz123');

    await waitFor(() => {
      expect(screen.getByText(/No results found matching/)).toBeInTheDocument();
      expect(screen.queryByText('Video Tutorials')).not.toBeInTheDocument();
    });
  });
});
