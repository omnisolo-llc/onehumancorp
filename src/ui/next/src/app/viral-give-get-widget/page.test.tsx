import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralGiveGetWidgetPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

describe('ViralGiveGetWidgetPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockResolvedValue(undefined),
      },
    });

    // Mock fetch for generating referral link
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ referral_link: 'https://ohc.app/give-get/join?ref=test-ref-123' }),
    });

    const localStorageMock = {
      getItem: vi.fn((key) => {
        if (key === 'tenant_id' || key === 'tenant') return 'test-tenant';
        return null;
      }),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock,
      writable: true
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders correctly', () => {
    render(<ViralGiveGetWidgetPage />);
    expect(screen.getByText('Viral Give-Get Generator 🎁')).toBeDefined();
    expect(screen.getByText('Give (Friend\'s Reward)')).toBeDefined();
    expect(screen.getByText('Get (Your Reward)')).toBeDefined();
  });

  it('updates give and get values', () => {
    render(<ViralGiveGetWidgetPage />);

    const giveInput = screen.getByLabelText(/Give/);
    fireEvent.change(giveInput, { target: { value: '30% Off' } });

    const getInput = screen.getByLabelText(/Get/);
    fireEvent.change(getInput, { target: { value: '$20 Credit' } });

    // Check if the display updates
    const displays = screen.getAllByText(/30% Off/);
    expect(displays.length).toBeGreaterThan(0);

    const getDisplays = screen.getAllByText(/\$20 Credit/);
    expect(getDisplays.length).toBeGreaterThan(0);
  });

  it('generates a referral link', async () => {
    render(<ViralGiveGetWidgetPage />);

    const generateBtn = screen.getByRole('button', { name: /Generate Referral Link/i });
    fireEvent.click(generateBtn);

    expect(global.fetch).toHaveBeenCalledWith('/api/v1/growth/referrals/generate', expect.any(Object));

    await waitFor(() => {
      expect(screen.getByText('Link Generated Successfully!')).toBeDefined();
    }, { timeout: 1500 }); // Wait for timeouts in handleGenerate

    expect(screen.getByDisplayValue(/test-ref-123/)).toBeDefined();
  });

  it('copies link to clipboard', async () => {
    render(<ViralGiveGetWidgetPage />);

    // Generate the link first
    const generateBtn = screen.getByRole('button', { name: /Generate Referral Link/i });
    fireEvent.click(generateBtn);

    await waitFor(() => {
      expect(screen.getByText('Link Generated Successfully!')).toBeDefined();
    }, { timeout: 1500 });

    const copyBtn = screen.getByRole('button', { name: 'Copy' });
    fireEvent.click(copyBtn);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByRole('button', { name: 'Copied!' })).toBeDefined();
  });

  it('navigates back to dashboard', () => {
    render(<ViralGiveGetWidgetPage />);

    const backBtn = screen.getByRole('button', { name: /Back to Dashboard/i });
    fireEvent.click(backBtn);

    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });

  it('renders Powered by OHC branding by default', () => {
    render(<ViralGiveGetWidgetPage />);
    const elements = screen.getAllByText(/Powered by OHC/i);
    expect(elements.length).toBeGreaterThan(0);
  });

  it('shows soft paywall when trying to remove branding without pro', () => {
    render(<ViralGiveGetWidgetPage />);
    const checkbox = screen.getByRole('checkbox');
    fireEvent.click(checkbox);

    const elements = screen.getAllByText('Upgrade to Pro');
    expect(elements.length).toBeGreaterThan(0);
  });
});
