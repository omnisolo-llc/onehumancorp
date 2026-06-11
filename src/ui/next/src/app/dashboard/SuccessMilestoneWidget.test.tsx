import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { SuccessMilestoneWidget } from './SuccessMilestoneWidget';
import * as React from 'react';

// Mock clipboard
Object.assign(navigator, {
  clipboard: {
    writeText: vi.fn(),
  },
});

describe('SuccessMilestoneWidget', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.defineProperty(window, 'localStorage', {
      value: {
        getItem: vi.fn(() => 'test-tenant'),
      },
      writable: true
    });
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        title: "100th Order Delivered! 🎉",
        subtitle: "You're growing fast. Share your success to unlock $50 in OHC credits.",
        shareText: "I just hit my 100th order using OHC to run my business! 🚀 Check them out and get $50 off your first month:",
        reward: "$50 Credit"
      }),
    } as any);
  });

  it('renders milestone data correctly from mock/backend', async () => {
    render(<SuccessMilestoneWidget />);

    // Check that title and subtitle are loaded
    expect(await screen.findByText('100th Order Delivered! 🎉')).toBeDefined();
    expect(screen.getByText(/You're growing fast/i)).toBeDefined();
    expect(screen.getByText('$50 Credit')).toBeDefined();

    // Check that the tenant id is incorporated in the link
    const shareTextEl = screen.getByText(/"I just hit my 100th order.*test-tenant/i);
    expect(shareTextEl).toBeDefined();
  });

  it('copies share link to clipboard and updates button state', async () => {
    render(<SuccessMilestoneWidget />);

    // Wait for the component to load data and render the button
    await screen.findByText('100th Order Delivered! 🎉');

    // Find the button by looking for its text content using a custom matcher
    const copyButton = screen.getByText(/Copy & Share to Unlock/i).closest('button');
    expect(copyButton).toBeDefined();

    fireEvent.click(copyButton!);

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith(
      expect.stringContaining('https://ohc.app/onboarding?ref=test-tenant&source=milestone_share')
    );

    expect(await screen.findByText(/Copied to Clipboard!/i)).toBeDefined();
  });

  it('provides a valid twitter intent link', async () => {
    render(<SuccessMilestoneWidget />);
    await screen.findByText('100th Order Delivered! 🎉');

    const twitterLink = screen.getByRole('link', { name: /Share on X/i });
    expect(twitterLink.getAttribute('href')).toContain('https://twitter.com/intent/tweet?text=');
    expect(twitterLink.getAttribute('href')).toContain('test-tenant');
  });
});
