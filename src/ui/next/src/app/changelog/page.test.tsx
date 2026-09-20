
import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor, act } from '@testing-library/react';
import { beforeEach, describe, it, expect, vi } from 'vitest';
import ChangelogPage from './page';

describe('ChangelogPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockImplementation(() => Promise.resolve(Response.json([{
      version: "Version 1.0 (Latest)",
      contentLines: [
        "### 🌟 New Features",
        "- **Interactive AI Store Builder:** You can now generate a complete storefront from just a short description of your business. AI will handle the layout and copy for you.",
        "- **Smart Tooltips:** We added helpful text bubbles to all major buttons to help you learn the system faster.",
        "Faster loading times for product images."
      ]
    }], { status: 200 })));
  });

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
    expect(link).toHaveAttribute('href', 'https://cloud.omnisolo.co/changelog');
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
    // Re-render to ensure a fresh response is decoded for another mount.
    await act(async () => {
      render(<ChangelogPage />);
    });
    expect(await screen.findByText('Faster loading times for product images.')).toHaveProperty('tagName', 'SPAN');
    expect(screen.getByText('Faster loading times for product images.').closest('p')).not.toBeNull();
  });
});
