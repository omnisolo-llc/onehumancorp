import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import TutorialsPage from './page';
import { describe, it, expect, vi, beforeEach } from 'vitest';

global.fetch = vi.fn() as any;

// Mock next/link to avoid router context errors in test
vi.mock('next/link', () => {
  return {
    default: ({ children, href }: { children: React.ReactNode, href: string }) => {
      return <a href={href}>{children}</a>;
    }
  };
});

describe('TutorialsPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders loading state initially', () => {
    (global.fetch as any).mockImplementationOnce(() => new Promise(() => {})); // Never resolves
    render(<TutorialsPage />);
    expect(screen.getByText('Video Tutorials')).toBeInTheDocument();
  });

  it('renders videos after fetching', async () => {
    const mockVideos = [
      {
        id: 1,
        title: "Test Video 1",
        duration: "1:00",
        description: "Desc 1",
        url: "url1"
      },
      {
        id: 2,
        title: "Test Video 2",
        duration: "2:00",
        description: "Desc 2",
        url: "url2"
      }
    ];

    (global.fetch as any).mockResolvedValueOnce({
      json: async () => mockVideos,
    });

    render(<TutorialsPage />);

    await waitFor(() => {
      expect(screen.getByText('Test Video 1')).toBeInTheDocument();
      expect(screen.getByText('Test Video 2')).toBeInTheDocument();
    });
  });

  it('handles fetch error gracefully', async () => {
    (global.fetch as any).mockRejectedValueOnce(new Error('Network error'));
    render(<TutorialsPage />);

    await waitFor(() => {
      expect(screen.queryByText('Test Video 1')).not.toBeInTheDocument();
    });
  });
});