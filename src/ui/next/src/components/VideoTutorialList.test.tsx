import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
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
});
