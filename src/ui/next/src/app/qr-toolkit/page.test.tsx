import { render, screen, fireEvent } from '@testing-library/react';
import QRToolkitPage from './page';
import { expect, test, describe, vi, beforeEach } from 'vitest';

// Mock window.print
window.print = vi.fn();

// Mock window.open
window.open = vi.fn();

// Mock localStorage
const localStorageMock = (() => {
  let store: Record<string, string> = {};
  return {
    getItem: (key: string) => store[key] || null,
    setItem: (key: string, value: string) => {
      store[key] = value.toString();
    },
    clear: () => {
      store = {};
    }
  };
})();
Object.defineProperty(window, 'localStorage', { value: localStorageMock });

// Mock navigator.clipboard
Object.defineProperty(navigator, 'clipboard', {
  value: {
    writeText: vi.fn().mockImplementation(() => Promise.resolve()),
  },
});

// Mock window.alert
window.alert = vi.fn();

describe('QRToolkitPage', () => {
  beforeEach(() => {
    localStorageMock.clear();
    vi.clearAllMocks();
  });

  test('renders the QR Toolkit page', () => {
    render(<QRToolkitPage />);
    expect(screen.getByText(/QR Code Toolkit/i)).toBeDefined();
    expect(screen.getByText(/Configuration/i)).toBeDefined();
    expect(screen.getByText(/Live Preview/i)).toBeDefined();
  });

  test('switches target action', () => {
    render(<QRToolkitPage />);
    const reviewBtn = screen.getByText(/Leave a Review/i);
    fireEvent.click(reviewBtn);

    // Check if the button is highlighted (has specific classes)
    const buttonParent = reviewBtn.closest('button');
    expect(buttonParent?.className).toContain('border-amber-500');
  });

  test('shows paywall when clicking premium features without pro', () => {
    render(<QRToolkitPage />);
    const frameBtn = screen.getByText(/Branded Frame/i);
    fireEvent.click(frameBtn);

    expect(screen.getByText(/Unlock Branded QR Tools/i)).toBeDefined();
  });

  test('triggers print when clicking print button', () => {
    render(<QRToolkitPage />);
    const printBtn = screen.getByText(/Print QR Code/i);
    fireEvent.click(printBtn);

    expect(window.print).toHaveBeenCalled();
  });

  test('copies link to clipboard', () => {
    // Set a tenant ID in localStorage for predictable URL
    localStorageMock.setItem('ohc_tenant_id', 'maya-cakes');

    render(<QRToolkitPage />);
    const copyBtn = screen.getByText(/Copy Target Link/i);
    fireEvent.click(copyBtn);

    expect(navigator.clipboard.writeText).toHaveBeenCalledWith('https://ohc.app/s/maya-cakes');
    expect(window.alert).toHaveBeenCalledWith('Link copied to clipboard!');
  });
});
