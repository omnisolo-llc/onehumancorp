import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralPostGeneratorPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(() => ({
    push: vi.fn(),
  })),
}));

describe('ViralPostGeneratorPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn(),
      },
    });

    const localStorageMock = {
      getItem: vi.fn((key) => {
        if (key === 'tenant') return 'test-tenant';
        return null;
      }),
      setItem: vi.fn(),
      clear: vi.fn()
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock,
      writable: true
    });

    // Mock window.open
    Object.defineProperty(window, 'open', {
        value: vi.fn(),
        writable: true
    });
    global.fetch = vi.fn().mockImplementation((url: string, options?: RequestInit) => {
      if (url === '/api/v1/growth/trial-extension/claim' && options?.method === 'POST') {
        return Promise.resolve({ ok: true, json: async () => ({}) });
      }
      return Promise.resolve({ ok: true, json: async () => ({ current_plan: 'free' }) });
    });
  });

  it('renders correctly', () => {
    render(<ViralPostGeneratorPage />);
    expect(screen.getByText('Promoter Agent Post Generator 🚀')).toBeDefined();
  });

  it('generates a post', () => {
    render(<ViralPostGeneratorPage />);

    const productNameInput = screen.getByPlaceholderText('e.g. Signature Coffee Blend');
    fireEvent.change(productNameInput, { target: { value: 'Test Product' } });

    const keyBenefitInput = screen.getByPlaceholderText('e.g. a bold start to your morning');
    fireEvent.change(keyBenefitInput, { target: { value: 'Testing benefits' } });

    const generateBtn = screen.getByRole('button', { name: 'Generate Post' });
    fireEvent.click(generateBtn);

    expect(screen.getByText(/Test Product/)).toBeDefined();
    expect(screen.getByText(/Testing benefits/)).toBeDefined();
    expect(screen.getAllByText(/Powered by OHC/).length).toBeGreaterThan(0);
  });

  it('copies to clipboard', () => {
    render(<ViralPostGeneratorPage />);

    const productNameInput = screen.getByPlaceholderText('e.g. Signature Coffee Blend');
    fireEvent.change(productNameInput, { target: { value: 'Test Product' } });

    const keyBenefitInput = screen.getByPlaceholderText('e.g. a bold start to your morning');
    fireEvent.change(keyBenefitInput, { target: { value: 'Testing benefits' } });

    const generateBtn = screen.getByRole('button', { name: 'Generate Post' });
    fireEvent.click(generateBtn);

    const copyBtn = screen.getByRole('button', { name: 'Copy to Clipboard' });
    fireEvent.click(copyBtn);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByText('Copied!')).toBeDefined();
  });

  it('shows paywall when toggling remove branding', () => {
    render(<ViralPostGeneratorPage />);
    const checkbox = screen.getByRole('checkbox');
    fireEvent.click(checkbox);

    expect(screen.getAllByText('Upgrade to Pro').length).toBeGreaterThan(0);
    expect(screen.getByRole('button', { name: 'Share on X to Unlock for Free' })).toBeDefined();
  });

  it('claims trial extension through the backend', async () => {
    render(<ViralPostGeneratorPage />);
    const checkbox = screen.getByRole('checkbox');
    fireEvent.click(checkbox);

    const shareBtn = screen.getByRole('button', { name: 'Share on X to Unlock for Free' });
    fireEvent.click(shareBtn);

    expect(window.open).toHaveBeenCalledWith(expect.stringContaining('twitter.com/intent/tweet'), '_blank');
    await waitFor(() => expect(global.fetch).toHaveBeenCalledWith('/api/v1/growth/trial-extension/claim', { method: 'POST' }));
    await waitFor(() => expect(screen.queryByText('Upgrade to Pro')).toBeNull());
    expect(window.localStorage.setItem).not.toHaveBeenCalled();
  });
});
