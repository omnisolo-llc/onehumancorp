import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import userEvent from '@testing-library/user-event';
import { HelpWidget, WalkthroughProvider } from './help';
import { TooltipProvider } from './TooltipRegistry';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
    replace: vi.fn(),
    prefetch: vi.fn(),
    back: vi.fn(),
    forward: vi.fn(),
    refresh: vi.fn(),
  }),
  usePathname: () => '/',
  useSearchParams: () => new URLSearchParams(),
}));

describe('HelpWidget', () => {
  beforeEach(() => {
    // Override the global fetch mock for these tests
    global.fetch = vi.fn().mockImplementation((url: string) => {
      if (url === '/api/videos') {
        return Promise.resolve(new Response(JSON.stringify([
          { id: 1, title: 'Test Video 1', duration: '1:23' },
          { id: 2, title: 'Test Video 2', duration: '2:34' },
        ]), { status: 200 }));
      }
      if (url === '/api/help') {
        return Promise.resolve(new Response(JSON.stringify([]), { status: 200 }));
      }
      return Promise.resolve(new Response(JSON.stringify({}), { status: 200 }));
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('renders widget button and opens chat interface', async () => {
    const user = userEvent.setup();
    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    const openButton = screen.getByLabelText('Help');
    expect(openButton).toBeInTheDocument();

    await user.click(openButton);

    expect(screen.getByText('Help Center')).toBeInTheDocument();
  });

  it('navigates to videos tab and selects a video to play', async () => {
    const user = userEvent.setup();
    render(
      <TooltipProvider>
        <WalkthroughProvider>
          <HelpWidget />
        </WalkthroughProvider>
      </TooltipProvider>
    );

    // Open widget
    await user.click(screen.getByLabelText('Help'));

    // Click Videos tab
    const videosTab = screen.getByRole('button', { name: /Videos/i });
    await user.click(videosTab);

    // Wait for videos to load
    await waitFor(() => {
      expect(screen.getByText('Test Video 1')).toBeInTheDocument();
    });

    // Click a video to open the player modal
    await user.click(screen.getByText('Test Video 1'));

    // Assert player modal is open
    expect(screen.getByLabelText('Close video')).toBeInTheDocument();

    // Close the video player
    await user.click(screen.getByLabelText('Close video'));
    expect(screen.queryByLabelText('Close video')).not.toBeInTheDocument();
  });
});
