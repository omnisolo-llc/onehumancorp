import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { VideoTutorialList } from './VideoTutorialList';
import userEvent from '@testing-library/user-event';
import { TooltipProvider } from './TooltipRegistry';

describe('VideoTutorialList', () => {
  it.each(['pointer', 'Enter', 'Space'])('opens the exact source using a native %s control and closes through its button', async activation => {
    const user = userEvent.setup();
    const selected = { id: 71, title: 'Accept payments', duration: '0:45', video_url: '/owned-payment-tutorial.mp4' };
    const { container, rerender } = render(<VideoTutorialList videos={[selected]} loading={false} />);
    const play = screen.getByRole('button', { name: 'Play video: Accept payments' });
    expect(play.tagName).toBe('BUTTON');
    expect(play).toHaveAttribute('type', 'button');
    if (activation === 'pointer') await user.click(play);
    else {
      await user.tab(); // Search videos.
      await user.tab(); // The actual video control, not a synthetic DOM click.
      expect(play).toHaveFocus();
      await user.keyboard(activation === 'Enter' ? '{Enter}' : ' ');
    }
    const media = container.querySelector('video');
    expect(media).toBeVisible();
    expect(media).toHaveAttribute('src', selected.video_url);
    expect(media).toHaveAttribute('controls');
    expect(media).toHaveAttribute('autoplay');
    // A delayed search response can update/reflow the list without discarding the chosen source.
    rerender(<VideoTutorialList videos={[{ ...selected, id: 72, title: 'Another video', video_url: '/other.mp4' }, selected]} loading={false} />);
    expect(container.querySelector('video')).toHaveAttribute('src', selected.video_url);
    await user.click(screen.getByRole('button', { name: 'Close video' }));
    expect(container.querySelector('video')).not.toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Play video: Another video' }));
    expect(container.querySelector('video')).toHaveAttribute('src', '/other.mp4');
    await user.click(screen.getByRole('button', { name: 'Close video' }));
    expect(container.querySelector('video')).not.toBeInTheDocument();
  });

  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders loading state initially', () => {
    // Mock fetch to not resolve immediately
    global.fetch = vi.fn().mockImplementation(() => new Promise(() => {}));

    const { container } = render(<VideoTutorialList />);
    expect(container.querySelector('.animate-spin')).toBeInTheDocument();
  });

  it('renders videos correctly', async () => {
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url === '/api/v1/tooltips') {
        return Promise.resolve(Response.json({}, { status: 200 }));
      }
      return Promise.resolve({
        json: () => Promise.resolve([
          { id: 1, title: "Test Video 1", duration: "1:23" },
          { id: 2, title: "Test Video 2", duration: "4:56" }
        ])
      });
    });

    render(<VideoTutorialList />);

    await waitFor(() => {
      expect(screen.getByText('Test Video 1')).toBeInTheDocument();
      expect(screen.getByText('1:23')).toBeInTheDocument();
      expect(screen.getByText('Test Video 2')).toBeInTheDocument();
      expect(screen.getByText('4:56')).toBeInTheDocument();
    });
  });

  it('renders empty state when no videos are returned', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve([])
    });

    render(<VideoTutorialList />);

    await waitFor(() => {
      expect(screen.getByText('No video tutorials available right now.')).toBeInTheDocument();
    });
  });

  it('treats a non-array service error payload as an empty list', async () => {
    global.fetch = vi.fn().mockResolvedValue(Response.json({ error: 'backend unavailable' }, { status: 502 }));

    render(<VideoTutorialList />);

    await waitFor(() => {
      expect(screen.getByText('No video tutorials available right now.')).toBeInTheDocument();
    });
  });

  it('filters videos correctly based on search query', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve([
        { id: 1, title: "How to setup your store", duration: "1:23" },
        { id: 2, title: "Adding new products", duration: "4:56" }
      ])
    });

    render(<VideoTutorialList />);

    await waitFor(() => {
      expect(screen.getByText('How to setup your store')).toBeInTheDocument();
      expect(screen.getByText('Adding new products')).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText('Search videos...');
    fireEvent.change(searchInput, { target: { value: 'setup' } });

    await waitFor(() => {
      expect(screen.getByText('How to setup your store')).toBeInTheDocument();
      expect(screen.queryByText('Adding new products')).not.toBeInTheDocument();
    });
  });

  it('renders empty search state when no videos match query', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve([
        { id: 1, title: "Test Video 1", duration: "1:23" }
      ])
    });

    render(<VideoTutorialList />);

    await waitFor(() => {
      expect(screen.getByText('Test Video 1')).toBeInTheDocument();
    });

    const searchInput = screen.getByPlaceholderText('Search videos...');
    fireEvent.change(searchInput, { target: { value: 'nonexistent' } });

    await waitFor(() => {
      expect(screen.queryByText('Test Video 1')).not.toBeInTheDocument();
      expect(screen.getByText(/No results found matching/)).toBeInTheDocument();
    });
  });
});

  it('handles fetch failure gracefully', async () => {
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    global.fetch = vi.fn().mockRejectedValue(new Error('Network error'));

    render(<VideoTutorialList />);

    await waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalledWith('Failed to load video tutorials', expect.any(Error));
    });

    expect(screen.getByText('No video tutorials available right now.')).toBeInTheDocument();

    consoleErrorSpy.mockRestore();
  });

  it('uses external videos and loading state', async () => {
    const externalVideos = [
      { id: 3, title: "External Video", duration: "2:00", video_url: "http://example.com/ext.mp4" }
    ];

    render(<VideoTutorialList videos={externalVideos} loading={false} />);

    expect(screen.getByText('External Video')).toBeInTheDocument();
  });

  it('opens and closes the video modal', async () => {
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url === '/api/v1/tooltips') {
        return Promise.resolve(Response.json({}, { status: 200 }));
      }
      return Promise.resolve({
        json: () => Promise.resolve([
          { id: 1, title: "Modal Video", duration: "1:23", video_url: "http://example.com/vid.mp4" }
        ])
      });
    });

    render(
      <TooltipProvider>
        <VideoTutorialList />
      </TooltipProvider>
    );

    await waitFor(() => {
      expect(screen.getByText('Modal Video')).toBeInTheDocument();
    });

    const videoTitle = screen.getByText('Modal Video');
    const videoCard = videoTitle.closest('.cursor-pointer');
    fireEvent.click(videoCard!);

    await waitFor(() => {
      expect(screen.getByLabelText('Close video')).toBeInTheDocument();
    });

    const closeBtn = screen.getByLabelText('Close video');
    fireEvent.click(closeBtn);

    await waitFor(() => {
      expect(screen.queryByLabelText('Close video')).not.toBeInTheDocument();
    });
  });
