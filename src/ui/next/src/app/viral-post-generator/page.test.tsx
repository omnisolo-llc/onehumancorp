import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
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
        if (key === 'has_pro') return 'false';
        if (key === 'ohc_post_gen_shared') return 'false';
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

    global.fetch = vi.fn() as any;
  });

  it('renders correctly', () => {
    render(<ViralPostGeneratorPage />);
    expect(screen.getByText('Promoter Agent Post Generator 🚀')).toBeDefined();
  });

  it('generates a post', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        variants: [
          { platform: "twitter", content: "Check out Test Product! Testing benefits." }
        ]
      })
    });

    render(<ViralPostGeneratorPage />);

    const productNameInput = screen.getByPlaceholderText('e.g. Signature Coffee Blend');
    fireEvent.change(productNameInput, { target: { value: 'Test Product' } });

    const keyBenefitInput = screen.getByPlaceholderText('e.g. a bold start to your morning');
    fireEvent.change(keyBenefitInput, { target: { value: 'Testing benefits' } });

    const generateBtn = screen.getByRole('button', { name: 'Generate Post' });
    fireEvent.click(generateBtn);

    expect(screen.getByText(/Generating\.\.\./)).toBeDefined();

    await screen.findByText(/Check out Test Product!/);
    expect(screen.getAllByText(/Powered by OHC/).length).toBeGreaterThan(0);
  });


  it('copies to clipboard', async () => {
    (global.fetch as any).mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        variants: [
          { platform: "twitter", content: "Check out Test Product! Testing benefits." }
        ]
      })
    });

    render(<ViralPostGeneratorPage />);

    const productNameInput = screen.getByPlaceholderText('e.g. Signature Coffee Blend');
    fireEvent.change(productNameInput, { target: { value: 'Test Product' } });

    const keyBenefitInput = screen.getByPlaceholderText('e.g. a bold start to your morning');
    fireEvent.change(keyBenefitInput, { target: { value: 'Testing benefits' } });

    const generateBtn = screen.getByRole('button', { name: 'Generate Post' });
    fireEvent.click(generateBtn);

    await screen.findByText(/Check out Test Product!/);

    const copyBtn = screen.getByRole('button', { name: 'Copy twitter Post' });
    fireEvent.click(copyBtn);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
  });


  it('shows paywall when toggling remove branding', () => {
    render(<ViralPostGeneratorPage />);
    const checkbox = screen.getByRole('checkbox');
    fireEvent.click(checkbox);

    expect(screen.getAllByText('Upgrade to Pro').length).toBeGreaterThan(0);
    expect(screen.getByRole('button', { name: 'Share on X to Unlock for Free' })).toBeDefined();
  });

  it('claims trial extension', () => {
    render(<ViralPostGeneratorPage />);
    const checkbox = screen.getByRole('checkbox');
    fireEvent.click(checkbox);

    const shareBtn = screen.getByRole('button', { name: 'Share on X to Unlock for Free' });
    fireEvent.click(shareBtn);

    expect(window.open).toHaveBeenCalledWith(expect.stringContaining('twitter.com/intent/tweet'), '_blank');
    expect(window.localStorage.setItem).toHaveBeenCalledWith('ohc_post_gen_shared', 'true');
    expect(screen.queryByText('Upgrade to Pro')).toBeNull(); // Paywall closed
  });
});
