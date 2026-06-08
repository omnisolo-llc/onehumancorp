import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import SmartPricingPage from './page';

vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

describe('SmartPricingPage', () => {
  it('renders the smart pricing header', () => {
    render(<SmartPricingPage />);
    expect(screen.getByText('Smart Pricing')).toBeDefined();
    expect(screen.getByText(/Let AI automatically adjust your prices/i)).toBeDefined();
  });

  it('toggles smart pricing and shows configuration', () => {
    render(<SmartPricingPage />);

    // Initially hidden
    expect(screen.queryByText('Configuration')).toBeNull();

    // Toggle on
    const enableToggle = screen.getByTestId('enable-smart-pricing-toggle');
    fireEvent.click(enableToggle);

    // Configuration now visible
    expect(screen.getByText('Configuration')).toBeDefined();
    expect(screen.getByText('Auto-discount perishables 2 hours before closing')).toBeDefined();
  });

  it('updates price preview when adjusting bounds', () => {
    render(<SmartPricingPage />);

    // Toggle on to see config
    const enableToggle = screen.getByTestId('enable-smart-pricing-toggle');
    fireEvent.click(enableToggle);

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
});
