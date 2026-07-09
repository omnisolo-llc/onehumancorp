import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import MysteryDiscountGeneratorPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(() => ({
    push: vi.fn(),
  })),
}));

describe('MysteryDiscountGeneratorPage', () => {
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

  it('renders correctly', () => {
    render(<MysteryDiscountGeneratorPage />);
    expect(screen.getByText('Viral Mystery Discount Generator 🎁')).toBeDefined();
    expect(screen.getByText('Widget Configuration')).toBeDefined();
    expect(screen.getByText('Live Preview')).toBeDefined();
  });

  it('updates configuration and embed code', () => {
    render(<MysteryDiscountGeneratorPage />);

    const titleInput = screen.getByLabelText('Widget Title');
    fireEvent.change(titleInput, { target: { value: 'Holiday Mystery' } });

    const descInput = screen.getByLabelText('Description');
    fireEvent.change(descInput, { target: { value: 'Get a special holiday discount.' } });

    // Check embed code updates
    const pre = screen.getByText(/Holiday%20Mystery/);
    expect(pre).toBeDefined();
    expect(pre.textContent).toContain('Holiday%20Mystery');
    expect(pre.textContent).toContain('Get%20a%20special%20holiday%20discount.');
  });

  it('copies to clipboard', () => {
    render(<MysteryDiscountGeneratorPage />);

    const copyBtn = screen.getByRole('button', { name: 'Copy Embed Code' });
    fireEvent.click(copyBtn);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByText('Copied to Clipboard!')).toBeDefined();
  });

  it('shows soft paywall when toggling remove branding', () => {
    render(<MysteryDiscountGeneratorPage />);

    // There might be multiple checkboxes, get the one for removing branding
    const removeBrandingCheckbox = screen.getByLabelText(/Remove "Powered by OHC" branding/);
    fireEvent.click(removeBrandingCheckbox);

    // Paywall should appear
    expect(screen.getAllByText('Upgrade to Pro').length).toBeGreaterThan(0);
    expect(screen.getByText(/Make the Mystery Box 100% yours/)).toBeDefined();
  });
});
