import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, act, waitFor, fireEvent } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { HelpWidget, WalkthroughProvider } from './help';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { TooltipProvider } from './TooltipRegistry';

vi.mock("next/navigation", () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('HelpWidget', () => {
  beforeEach(() => {
    window.HTMLElement.prototype.scrollIntoView = vi.fn();
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url.includes('/api/walkthrough/store-setup')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve([{ targetId: 'test-target', title: 'Test', content: 'Test Content' }])
        });
      }
      if (url.includes('/api/walkthrough/pos')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve([{ targetId: 'test-target-pos', title: 'Mock Walkthrough', content: 'Mock Content' }])
        });
      }
      if (url.includes('/api/walkthrough/assistant')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve([])
        });
      }
      if (url.includes('/api/walkthrough/meeting-room')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve(null) // test null fallback
        });
      }
      if (url.includes('/api/help/search')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve([])
        });
      }
      if (url.includes('/api/help')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve([
            { title: "Test Article", desc: "A test article", link: "/help/test", category: "Test Category" },
            { title: "Another Article", desc: "A test article 2", link: "/help/test2" } // No category
          ])
        });
      }
      if (url.includes('/api/videos')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve([
            { id: 1, title: "Test Video", duration: "1:00", video_url: "http://example.com/video.mp4" }
          ])
        });
      }
      if (url.includes('/api/changelog')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve([
            { version: "1.0", contentLines: ["- Added feature A", "### Big update"], screenshot_url: "" }
          ])
        });
      }
      if (url.includes('/api/chat')) {
        return Promise.resolve({
          ok: true,
          json: () => Promise.resolve({ reply: "Hello from AI", link: { url: "https://example.com", title: "Example" } })
        });
      }
      return Promise.resolve({
        ok: true,
        json: () => Promise.resolve([])
      });
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders the help widget', async () => {
    render(<TooltipProvider><WalkthroughProvider><HelpWidget /></WalkthroughProvider></TooltipProvider>);
    expect(screen.getByRole('button', { name: 'Help' })).toBeInTheDocument();
  });

  it('fetches dynamic walkthroughs when clicked and handles fallback data', async () => {
    const user = userEvent.setup();
    render(<div><div id="test-target">Mock Target</div><TooltipProvider><WalkthroughProvider><HelpWidget /></WalkthroughProvider></TooltipProvider></div>);

    const helpBtn = screen.getByRole('button', { name: 'Help' });
    await user.click(helpBtn);

    const tourBtn = screen.getByText('Tour: Set up your store');
    await user.click(tourBtn);
    expect(global.fetch).toHaveBeenCalledWith('/api/walkthrough/store-setup');

    // Test fallback behavior
    await user.click(helpBtn);
    const tourBtn3 = screen.getByText('Tour: Activate your AI Support Agent');
    await user.click(tourBtn3);

    await user.click(helpBtn);
    const tourBtn4 = screen.getByText('Tour: Virtual Meeting Room & UltraPlan');
    await user.click(tourBtn4);

    await user.click(helpBtn);
    const tourBtn5 = screen.getByText('Tour: KAIROS AI OS Orchestration');
    await user.click(tourBtn5);
  });

  it('switches to the Ask anything tab and submits a message', async () => {
    const user = userEvent.setup();
    render(<div><TooltipProvider><WalkthroughProvider><HelpWidget /></WalkthroughProvider></TooltipProvider></div>);

    const helpBtn = screen.getByRole('button', { name: 'Help' });
    await user.click(helpBtn);

    const chatTab = screen.getByText('Ask anything');
    await user.click(chatTab);

    expect(screen.getByPlaceholderText('Ask anything...')).toBeInTheDocument();

    const input = screen.getByPlaceholderText('Ask anything...');
    await user.type(input, 'Test message');

    const sendBtn = screen.getByRole('button', { name: 'Send message' });
    await user.click(sendBtn);

    await waitFor(() => {
        expect(screen.getByText('Test message')).toBeInTheDocument();
        expect(screen.getByText('Hello from AI')).toBeInTheDocument();
    });
  });

  it('handles empty chat message gracefully', async () => {
    const user = userEvent.setup();
    render(<div><TooltipProvider><WalkthroughProvider><HelpWidget /></WalkthroughProvider></TooltipProvider></div>);

    const helpBtn = screen.getByRole('button', { name: 'Help' });
    await user.click(helpBtn);

    const chatTab = screen.getByText('Ask anything');
    await user.click(chatTab);

    const sendBtn = screen.getByRole('button', { name: 'Send message' });
    expect(sendBtn).toBeDisabled();
  });

  it('switches to the Videos tab and plays a video', async () => {
    const user = userEvent.setup();
    render(<div><TooltipProvider><WalkthroughProvider><HelpWidget /></WalkthroughProvider></TooltipProvider></div>);

    const helpBtn = screen.getByRole('button', { name: 'Help' });
    await user.click(helpBtn);

    const videosTab = screen.getByText('Videos');
    await user.click(videosTab);

    await waitFor(() => {
        expect(screen.getByText('Test Video')).toBeInTheDocument();
    });

    const videoCard = screen.getByText('Test Video').parentElement?.parentElement;
    if (videoCard) {
      await user.click(videoCard);
    }

    await waitFor(() => {
      expect(screen.getByLabelText('Close video')).toBeInTheDocument();
    });

    await user.click(screen.getByLabelText('Close video'));

    await waitFor(() => {
        expect(screen.queryByLabelText('Close video')).not.toBeInTheDocument();
    });
  });

  it('switches to the What is New tab and renders content', async () => {
    const user = userEvent.setup();
    render(<div><TooltipProvider><WalkthroughProvider><HelpWidget /></WalkthroughProvider></TooltipProvider></div>);

    const helpBtn = screen.getByRole('button', { name: 'Help' });
    await user.click(helpBtn);

    const newTab = screen.getByText('New');
    await user.click(newTab);

    await waitFor(() => {
        expect(screen.getByText("What's New")).toBeInTheDocument();
        expect(screen.getByText("New AI Store Builder")).toBeInTheDocument();
    });
  });

  it('renders articles and handles search', async () => {
    const user = userEvent.setup();
    render(<div><TooltipProvider><WalkthroughProvider><HelpWidget /></WalkthroughProvider></TooltipProvider></div>);

    const helpBtn = screen.getByRole('button', { name: 'Help' });
    await user.click(helpBtn);

    await waitFor(() => {
        expect(screen.getByText('Test Article')).toBeInTheDocument();
        expect(screen.getByText('Another Article')).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText('Search for help...');
    await user.type(searchInput, 'nonexistent');

    await waitFor(() => {
        expect(screen.queryByText('Test Article')).not.toBeInTheDocument();
        expect(screen.queryByText('Another Article')).not.toBeInTheDocument();
    });
  });

  it('closes the help widget when clicking close button', async () => {
    const user = userEvent.setup();
    render(<div><TooltipProvider><WalkthroughProvider><HelpWidget /></WalkthroughProvider></TooltipProvider></div>);

    const helpBtn = screen.getByRole('button', { name: 'Help' });
    await user.click(helpBtn);

    const closeBtn = screen.getByLabelText('Close Help Widget');
    await user.click(closeBtn);

    expect(screen.queryByText('In-App Help Center')).not.toBeInTheDocument();
  });

  it('opens chat on open-help-chat event', async () => {
    render(<div><TooltipProvider><WalkthroughProvider><HelpWidget /></WalkthroughProvider></TooltipProvider></div>);

    act(() => {
        window.dispatchEvent(new CustomEvent('open-help-chat'));
    });

    expect(screen.getByPlaceholderText('Ask anything...')).toBeInTheDocument();
  });

  it('handles chat fetch error', async () => {
    global.fetch = vi.fn().mockImplementation((url) => {
        if (url.includes('/api/chat')) {
            return Promise.resolve({
              ok: false,
              json: () => Promise.resolve({})
            });
        }
        return Promise.resolve({ ok: true, json: () => Promise.resolve([]) });
    });

    const user = userEvent.setup();
    render(<div><TooltipProvider><WalkthroughProvider><HelpWidget /></WalkthroughProvider></TooltipProvider></div>);

    const helpBtn = screen.getByRole('button', { name: 'Help' });
    await user.click(helpBtn);

    const chatTab = screen.getByText('Ask anything');
    await user.click(chatTab);

    const input = screen.getByPlaceholderText('Ask anything...');
    await user.type(input, 'Test message fail');

    const sendBtn = screen.getByRole('button', { name: 'Send message' });
    await user.click(sendBtn);

    await waitFor(() => {
        expect(screen.getByText("Sorry, I'm having trouble connecting right now.")).toBeInTheDocument();
    });
  });

});
