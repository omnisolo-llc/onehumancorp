import React from 'react';
import { render, screen } from '@testing-library/react';
import { expect, test, vi } from 'vitest';
import BookingWidgetBuilder from './page';

// Mocks
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
}));

test('BookingWidgetBuilder renders successfully with deposit and travel time inputs', () => {
  render(<BookingWidgetBuilder />);
  expect(screen.getByText('Booking Widget')).toBeInTheDocument();
  expect(screen.getByText('Deposit Amount ($)')).toBeInTheDocument();
  expect(screen.getByText('Need Travel Time? (AI Automated Buffer)')).toBeInTheDocument();
});
