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
  });

  afterEach(() => {
    vi.runOnlyPendingTimers();
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  it('renders the smart pricing header', () => {
    render(<SmartPricingPage />);
    expect(screen.getByText('Smart Pricing')).toBeDefined();
    expect(screen.getByText(/Let AI automatically adjust your prices/i)).toBeDefined();
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

  it('updates price preview when adjusting bounds', async () => {
    render(<SmartPricingPage />);

    // Toggle on to see config
    const enableToggle = screen.getByTestId('enable-smart-pricing-toggle');
    fireEvent.click(enableToggle);
    vi.advanceTimersByTime(350);

    await waitFor(() => {
        expect(screen.getByTestId('preview-min-price')).toBeDefined();
    });

    // Initial 20% bounds for $10
    expect(screen.getByTestId('preview-min-price').textContent).toBe('$8.00');
    expect(screen.getByTestId('preview-max-price').textContent).toBe('$12.00');

    // Change to 50%
    const slider = screen.getByTestId('price-bounds-slider');
    fireEvent.change(slider, { target: { value: '50' } });

    // Should now be $5 to $15
    expect(screen.getByTestId('preview-min-price').textContent).toBe('$5.00');
    expect(screen.getByTestId('preview-max-price').textContent).toBe('$15.00');
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
        expect(perishablesToggle.className).toContain('bg-blue-500'); // toggled on
    });

    const surgePricingToggle = screen.getByTestId('surge-pricing-toggle');
    expect(surgePricingToggle.className).toContain('bg-gray-300'); // initially off
    fireEvent.click(surgePricingToggle);
    vi.advanceTimersByTime(350);

    await waitFor(() => {
        expect(surgePricingToggle.className).toContain('bg-blue-500'); // toggled on
    });
  });

  it('navigates to dashboard when clicking "Back to Dashboard"', () => {
    render(<SmartPricingPage />);
    const backButton = screen.getByText('Back to Dashboard');
    fireEvent.click(backButton);
    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });
});
