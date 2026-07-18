import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import SmartPricingPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: mockPush,
  }),
}));

import { waitFor } from '@testing-library/react';
import { beforeEach, afterEach } from 'vitest';

describe('SmartPricingPage', () => {
  beforeEach(() => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
    vi.spyOn(console, 'error').mockImplementation(() => {});
    localStorage.clear();

    // Mock global fetch to return a resolved promise so `.catch` works safely
    global.fetch = vi.fn(() => Promise.resolve({
      ok: true,
      json: () => Promise.resolve({})
    })) as any;
  });

  afterEach(() => {
    if (vi.isFakeTimers && vi.isFakeTimers()) { vi.runOnlyPendingTimers(); }
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  it('renders the smart pricing header', () => {
    render(<SmartPricingPage />);
    expect(screen.getByText('Smart Pricing')).toBeDefined();
    expect(screen.getByText(/Preview smart-pricing configuration/i)).toBeDefined();
  });

  it('toggles smart pricing and shows configuration', async () => {
    render(<SmartPricingPage />);

    // Initially hidden
    expect(screen.queryByText('Configuration')).toBeNull();

    // Toggle on
    const enableToggle = screen.getByTestId('enable-smart-pricing-toggle');
    fireEvent.click(enableToggle);
    vi.advanceTimersByTime(350);

    // Configuration now visible
    await waitFor(() => {
        expect(screen.getByText('Configuration')).toBeDefined();
        expect(screen.getByText('Auto-discount perishables 2 hours before closing')).toBeDefined();
    });
  });

  it('toggles discount perishables and surge pricing', async () => {
    render(<SmartPricingPage />);

    const enableToggle = screen.getByTestId('enable-smart-pricing-toggle');
    fireEvent.click(enableToggle);
    vi.advanceTimersByTime(350);

    await waitFor(() => {
        expect(screen.getByTestId('discount-perishables-toggle')).toBeDefined();
    });

    const perishablesToggle = screen.getByTestId('discount-perishables-toggle');
    expect(perishablesToggle.className).toContain('bg-gray-300'); // initially off
    fireEvent.click(perishablesToggle);
    vi.advanceTimersByTime(350);

    await waitFor(() => {
        expect(perishablesToggle.className).toContain('bg-[#0066FF]'); // toggled on
    });

    const surgePricingToggle = screen.getByTestId('surge-pricing-toggle');
    expect(surgePricingToggle.className).toContain('bg-gray-300'); // initially off
    fireEvent.click(surgePricingToggle);
    vi.advanceTimersByTime(350);

    await waitFor(() => {
        expect(surgePricingToggle.className).toContain('bg-[#0066FF]'); // toggled on
    });
  });

  it('navigates to dashboard when clicking "Back to Dashboard"', () => {
    render(<SmartPricingPage />);
    const backButton = screen.getByText('Back to Dashboard');
    fireEvent.click(backButton);
    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });

  it('initializes from and persists to localStorage', async () => {
    localStorage.setItem('smartPricingEnabled', 'true');
    localStorage.setItem('smartPricingPerishables', 'true');

    render(<SmartPricingPage />);

    await waitFor(() => {
        expect(screen.getByText('Configuration')).toBeDefined();
    });

    const perishablesToggle = screen.getByTestId('discount-perishables-toggle');
    expect(perishablesToggle.className).toContain('bg-[#0066FF]'); // toggled on from local storage

    // Toggle surge pricing to true
    const surgePricingToggle = screen.getByTestId('surge-pricing-toggle');
    fireEvent.click(surgePricingToggle);
    vi.advanceTimersByTime(350);

    await waitFor(() => {
        expect(localStorage.getItem('smartPricingSurge')).toBe('true');
    });
  });
});
