import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import ViralMysteryBoxPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

vi.mock('../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('ViralMysteryBoxPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    Object.assign(navigator, {
      clipboard: {
        writeText: vi.fn().mockResolvedValue(undefined),
      },
    });

    const localStorageMock = {
      getItem: vi.fn((key) => {
        if (key === 'business_display_name') return 'test-tenant';
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
    render(<ViralMysteryBoxPage />);
    expect(screen.getByText('Viral Mystery Box 🎁')).toBeDefined();
    expect(screen.getByText('Mystery Box Setup')).toBeDefined();
    expect(screen.getByText('Mobile Preview: Mystery Box')).toBeDefined();
    expect(screen.getByTestId('powered-by-ohc')).toBeDefined();
  });

  it('updates form inputs and preview correctly', () => {
    render(<ViralMysteryBoxPage />);

    const titleInput = screen.getByPlaceholderText('e.g. Secret Sneaker Box');
    fireEvent.change(titleInput, { target: { value: 'Awesome Box' } });

    const priceInput = screen.getByPlaceholderText('29.99');
    fireEvent.change(priceInput, { target: { value: '99.99' } });

    const valueInput = screen.getByPlaceholderText('100.00');
    fireEvent.change(valueInput, { target: { value: '250.00' } });

    const discountInput = screen.getByPlaceholderText('15');
    fireEvent.change(discountInput, { target: { value: '25' } });

    // Check if preview updates
    expect(screen.getByText('Awesome Box')).toBeDefined();
    expect(screen.getByText('$99.99')).toBeDefined();
    expect(screen.getByText('$250.00 Value!')).toBeDefined();
    expect(screen.getByText('25% OFF')).toBeDefined();
  });

  it('displays empty states when inputs are cleared', () => {
    render(<ViralMysteryBoxPage />);

    const titleInput = screen.getByPlaceholderText('e.g. Secret Sneaker Box');
    fireEvent.change(titleInput, { target: { value: '' } });

    const priceInput = screen.getByPlaceholderText('29.99');
    fireEvent.change(priceInput, { target: { value: '' } });

    expect(screen.getByText('Mystery Box')).toBeDefined();
    expect(screen.getByText('$0')).toBeDefined();
  });

  it('copies link to clipboard', () => {
    render(<ViralMysteryBoxPage />);

    const copyBtn = screen.getByRole('button', { name: 'Copy Link' });
    fireEvent.click(copyBtn);

    expect(navigator.clipboard.writeText).toHaveBeenCalled();
    expect(screen.getByRole('button', { name: 'Copied!' })).toBeDefined();
  });

  it('navigates back to dashboard', () => {
    render(<ViralMysteryBoxPage />);

    const backBtn = screen.getByRole('button', { name: /Back to Dashboard/i });
    fireEvent.click(backBtn);

    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });
});
