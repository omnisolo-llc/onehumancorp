import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import LeaveReviewPage from './page';

const mockUseSearchParams = vi.fn(() => new URLSearchParams('?order=123'));
vi.mock('next/navigation', () => ({
  useRouter: () => ({
    push: vi.fn(),
  }),
  useSearchParams: () => mockUseSearchParams(),
}));

describe('LeaveReviewPage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders the leave review form', () => {
    render(<LeaveReviewPage />);
    expect(screen.getByText('How was your experience?')).toBeDefined();
    expect(screen.getByText('Order #123')).toBeDefined();
    expect(screen.getByText('Submit Review')).toBeDefined();
    expect(screen.getByText('⚡ Powered by OHC')).toBeDefined();
  });

  it('shows viral widget after a 5-star review', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ referral_link: 'http://test.link/vip' })
    } as any);

    render(<LeaveReviewPage />);

    // Find all stars
    const stars = screen.getAllByText('★');
    expect(stars.length).toBe(5);

    // Click 5th star
    fireEvent.click(stars[4].parentElement!);

    // Submit
    const submitBtn = screen.getByText('Submit Review');
    fireEvent.click(submitBtn);

    // Verify success screen shows the viral widget
    await waitFor(() => {
      expect(screen.getByText('Thank you for your review!')).toBeDefined();
      expect(screen.getByText('Get 15% Off Your Next Order')).toBeDefined();
      expect(screen.getByDisplayValue('http://test.link/vip')).toBeDefined();
      expect(screen.getByText(/Powered by OHC/)).toBeDefined();
    });
  });

  it('does not show viral widget after a 3-star review', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ referral_link: 'http://test.link/vip' })
    } as any);

    render(<LeaveReviewPage />);

    // Click 3rd star
    const stars = screen.getAllByText('★');
    fireEvent.click(stars[2].parentElement!);

    // Submit
    const submitBtn = screen.getByText('Submit Review');
    fireEvent.click(submitBtn);

    // Verify success screen does NOT show the viral widget
    await waitFor(() => {
      expect(screen.getByText('Thank you for your review!')).toBeDefined();
      expect(screen.queryByText('Get 15% Off Your Next Order')).toBeNull();
    });
  });
});
