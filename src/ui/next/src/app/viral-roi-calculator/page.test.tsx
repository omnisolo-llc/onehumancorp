import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import ViralRoiCalculatorPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('ViralRoiCalculatorPage', () => {
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
    render(<ViralRoiCalculatorPage />);
    expect(screen.getByText('Viral ROI Calculator 📈')).toBeDefined();
    expect(screen.getByText('Service or Product Name')).toBeDefined();
    expect(screen.getByText('Live Preview')).toBeDefined();
  });

  it('updates form inputs and iframe src', () => {
    render(<ViralRoiCalculatorPage />);

    const serviceInput = screen.getByPlaceholderText('e.g. SEO Optimization');
    fireEvent.change(serviceInput, { target: { value: 'Marketing Package' } });

    const currencyInput = screen.getByPlaceholderText('e.g. $');
    fireEvent.change(currencyInput, { target: { value: '£' } });

    // Check if iframe src updates
    const iframe = document.querySelector('iframe');
    expect(iframe?.getAttribute('src')).toContain('serviceName=Marketing%20Package');
    expect(iframe?.getAttribute('src')).toContain('currency=%C2%A3');
  });

  it('copies embed code to clipboard', async () => {
    render(<ViralRoiCalculatorPage />);

    const copyBtn = screen.getByRole('button', { name: 'Copy Embed Code' });
    fireEvent.click(copyBtn);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByRole('button', { name: 'Copied to Clipboard!' })).toBeDefined();
  });

  it('shows paywall when removing branding without pro', () => {
    render(<ViralRoiCalculatorPage />);

    const checkbox = screen.getByRole('checkbox');
    fireEvent.click(checkbox);

    expect(screen.getByText('Upgrade to Remove Branding')).toBeDefined();
  });

  it('navigates back to dashboard', () => {
    render(<ViralRoiCalculatorPage />);

    const backBtn = screen.getByRole('button', { name: /Back to Dashboard/i });
    fireEvent.click(backBtn);

    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });
});
