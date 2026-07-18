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

  it('does not fabricate review or referral success after a 5-star review', async () => {
    global.fetch = vi.fn();
    render(<LeaveReviewPage />);

    // Find all stars
    const stars = screen.getAllByText('★');
    expect(stars.length).toBe(5);

    // Click 5th star
    fireEvent.click(stars[4].parentElement!);

    // Submit
    const submitBtn = screen.getByText('Submit Review');
    fireEvent.click(submitBtn);

    await waitFor(() => {
      expect(screen.getByRole('alert')).toHaveTextContent(/Review submission is unavailable/);
      expect(screen.queryByText('Thank you for your review!')).toBeNull();
      expect(screen.queryByText('Get 15% Off Your Next Order')).toBeNull();
    });
    expect(global.fetch).not.toHaveBeenCalled();
  });

  it('does not report a 3-star review as submitted without a review API', async () => {
    global.fetch = vi.fn();
    render(<LeaveReviewPage />);

    // Click 3rd star
    const stars = screen.getAllByText('★');
    fireEvent.click(stars[2].parentElement!);

    // Submit
    const submitBtn = screen.getByText('Submit Review');
    fireEvent.click(submitBtn);

    await waitFor(() => {
      expect(screen.getByRole('alert')).toHaveTextContent(/Review submission is unavailable/);
      expect(screen.queryByText('Thank you for your review!')).toBeNull();
      expect(screen.queryByText('Get 15% Off Your Next Order')).toBeNull();
    });
    expect(global.fetch).not.toHaveBeenCalled();
  });
});
