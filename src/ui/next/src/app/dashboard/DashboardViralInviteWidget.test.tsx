import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { DashboardViralInviteWidget } from './DashboardViralInviteWidget';
import React from 'react';

Object.assign(navigator, {
  clipboard: {
    writeText: vi.fn(),
  },
});

describe('DashboardViralInviteWidget', () => {
  let fetchMock: any;

  beforeEach(() => {
    vi.clearAllMocks();
    fetchMock = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ invite_link: "https://ohc.app/invite/test-tenant-123" }),
    });
    global.fetch = fetchMock;
    Object.defineProperty(window, 'localStorage', {
      value: {
        getItem: vi.fn((key) => {
          if (key === 'tenant_id') return 'test-tenant-123';
          return null;
        }),
      },
      writable: true,
    });
  });

  it('renders and generates link via API', async () => {
    render(<DashboardViralInviteWidget />);

    expect(screen.getByText('Refer & Earn $50')).toBeDefined();
    expect(screen.getByText(/Invite another business owner to OHC/)).toBeDefined();

    const generateBtn = screen.getByRole('button', { name: 'Generate Invite Link' });
    fireEvent.click(generateBtn);

    await waitFor(() => {
        expect(fetchMock).toHaveBeenCalledWith('/api/v1/growth/team-invites', expect.objectContaining({
            method: 'POST',
        }));
    });

    const input = await screen.findByDisplayValue('https://ohc.app/invite/test-tenant-123');
    expect(input).toBeDefined();
  });

  it('copies link to clipboard and shows Copied! text', async () => {
    render(<DashboardViralInviteWidget />);

    const generateBtn = screen.getByRole('button', { name: 'Generate Invite Link' });
    fireEvent.click(generateBtn);

    const copyButton = await screen.findByRole('button', { name: 'Copy Link' });
    fireEvent.click(copyButton);

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith(
      'https://ohc.app/invite/test-tenant-123'
    );
    expect(await screen.findByText('Copied!')).toBeDefined();
  });

  it('contains a valid WhatsApp share link', async () => {
    render(<DashboardViralInviteWidget />);

    const generateBtn = screen.getByRole('button', { name: 'Generate Invite Link' });
    fireEvent.click(generateBtn);

    const whatsappLink = await screen.findByRole('link', { name: /Share on WhatsApp/i });
    expect(whatsappLink.getAttribute('href')).toContain('wa.me');
    expect(whatsappLink.getAttribute('href')).toContain('test-tenant-123');
  });
});
