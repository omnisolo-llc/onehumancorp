
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

    expect(screen.getByText('Changelog Updates')).toBeInTheDocument();

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

// Component contract tests use controlled API responses; these are not real-stack E2E.
describe('Changelog response states', () => {
  it('announces loading until the request resolves', () => {
    global.fetch = vi.fn(() => new Promise<Response>(() => {}));
    render(<ChangelogPage />);
    expect(screen.getByRole('status')).toHaveTextContent('Loading changelog');
    expect(screen.queryByText('No changelog available.')).not.toBeInTheDocument();
  });

  it('uses the empty state only for a valid empty array', async () => {
    global.fetch = vi.fn().mockResolvedValue(Response.json([]));
    render(<ChangelogPage />);
    expect(await screen.findByText('No changelog available.')).toBeInTheDocument();
    expect(screen.queryByRole('alert')).not.toBeInTheDocument();
  });

  it.each([401, 403, 500])('shows HTTP %s as unavailable instead of empty', async (status) => {
    global.fetch = vi.fn().mockResolvedValue(Response.json({ error: 'unavailable' }, { status }));
    render(<ChangelogPage />);
    expect(await screen.findByRole('alert')).toHaveTextContent(`HTTP ${status}`);
    expect(screen.queryByText('No changelog available.')).not.toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Retry' })).toBeEnabled();
  });

  it.each([
    { error: 'not changelog content' },
    [{ version: 'v1', contentLines: 'not an array' }],
    [{ version: 1, contentLines: [] }],
    [{ version: 'v1', contentLines: [null] }],
    [{ version: 'v1', contentLines: [], screenshot_url: 123 }],
  ].map((payload) => [payload]))('rejects a malformed success payload: %j', async (payload) => {
    global.fetch = vi.fn().mockResolvedValue(Response.json(payload));
    render(<ChangelogPage />);
    expect(await screen.findByRole('alert')).toHaveTextContent('invalid changelog response');
    expect(screen.queryByText('No changelog available.')).not.toBeInTheDocument();
  });

  it('shows malformed JSON as an error rather than empty history', async () => {
    global.fetch = vi.fn().mockResolvedValue(new Response('{broken-json', { status: 200 }));
    render(<ChangelogPage />);
    expect(await screen.findByRole('alert')).toHaveTextContent('Unable to load changelog.');
    expect(screen.queryByText('No changelog available.')).not.toBeInTheDocument();
  });

  it('recovers from a network failure only after a successful retry', async () => {
    global.fetch = vi.fn()
      .mockRejectedValueOnce(new Error('Network disconnected'))
      .mockResolvedValueOnce(Response.json([{ version: 'Recorded release', contentLines: ['Recorded change'] }]));
    render(<ChangelogPage />);
    expect(await screen.findByRole('alert')).toHaveTextContent('Network disconnected');
    const { fireEvent } = await import('@testing-library/react');
    fireEvent.click(screen.getByRole('button', { name: 'Retry' }));
    expect(await screen.findByText('Recorded release')).toBeInTheDocument();
    expect(screen.getByText('Recorded change')).toBeInTheDocument();
    expect(screen.queryByRole('alert')).not.toBeInTheDocument();
    expect(global.fetch).toHaveBeenCalledTimes(2);
  });

  it('preserves headings, links, screenshots and literal markup from the API', async () => {
    global.fetch = vi.fn().mockResolvedValue(Response.json([{
      version: 'Recorded release',
      contentLines: ['### Release details', '- See [documentation](/help)', '<script>inert</script>'],
      screenshot_url: '/recorded-image.png',
    }]));
    const { container } = render(<ChangelogPage />);
    expect(await screen.findByRole('heading', { name: 'Release details' })).toBeInTheDocument();
    expect(screen.getByRole('link', { name: 'documentation' })).toHaveAttribute('href', '/help');
    expect(screen.getByRole('img', { name: 'Recorded release Screenshot' })).toHaveAttribute('src', '/recorded-image.png');
    expect(screen.getByText('<script>inert</script>')).toBeInTheDocument();
    expect(container.querySelector('script')).toBeNull();
  });
});
