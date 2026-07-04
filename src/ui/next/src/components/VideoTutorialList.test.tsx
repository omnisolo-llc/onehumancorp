import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { VideoTutorialList } from './VideoTutorialList';

describe('VideoTutorialList', () => {
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
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve([
        { id: 1, title: "Test Video 1", duration: "1:23" },
        { id: 2, title: "Test Video 2", duration: "4:56" }
      ])
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
      expect(screen.getByText('No video tutorials match your search.')).toBeInTheDocument();
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
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve([
        { id: 1, title: "Modal Video", duration: "1:23", video_url: "http://example.com/vid.mp4" }
      ])
    });

    render(<VideoTutorialList />);

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
