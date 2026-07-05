/** @vitest-environment jsdom */

import React from 'react';
import { render, screen, fireEvent, cleanup } from '@testing-library/react';
import { describe, it, expect, vi, afterEach } from 'vitest';
import { PayoutSummaryCard } from '../PayoutSummaryCard';

describe('PayoutSummaryCard', () => {
  afterEach(() => {
    cleanup();
  });

  it('renders correctly and formats currencies properly', () => {
    const mockOnViewDetails = vi.fn();
    render(
      <PayoutSummaryCard
        totalUsdEarned={15000} // $150.00
        totalEurEarned={5000}  // €50.00
        totalUsdPayout={14500} // $145.00
        onViewDetails={mockOnViewDetails}
      />
    );

    // Assert that the text is correctly rendered and formatted
    expect(screen.getByText('Your Payout Summary is ready.')).toBeDefined();
    expect(screen.getByText('Pending')).toBeDefined();
    expect(
      screen.getByText(
        'You earned $150.00 USD and €50.00 EUR this week. After conversion and fees, $145.00 USD will hit your Chase account tomorrow.'
      )
    ).toBeDefined();

    // Verify button exists
    const viewDetailsButton = screen.getByRole('button', { name: 'View Details' });
    expect(viewDetailsButton).toBeDefined();
  });

  it('calls onViewDetails when the button is clicked', () => {
    const mockOnViewDetails = vi.fn();
    render(
      <PayoutSummaryCard
        totalUsdEarned={15000}
        totalEurEarned={5000}
        totalUsdPayout={14500}
        onViewDetails={mockOnViewDetails}
      />
    );

    const viewDetailsButton = screen.getByRole('button', { name: 'View Details' });
    fireEvent.click(viewDetailsButton);

    expect(mockOnViewDetails).toHaveBeenCalledTimes(1);
  });
});
