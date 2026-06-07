import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import UpgradeROIPage from './page';

const mockPush = vi.fn();
vi.mock('next/navigation', () => ({
  useRouter: () => ({ push: mockPush }),
}));

describe('UpgradeROIPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('renders initial state correctly', () => {
    render(<UpgradeROIPage />);

    expect(screen.getByText('Pro Plan ROI Calculator 📈')).toBeDefined();

    // Check initial values (50 orders, $40 AOV)
    const inputs = screen.getAllByRole('slider');
    expect(inputs).toHaveLength(2);
    expect((inputs[0] as HTMLInputElement).value).toBe('50');
    expect((inputs[1] as HTMLInputElement).value).toBe('40');

    // Initial math:
    // orders = 50, aov = 40 => current revenue = 2000
    // projected orders = 50 * 1.25 = 63 (Math.round)
    // projected AOV = 40 * 1.15 = 46
    // projected revenue = 63 * 46 = 2898
    // increase = 898
    expect(screen.getByText('$2,000')).toBeDefined();
    expect(screen.getByText('$2,898')).toBeDefined();
    expect(screen.getByText('+$898')).toBeDefined();
  });

  it('updates calculation when inputs change', () => {
    render(<UpgradeROIPage />);

    const inputs = screen.getAllByRole('slider');
    const ordersInput = inputs[0];
    const aovInput = inputs[1];

    // Change orders to 100
    fireEvent.change(ordersInput, { target: { value: '100' } });

    // orders = 100, aov = 40 => current revenue = 4000
    // projected orders = 100 * 1.25 = 125
    // projected AOV = 40 * 1.15 = 46
    // projected revenue = 125 * 46 = 5750
    // increase = 1750
    expect(screen.getByText('$4,000')).toBeDefined();
    expect(screen.getByText('$5,750')).toBeDefined();
    expect(screen.getByText('+$1,750')).toBeDefined();

    // Change AOV to 100
    fireEvent.change(aovInput, { target: { value: '100' } });

    // orders = 100, aov = 100 => current revenue = 10000
    // projected orders = 100 * 1.25 = 125
    // projected AOV = 100 * 1.15 = 115
    // projected revenue = 125 * 115 = 14375
    // increase = 4375
    expect(screen.getByText('$10,000')).toBeDefined();
    expect(screen.getByText('$14,375')).toBeDefined();
    expect(screen.getByText('+$4,375')).toBeDefined();
  });

  it('navigates to checkout when clicking upgrade CTA', () => {
    render(<UpgradeROIPage />);

    const upgradeButton = screen.getByText(/Upgrade to Pro/);
    fireEvent.click(upgradeButton);

    expect(mockPush).toHaveBeenCalledWith('/checkout?tier=Pro');
  });

  it('navigates to dashboard when clicking back button', () => {
    render(<UpgradeROIPage />);

    const backButton = screen.getByText('Back to Dashboard');
    fireEvent.click(backButton);

    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });
});
