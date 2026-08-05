import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import ViralBundleBuilderPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('ViralBundleBuilderPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockResolvedValue(undefined),
      },
    });

    const localStorageMock = {
      getItem: vi.fn((key) => {
        if (key === 'tenant_id' || key === 'tenant') return 'test-tenant';
        if (key === 'has_pro') return 'false';
        return null;
      }),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock,
      writable: true
    });

    global.fetch = vi.fn();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders correctly', () => {
    render(<ViralBundleBuilderPage />);
    expect(screen.getByText('Viral Bundle Builder 📦')).toBeDefined();
    expect(screen.getByText('Bundle Settings')).toBeDefined();
    expect(screen.getByText('Build Your Dream Bundle')).toBeDefined();
  });

  it('updates form inputs and preview', () => {
    render(<ViralBundleBuilderPage />);

    const headlineInput = screen.getByDisplayValue('Build Your Dream Bundle');
    fireEvent.change(headlineInput, { target: { value: 'Summer Bundle' } });

    const discountInput = screen.getByDisplayValue('15%');
    fireEvent.change(discountInput, { target: { value: '20% Off' } });

    const sharesInput = screen.getByDisplayValue('3');
    fireEvent.change(sharesInput, { target: { value: '5' } });

    // Check if preview updates
    expect(screen.getAllByText('Summer Bundle').length).toBeGreaterThan(0);
    expect(screen.getAllByText('20% Off').length).toBeGreaterThan(0);
    expect(screen.getByText('1 / 5')).toBeDefined();
  });

  it('copies link to clipboard', async () => {
    render(<ViralBundleBuilderPage />);

    const copyBtn = screen.getByRole('button', { name: 'Copy Embed Code' });
    fireEvent.click(copyBtn);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByRole('button', { name: 'Copied to Clipboard!' })).toBeDefined();
  });

  it('shows paywall when removing branding without pro', () => {
    render(<ViralBundleBuilderPage />);

    const checkbox = screen.getByLabelText(/Remove "Powered by OHC" Badge/i);
    fireEvent.click(checkbox);

    expect(screen.getByText('Upgrade to Remove Branding')).toBeDefined();
  });

  it('can claim trial extension from paywall', async () => {
    (global.fetch as any).mockResolvedValueOnce({ ok: true });

    render(<ViralBundleBuilderPage />);

    // Click checkbox to show paywall
    const checkbox = screen.getByLabelText(/Remove "Powered by OHC" Badge/i);
    fireEvent.click(checkbox);

    // Find and click the claim trial button
    const claimBtn = screen.getByText('Share on X for 7-Day Pro Trial');
    fireEvent.click(claimBtn);

    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalledWith('/api/v1/growth/trial-extension/claim', { method: 'POST' });
    });

    // Paywall should be closed
    expect(screen.queryByText('Upgrade to Remove Branding')).toBeNull();
  });

  it('navigates back to dashboard', () => {
    render(<ViralBundleBuilderPage />);

    const backBtn = screen.getByRole('button', { name: /Back to Dashboard/i });
    fireEvent.click(backBtn);

    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });
});
