import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { AIUsageLimitWidget } from './AIUsageLimitWidget';
import * as React from 'react';

// Mock clipboard
Object.assign(navigator, {
  clipboard: {
    writeText: vi.fn(),
  },
});

describe('AIUsageLimitWidget', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.defineProperty(window, 'localStorage', {
      value: {
        getItem: vi.fn(() => 'test-tenant'),
      },
      writable: true
    });

    // Mock fetch for the API call
    global.fetch = vi.fn(() =>
      Promise.resolve({
        json: () => Promise.resolve({ departments: [{ actions_used: 85, action_limit: 100 }] }),
      })
    ) as any;
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders correctly', async () => {
    render(<AIUsageLimitWidget />);

    expect(screen.getByText(/Approaching Free Tier Limit/i)).toBeDefined();
    await waitFor(() => {
      expect(screen.getAllByText(/85/).length).toBeGreaterThan(0);
    });
    expect(screen.getByText(/\/ 100/)).toBeDefined();
    expect(screen.getByText(/Upgrade to Pro \(Unlimited\)/)).toBeDefined();
    expect(screen.getByText(/Share on X to get \+50 Actions/)).toBeDefined();
  });

  it('generates link and copies it', async () => {
    render(<AIUsageLimitWidget />);

    await waitFor(() => {
      expect(screen.getAllByText(/85/).length).toBeGreaterThan(0);
    });

    const generateBtn = screen.getByText(/Share on X to get \+50 Actions/);
    fireEvent.click(generateBtn);

    expect(screen.getByText(/Generating.../)).toBeDefined();

    await waitFor(() => {
      expect(screen.getByText(/Copy Link/)).toBeDefined();
    }, { timeout: 1500 });

    const copyBtn = screen.getByText(/Copy Link/);
    fireEvent.click(copyBtn);

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith(
      expect.stringContaining('/onboarding?ref=test-tenant&source=ai_limit_paywall')
    );

    expect(screen.getByText(/Copied Link!/)).toBeDefined();

    await waitFor(() => {
        expect(screen.getAllByText(/35/).length).toBeGreaterThan(0);
    }, { timeout: 2000 });

    await waitFor(() => {
        expect(screen.queryByText(/Copied Link!/)).toBeNull();
        expect(screen.getByText(/Copy Link/)).toBeDefined();
    }, { timeout: 2500 });
  });

  it('opens X share intent and updates usage', async () => {
    render(<AIUsageLimitWidget />);

    await waitFor(() => {
      expect(screen.getAllByText(/85/).length).toBeGreaterThan(0);
    });

    const windowOpenSpy = vi.spyOn(window, 'open').mockImplementation(() => null);

    const generateBtn = screen.getByText(/Share on X to get \+50 Actions/);
    fireEvent.click(generateBtn);

    await waitFor(() => {
        expect(screen.getAllByText(/Share on X/).length).toBeGreaterThan(0);
    }, { timeout: 1500 });

    const shareBtns = screen.getAllByText(/Share on X/);
    const targetBtn = shareBtns.find(b => b.tagName === 'BUTTON');
    if (targetBtn) {
       fireEvent.click(targetBtn);
    }

    expect(windowOpenSpy).toHaveBeenCalledWith(
      expect.stringContaining('https://twitter.com/intent/tweet?text='),
      '_blank'
    );

    await waitFor(() => {
        expect(screen.getAllByText(/35/).length).toBeGreaterThan(0);
    }, { timeout: 2000 });
  });

  it('handles fetch failure gracefully', async () => {
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    global.fetch = vi.fn(() => Promise.reject(new Error("API Down"))) as any;

    render(<AIUsageLimitWidget />);

    // Should default to 0 used and 100 limit, and log an error
    await waitFor(() => {
      expect(consoleSpy).toHaveBeenCalledWith("Failed to fetch usage", expect.any(Error));
    });

    expect(screen.getByText(/Approaching Free Tier Limit/i)).toBeDefined();
    expect(screen.getAllByText(/0/).length).toBeGreaterThan(0);
  });
});
