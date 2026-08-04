
import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ChangelogPage from './page';

describe('ChangelogPage', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it('renders the release notes page correctly', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve([{
        version: "Version 1.0 (Latest)",
        contentLines: [
          "### 🌟 New Features",
          "- **Interactive AI Store Builder:** You can now generate a complete storefront from just a short description of your business. AI will handle the layout and copy for you.",
          "- **Smart Tooltips:** We added helpful text bubbles to all major buttons to help you learn the system faster.",
          "Faster loading times for product images."
        ]
      }]),
      ok: true
    }) as any;

    await act(async () => {
      render(<ChangelogPage />);
    });

    expect(screen.getByText('Release Notes & Changelog')).toBeInTheDocument();

    await waitFor(() => {
      expect(screen.getByText('Version 1.0 (Latest)')).toBeInTheDocument();
    });

    // Check for some content points
    expect(screen.getByText(/Interactive AI Store Builder:/)).toBeInTheDocument();
    expect(screen.getByText(/Smart Tooltips:/)).toBeInTheDocument();
  });

  it('renders links correctly', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve([{
        version: "Version 1.1",
        contentLines: [
          "Here is a link to our [website](https://example.com)!",
          "- List item with [link](https://test.com)"
        ]
      }]),
      ok: true
    }) as any;

    await act(async () => {
      render(<ChangelogPage />);
    });

    await waitFor(() => {
      expect(screen.getByText('Version 1.1')).toBeInTheDocument();
    });

    const websiteLink = screen.getByText('website');
    expect(websiteLink).toHaveAttribute('href', 'https://example.com');

    const testLink = screen.getByText('link');
    expect(testLink).toHaveAttribute('href', 'https://test.com');
  });

  it('renders correctly with screenshot_url', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve([{
        version: "Version 1.2",
        contentLines: [
          "New feature with screenshot."
        ],
        screenshot_url: "https://example.com/image.png"
      }]),
      ok: true
    }) as any;

    await act(async () => {
      render(<ChangelogPage />);
    });

    await waitFor(() => {
      expect(screen.getByText('Version 1.2')).toBeInTheDocument();
    });

    const img = screen.getByAltText('Version 1.2 Screenshot');
    expect(img).toBeInTheDocument();
    expect(img).toHaveAttribute('src', 'https://example.com/image.png');
  });

  it('renders empty string array properly', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve([{
        version: "Version 1.3",
        contentLines: [
          ""
        ]
      }]),
      ok: true
    }) as any;

    await act(async () => {
      render(<ChangelogPage />);
    });

    await waitFor(() => {
      expect(screen.getByText('Version 1.3')).toBeInTheDocument();
    });
  });

  it('renders links at start of line', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve([{
        version: "Version 1.4",
        contentLines: [
          "[start](https://example.com) text"
        ]
      }]),
      ok: true
    }) as any;

    await act(async () => {
      render(<ChangelogPage />);
    });

    await waitFor(() => {
      expect(screen.getByText('Version 1.4')).toBeInTheDocument();
    });

    const startLink = screen.getByText('start');
    expect(startLink).toHaveAttribute('href', 'https://example.com');
  });


  it('handles empty changelog gracefully', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve([]),
      ok: true
    }) as any;

    await act(async () => {
      render(<ChangelogPage />);
    });

    await waitFor(() => {
      expect(screen.getByText('No changelog available.')).toBeInTheDocument();
    });
  });

  it('handles fetch failure gracefully', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error("Network error"));

    await act(async () => {
      render(<ChangelogPage />);
    });

    await waitFor(() => {
      expect(screen.getByText('No changelog available.')).toBeInTheDocument();
    });
  });

  it('handles fetch error status gracefully', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false
    }) as any;

    await act(async () => {
      render(<ChangelogPage />);
    });

    await waitFor(() => {
      expect(screen.getByText('No changelog available.')).toBeInTheDocument();
    });
  });

  it('handles when data is not an array', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve({ data: "not array" }),
      ok: true
    }) as any;

    await act(async () => {
      render(<ChangelogPage />);
    });

    await waitFor(() => {
      expect(screen.getByText('No changelog available.')).toBeInTheDocument();
    });
  });
});
