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
  beforeEach(() => {
    vi.clearAllMocks();
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

  it('renders correctly with tenant link', async () => {
    render(<DashboardViralInviteWidget />);

    expect(screen.getByText('Refer & Earn $50')).toBeDefined();
    expect(screen.getByText(/Invite another business owner to OHC/)).toBeDefined();

    const input = screen.getByDisplayValue(/test-tenant-123/);
    expect(input).toBeDefined();
  });

  it('copies link to clipboard and shows Copied! text', async () => {
    render(<DashboardViralInviteWidget />);

    const copyButton = screen.getByRole('button', { name: 'Copy Link' });
    fireEvent.click(copyButton);

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith(
      expect.stringContaining('test-tenant-123')
    );
    expect(screen.getByText('Copied!')).toBeDefined();
  });

  it('contains a valid WhatsApp share link', async () => {
    render(<DashboardViralInviteWidget />);

    const whatsappLink = screen.getByRole('link', { name: /Share on WhatsApp/i });
    expect(whatsappLink.getAttribute('href')).toContain('wa.me');
    expect(whatsappLink.getAttribute('href')).toContain('test-tenant-123');
  });
});
