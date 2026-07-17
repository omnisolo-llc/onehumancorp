import { render, screen, waitFor } from '@testing-library/react';
import { WrappedWidget } from './WrappedWidget';
import React from 'react';
import { vi } from 'vitest';

describe('WrappedWidget', () => {
  beforeEach(() => {
    // Mock the fetch call
    global.fetch = vi.fn().mockResolvedValue({
      json: vi.fn().mockResolvedValue({
        year: 2024,
        title: "Your Year in Review 🎉",
        subtitle: "You crushed it this year! See your impact and share with your community.",
        stats: {
          totalSales: "$14,250",
          totalOrders: 342,
          newCustomers: 128,
          topProduct: "Custom Logo Design",
          aiHoursSaved: 42,
        },
        shareText: "I just reviewed my 2024 business stats on OHC and I'm blown away! I saved 42 hours using AI and served 128 new customers. Start growing your business on OHC:",
      }),
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders the wrapped widget with fetched data', async () => {
    render(<WrappedWidget />);

    // Wait for the data to load and component to render
    await waitFor(() => {
      expect(screen.getByTestId('wrapped-widget')).toBeTruthy();
    });

    expect(screen.getByText('Your Year in Review 🎉')).toBeTruthy();
    expect(screen.getByText('$14,250')).toBeTruthy();
    expect(screen.getByText('342')).toBeTruthy();
    expect(screen.getByText('128')).toBeTruthy();
  });
});
