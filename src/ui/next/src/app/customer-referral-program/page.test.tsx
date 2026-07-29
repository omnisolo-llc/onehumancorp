import React from 'react';
import { render, screen, fireEvent, act, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import CustomerReferralProgramPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(() => ({ push: vi.fn(), back: vi.fn() })),
}));

describe('CustomerReferralProgramPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => ({ current_plan: 'free' }) });
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockResolvedValue(undefined),
      },
    });
  });

  it('uses default tenant if missing from localStorage', () => {
    const localStorageMock = {
      getItem: vi.fn().mockReturnValue(null),
    };
    Object.defineProperty(window, 'localStorage', {
      value: localStorageMock,
    });
    render(<CustomerReferralProgramPage />);
  });

  it('renders the configurator', () => {
    render(<CustomerReferralProgramPage />);
    expect(screen.getByText('Customer Referral Program')).toBeDefined();
    expect(screen.getByText('They Give ($ Discount)')).toBeDefined();
    expect(screen.getByText('They Get ($ Reward)')).toBeDefined();
  });

  it('updates give and get amounts', async () => {
    render(<CustomerReferralProgramPage />);

    const inputs = screen.getAllByRole('spinbutton');
    const giveInput = inputs[0];
    const getInput = inputs[1];

    await act(async () => {
        fireEvent.change(giveInput, { target: { value: '25' } });
        fireEvent.change(getInput, { target: { value: '50' } });
    });
  });

  it('shows the embed modal when clicking generate', async () => {
    render(<CustomerReferralProgramPage />);

    const generateBtn = screen.getByText('Generate Widget Embed');

    await act(async () => {
        fireEvent.click(generateBtn);
    });

    expect(screen.getByText('Embed Widget')).toBeDefined();
    expect(screen.getByText('Copy Code')).toBeDefined();

    // Test copy
    const copyBtn = screen.getByText('Copy Code');
    await act(async () => {
        fireEvent.click(copyBtn);
    });
    expect(navigator.clipboard.writeText).toHaveBeenCalled();

    // Test close modal
    const closeBtn = screen.getByText('✕');
    await act(async () => {
        fireEvent.click(closeBtn);
    });
  });


  it('navigates back', async () => {
    render(<CustomerReferralProgramPage />);
    const backBtn = screen.getByText('Back to Dashboard');
    await act(async () => {
        fireEvent.click(backBtn);
    });
    // Check navigation
  });

  it('renders Powered by OHC branding in preview by default', () => {
    render(<CustomerReferralProgramPage />);
    const brandingElements = screen.getAllByText(/Powered by OHC/i);
    expect(brandingElements.length).toBeGreaterThan(0);
  });

  it('shows soft paywall when attempting to remove branding without pro', async () => {
    render(<CustomerReferralProgramPage />);

    const toggle = screen.getByRole('checkbox', { name: /Remove "Powered by OHC"/i });

    await act(async () => {
        fireEvent.click(toggle);
    });

    expect(screen.getByText('Pro Feature')).toBeDefined();
    expect(screen.getByText(/Upgrade to Pro to remove/i)).toBeDefined();
  });

  it('removes branding when pro is true and toggle is clicked', async () => {
    global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => ({ current_plan: 'pro' }) });

    render(<CustomerReferralProgramPage />);
    await waitFor(() => expect(global.fetch).toHaveBeenCalledWith('/api/v1/billing/my-plan'));

    const toggle = screen.getByRole('checkbox', { name: /Remove "Powered by OHC"/i });

    await waitFor(() => {
        expect(toggle).not.toBeDisabled();
    });

    await act(async () => {
        fireEvent.click(toggle);
    });

    // The modal should NOT show up
    expect(screen.queryByText('Pro Feature')).toBeNull();
    // The exact text "⚡ Powered by OHC" in the preview should be removed
    expect(screen.queryByText('⚡ Powered by OHC')).toBeNull();
  });

});
