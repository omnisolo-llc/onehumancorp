import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ChangelogPage from './page';

describe('ChangelogPage', () => {
  beforeEach(() => {
    global.fetch = vi.fn().mockImplementation(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve([
          {
            version: "Version 1.0 (Latest)",
            contentLines: [
              "### 🌟 New Features",
              "- **Interactive AI Store Builder:** You can now generate a complete storefront.",
              "- **Smart Tooltips:** We added helpful text bubbles."
            ]
          }
        ])
      })
    );
  });

  it('renders the release notes page correctly', async () => {
    render(<ChangelogPage />);

    await waitFor(() => {
      expect(screen.getByText('Release Notes & Changelog')).toBeInTheDocument();
    });
    expect(screen.getByText('Version 1.0 (Latest)')).toBeInTheDocument();

    // Check for some content points
    expect(screen.getByText(/Interactive AI Store Builder:/)).toBeInTheDocument();
    expect(screen.getByText(/Smart Tooltips:/)).toBeInTheDocument();
  });
});
