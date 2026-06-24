import React from 'react';
import { render, screen, fireEvent, act, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import LinkInBioGeneratorPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
    back: vi.fn(),
  }),
}));

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('LinkInBioGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: async () => ({
        store_name: 'Existing Store',
        bio: 'Existing Bio',
        theme: 'dark',
        links: [{ title: 'Existing Link', url: 'https://existing.com' }],
      }),
    });

    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockResolvedValue(undefined),
      },
    });
  });

  it('renders the configurator and loads config', async () => {
    await act(async () => {
        render(<LinkInBioGeneratorPage />);
    });

    expect(screen.getByText('Link in Bio Generator')).toBeDefined();

    await waitFor(() => {
        const titleInputs = screen.getAllByDisplayValue('Existing Store');
        expect(titleInputs.length).toBeGreaterThan(0);
        expect(screen.getByDisplayValue('Existing Bio')).toBeDefined();
        expect(screen.getByDisplayValue('Existing Link')).toBeDefined();
        expect(screen.getByDisplayValue('https://existing.com')).toBeDefined();
    });
  });

  it('adds and removes links', async () => {
    await act(async () => {
        render(<LinkInBioGeneratorPage />);
    });

    await waitFor(() => {
        expect(screen.getByDisplayValue('Existing Link')).toBeDefined();
    });

    const addLinkBtn = screen.getByText('+ Add Link');
    await act(async () => {
        fireEvent.click(addLinkBtn);
    });

    // We should now have 2 link blocks, the new one defaults to 'New Link'
    expect(screen.getByDisplayValue('New Link')).toBeDefined();

    // Now remove the first link (which has 'Existing Link')
    // Wait for the remove buttons to appear (there are 2 now)
    const removeBtns = screen.getAllByText('Remove');
    expect(removeBtns.length).toBe(2);

    await act(async () => {
        fireEvent.click(removeBtns[0]);
    });

    // 'Existing Link' should be gone, 'New Link' should remain
    expect(screen.queryByDisplayValue('Existing Link')).toBeNull();
    expect(screen.getByDisplayValue('New Link')).toBeDefined();
  });

  it('saves config', async () => {
    await act(async () => {
        render(<LinkInBioGeneratorPage />);
    });

    const saveBtn = screen.getByText('Save & Publish');

    await act(async () => {
        fireEvent.click(saveBtn);
    });

    expect(global.fetch).toHaveBeenCalledWith('/api/v1/growth/link-in-bio', expect.objectContaining({
      method: 'POST',
      body: expect.any(String),
    }));
  });

  it('copies link', async () => {
    await act(async () => {
        render(<LinkInBioGeneratorPage />);
    });

    const copyBtn = screen.getByText('Copy Link');

    await act(async () => {
        fireEvent.click(copyBtn);
    });

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith(expect.stringContaining('/bio/my-store'));
  });
});
