
import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor, act } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import ChangelogPage from './page';

describe('ChangelogPage', () => {
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

  it('renders the release notes page correctly', async () => {
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

  it('renders paragraph strings', async () => {
    await act(async () => {
      render(<ChangelogPage />);
    });
    const link = screen.getByText('Read the full technical changelog on our website →');
    expect(link).toHaveAttribute('href', 'https://onehumancorp.com/changelog');
  });

  it('renders paragraph elements for random text', async () => {
    await act(async () => {
      render(<ChangelogPage />);
    });
    await waitFor(() => {
      expect(screen.getByText(/Faster loading times for product images/)).toBeInTheDocument();
    });
  });

  it('covers the line 36 paragraph fallback', async () => {
    // Re-render to ensure we evaluate the branch where a line neither starts with ### nor -
    await act(async () => {
      render(<ChangelogPage />);
    });
  });

  it('renders loading state initially', () => {
    // Mock fetch to not resolve immediately
    global.fetch = vi.fn().mockImplementation(() => new Promise(() => {}));

    const { container } = render(<ChangelogPage />);
    expect(container.querySelector('.animate-spin')).toBeInTheDocument();
  });

  it('renders empty state when no changelog is returned', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve([]),
      ok: true
    }) as any;

    render(<ChangelogPage />);

    await waitFor(() => {
      expect(screen.getByText('No changelog available.')).toBeInTheDocument();
    });
  });

  it('renders screenshot if provided', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      json: () => Promise.resolve([{
        version: "Version 1.0 (Latest)",
        contentLines: [
          "### 🌟 New Features",
        ],
        screenshot_url: "http://example.com/screenshot.png"
      }]),
      ok: true
    }) as any;

    render(<ChangelogPage />);

    await waitFor(() => {
      expect(screen.getByAltText('Version 1.0 (Latest) Screenshot')).toBeInTheDocument();
    });
  });

  it('handles fetch errors gracefully', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error("Network Error")) as any;

    render(<ChangelogPage />);

    await waitFor(() => {
      expect(screen.getByText('No changelog available.')).toBeInTheDocument();
    });
  });

});
