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
    global.fetch = vi.fn().mockImplementation(() => new Promise(() => {}));
    const { container } = render(<VideoTutorialList />);
    expect(container.querySelector('.animate-spin')).toBeInTheDocument();
  });

  it('renders videos correctly and opens/closes video modal', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve([
        { id: 1, title: "Test Video 1", duration: "1:23" },
        { id: 2, title: "Test Video 2", duration: "4:56" }
      ])
    });

    const { container } = render(<VideoTutorialList />);

    await waitFor(() => {
      expect(screen.getByText('Test Video 1')).toBeInTheDocument();
      expect(screen.getByText('1:23')).toBeInTheDocument();
    });

    // Click to open video modal
    const videoCard = screen.getByText('Test Video 1').closest('.bg-white\\/80');
    expect(videoCard).not.toBeNull();
    fireEvent.click(videoCard!);

    // Check if modal is rendered
    await waitFor(() => {
      const videoElement = container.querySelector('video');
      expect(videoElement).toBeInTheDocument();
      expect(videoElement?.getAttribute('src')).toBe('/videos/1.mp4');
    });

    // Click close button
    const closeBtn = screen.getByLabelText('Close video');
    fireEvent.click(closeBtn);

    // Verify modal is closed
    await waitFor(() => {
      expect(container.querySelector('video')).not.toBeInTheDocument();
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

  it('handles fetch errors correctly', async () => {
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    global.fetch = vi.fn().mockRejectedValue(new Error('Network error'));

    render(<VideoTutorialList />);

    await waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalledWith('Failed to load video tutorials', expect.any(Error));
      expect(screen.getByText('No video tutorials available right now.')).toBeInTheDocument();
    });
    consoleErrorSpy.mockRestore();
  });
});
