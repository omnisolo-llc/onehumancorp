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
  let originalFetch: typeof global.fetch;

  beforeEach(() => {
    vi.clearAllMocks();
    originalFetch = global.fetch;
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

  afterEach(() => {
    global.fetch = originalFetch;
  });

  it('renders correctly', async () => {
    render(<DashboardViralInviteWidget />);

    expect(screen.getByText('Invite & Earn')).toBeDefined();
    expect(screen.getByText(/Invite a fellow business owner to OHC/)).toBeDefined();

    const generateBtn = screen.getByRole('button', { name: 'Get My Invite Link' });
    expect(generateBtn).toBeDefined();
  });

  it('generates link, copies to clipboard, and shares to X', async () => {
    // Mock the fetch call for the generation
    const mockFetch = vi.fn().mockResolvedValue({
      json: async () => ({ referral_link: 'https://ohc.app/ref/test-tenant-123' }),
    });
    global.fetch = mockFetch;

    render(<DashboardViralInviteWidget />);

    const generateBtn = screen.getByRole('button', { name: 'Get My Invite Link' });
    fireEvent.click(generateBtn);

    // Wait for the input and buttons to appear
    await waitFor(() => {
      expect(screen.getByDisplayValue(/test-tenant-123/)).toBeDefined();
    });

    const copyButton = screen.getByRole('button', { name: 'Copy' });
    fireEvent.click(copyButton);

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith(
      expect.stringContaining('https://ohc.app/ref/test-tenant-123')
    );
    expect(screen.getByText('Copied!')).toBeDefined();

    // Verify Share on X button exists and its function
    const xButton = screen.getByRole('button', { name: 'Share on X' });
    expect(xButton).toBeDefined();

    const openSpy = vi.spyOn(window, 'open').mockImplementation(() => null as any);
    fireEvent.click(xButton);
    expect(openSpy).toHaveBeenCalledWith(expect.stringContaining('twitter.com/intent/tweet'), '_blank');
  });
});
