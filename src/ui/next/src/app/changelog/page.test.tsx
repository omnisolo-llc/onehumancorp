import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ChangelogPage from './page';

describe('ChangelogPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn(() =>
      Promise.resolve({
        json: () =>
          Promise.resolve([
            {
              version: "Version 1.0 (Latest)",
              contentLines: [
                "### 🌟 New Features",
                "- **Interactive AI Store Builder:** You can now generate a complete storefront from just a short description of your business. AI will handle the layout and copy for you.",
                "- **Smart Tooltips:** We added helpful text bubbles to all major buttons to help you learn the system faster.",
                "- **Help Center Upgrade:** Find answers instantly with our new searchable Help Center.",
                "### 🛠️ Improvements",
                "- Faster loading times for product images.",
                "- Simplified checkout process for your customers.",
                "This is a plain paragraph test line."
              ]
            }
          ]),
      })
    ) as jest.Mock;
  });

  it('renders the release notes page correctly', async () => {
    render(<ChangelogPage />);

    expect(screen.getByText('Release Notes & Changelog')).toBeInTheDocument();

    await waitFor(() => {
        expect(screen.getByText('Version 1.0 (Latest)')).toBeInTheDocument();
    });

    // Check for some content points
    expect(screen.getByText(/Interactive AI Store Builder:/)).toBeInTheDocument();
    expect(screen.getByText(/Smart Tooltips:/)).toBeInTheDocument();
  });

  it('renders paragraph strings', async () => {
    render(<ChangelogPage />);

    await waitFor(() => {
        const link = screen.getByText('Read the full technical changelog on our website →');
        expect(link).toHaveAttribute('href', 'https://onehumancorp.com/changelog');
    });
  });

  it('renders paragraph elements for random text', async () => {
    render(<ChangelogPage />);
    await waitFor(() => {
        expect(screen.getByText(/Faster loading times for product images/)).toBeInTheDocument();
    });
  });

  it('covers the line 36 paragraph fallback', async () => {
    // Re-render to ensure we evaluate the branch where a line neither starts with ### nor -
    render(<ChangelogPage />);
    await waitFor(() => {
        expect(screen.getByText(/This is a plain paragraph test line./)).toBeInTheDocument();
    });
  });
});
