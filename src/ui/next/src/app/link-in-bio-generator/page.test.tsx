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

vi.mock('../components/PoweredByOmniSolo', () => ({
  PoweredByOmniSolo: () => <div data-testid="powered-by-omnisolo" />,
}));

describe('LinkInBioGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    localStorage.clear();
    global.fetch = vi.fn(async (url, options) => {
      if (url === '/api/v1/auth/session-identity') return Response.json({userId:'member',tenantId:'owned-tenant',expiresAt:Date.now()+60_000});
      if (options?.method === 'POST') return new Response('',{status:200});
      return Response.json({store_name:'Existing Store',bio:'Existing Bio',theme:'dark',links:[{title:'Existing Link',url:'https://existing.com'}]});
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

  it('exposes exclusive theme selection and restores it through real theme clicks', async () => {
    await act(async () => { render(<LinkInBioGeneratorPage />); });
    const light = screen.getByRole('button', { name: 'Light' });
    const dark = screen.getByRole('button', { name: 'Dark' });
    expect(dark).toHaveAttribute('aria-pressed', 'true');
    expect(light).toHaveAttribute('aria-pressed', 'false');
    const preview = screen.getByRole('link', { name: 'Existing Link' });
    expect(preview).toHaveClass('bg-[#222222]');
    fireEvent.click(light);
    expect(light).toHaveAttribute('aria-pressed', 'true');
    expect(dark).toHaveAttribute('aria-pressed', 'false');
    expect(preview).toHaveClass('bg-white');
    fireEvent.click(dark);
    expect(dark).toHaveAttribute('aria-pressed', 'true');
    expect(light).toHaveAttribute('aria-pressed', 'false');
    expect(preview).toHaveClass('bg-[#222222]');
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

    const saveBtn = screen.getByText('Save private configuration');

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

    const copyBtn = screen.getByText('Copy saved private preview link');

    await act(async () => {
        fireEvent.click(copyBtn);
    });

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith(`${window.location.origin}/bio/owned-tenant`);
  });
});
