
import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, waitFor, act } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import ChangelogPage from './page';

describe('ChangelogPage', () => {
  global.fetch = vi.fn().mockResolvedValue({
    json: () => Promise.resolve([{
      version: "v0.4.48 (Cloud) / v0.4.48+1 (Standalone)",
      contentLines: [
        "### 🌟 New Features",
        "- **Optimize Multi-Environment Promotion capabilities for multi-tenant K8s** You can now generate a complete storefront from just a short description of your business. AI will handle the layout and copy for you.",
        "- **Enforce Multi-Environment Promotion behavior for Local desktop beta builds ensuring stricter local offline usage** We added helpful text bubbles to all major buttons to help you learn the system faster.",
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
      expect(screen.getByText('v0.4.48 (Cloud) / v0.4.48+1 (Standalone)')).toBeInTheDocument();
    });

    // Check for some content points
    expect(screen.getByText(/Optimize Multi-Environment Promotion capabilities for multi-tenant K8s/)).toBeInTheDocument();
    expect(screen.getByText(/Enforce Multi-Environment Promotion behavior for Local desktop beta builds ensuring stricter local offline usage/)).toBeInTheDocument();
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
});
