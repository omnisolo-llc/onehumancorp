import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import VideosPage from './page';
import { describe, it, expect, vi } from 'vitest';

global.fetch = vi.fn(() =>
  Promise.resolve({
    json: () => Promise.resolve([
      { id: 1, title: "Test Video 1", duration: "1:20" },
      { id: 2, title: "Test Video 2", duration: "0:45" }
    ]),
  })
) as any;

describe('VideosPage Component', () => {
  it('renders videos fetched from API', async () => {
    render(<VideosPage />);

    expect(screen.getByText('Video Tutorials')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('Test Video 1')).toBeInTheDocument();
      expect(screen.getByText('1:20')).toBeInTheDocument();
      expect(screen.getByText('Test Video 2')).toBeInTheDocument();
      expect(screen.getByText('0:45')).toBeInTheDocument();
    });
  });
});
