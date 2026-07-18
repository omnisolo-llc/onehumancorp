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
    global.fetch = vi.fn((url: string) => Promise.resolve(
      url.includes('department-tier-usage')
        ? { ok: true, json: () => Promise.resolve({ departments: [{ actions_used: 85, action_limit: 100 }] }) }
        : { ok: true, json: () => Promise.resolve({ referral_link: 'https://ohc.app/onboarding?ref=verified' }) },
    )) as any;
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
    expect(screen.getByText(/Generate Referral Link/)).toBeDefined();
  });

  it('generates link and copies it', async () => {
    render(<AIUsageLimitWidget />);

    await waitFor(() => {
      expect(screen.getAllByText(/85/).length).toBeGreaterThan(0);
    });

    const generateBtn = screen.getByText(/Generate Referral Link/);
    fireEvent.click(generateBtn);

    await waitFor(() => {
      expect(screen.getByText(/Copy Link/)).toBeDefined();
    }, { timeout: 1500 });

    const copyBtn = screen.getByText(/Copy Link/);
    fireEvent.click(copyBtn);

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith(
      'https://ohc.app/onboarding?ref=verified'
    );

    expect(screen.getByText(/Copied Link!/)).toBeDefined();

    expect(screen.getAllByText(/85/).length).toBeGreaterThan(0);
  });

  it('opens X share intent without changing recorded usage', async () => {
    render(<AIUsageLimitWidget />);

    await waitFor(() => {
      expect(screen.getAllByText(/85/).length).toBeGreaterThan(0);
    });

    const windowOpenSpy = vi.spyOn(window, 'open').mockImplementation(() => null);

    const generateBtn = screen.getByText(/Generate Referral Link/);
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

    expect(screen.getAllByText(/85/).length).toBeGreaterThan(0);
  });

  it('handles fetch failure gracefully', async () => {
    global.fetch = vi.fn(() => Promise.reject(new Error("API Down"))) as any;

    render(<AIUsageLimitWidget />);

    await waitFor(() => {
      expect(screen.getByText('Usage data is unavailable.')).toBeDefined();
    });
    expect(screen.queryByText(/\/ 100/)).toBeNull();
  });
});
