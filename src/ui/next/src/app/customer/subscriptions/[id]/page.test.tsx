import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import CustomerSubscriptionPortal from './page';

// Mock useParams
vi.mock('next/navigation', () => ({
  useParams: () => ({ id: 'sub_123' }),
}));

// Mock PoweredByOHC
vi.mock('../../../components/PoweredByOHC', () => ({
  PoweredByOHC: () => <div data-testid="powered-by-ohc" />,
}));

describe('CustomerSubscriptionPortal', () => {
  beforeEach(() => {
    vi.useFakeTimers({ shouldAdvanceTime: true });
  });

  afterEach(() => {
    vi.runOnlyPendingTimers();
    vi.useRealTimers();
  });

  it('renders loading state initially', async () => {
    const { unmount } = render(<CustomerSubscriptionPortal />);
    expect(screen.getByText('Loading your subscription...')).toBeDefined();

    // allow state to settle
    await act(async () => {
      vi.advanceTimersByTime(500);
    });
    unmount();
  });

  it('renders subscription details after loading', async () => {
    const { unmount } = render(<CustomerSubscriptionPortal />);

    await act(async () => {
      vi.advanceTimersByTime(500);
    });

    await waitFor(() => {
      expect(screen.queryByText('Loading your subscription...')).toBeNull();
    });

    expect(screen.getByText('Manage Subscription')).toBeDefined();
    expect(screen.getByText('Artisan Coffee Blend')).toBeDefined();
    expect(screen.getByText('Monthly')).toBeDefined();
    expect(screen.getByText('Active')).toBeDefined();
    expect(screen.getByText('$21.60')).toBeDefined();
    unmount();
  });

  it('handles skip next delivery action', async () => {
    const { unmount } = render(<CustomerSubscriptionPortal />);

    await act(async () => {
      vi.advanceTimersByTime(500);
    });

    await waitFor(() => {
      expect(screen.getByText('Manage Subscription')).toBeDefined();
    });

    const skipButton = screen.getByText('Skip Next Delivery');
    fireEvent.click(skipButton);

    await act(async () => {
      vi.advanceTimersByTime(1000);
    });

    await waitFor(() => {
      expect(screen.getByText('Your next delivery has been skipped.')).toBeDefined();
      expect(screen.getByText('2023-12-15')).toBeDefined();
    });
    unmount();
  });

  it('handles pause subscription action', async () => {
    const { unmount } = render(<CustomerSubscriptionPortal />);

    await act(async () => {
      vi.advanceTimersByTime(500);
    });

    await waitFor(() => {
      expect(screen.getByText('Manage Subscription')).toBeDefined();
    });

    const pauseButton = screen.getByText('Pause Subscription');
    fireEvent.click(pauseButton);

    await act(async () => {
      vi.advanceTimersByTime(1000);
    });

    await waitFor(() => {
      expect(screen.getByText('Your subscription has been paused.')).toBeDefined();
      expect(screen.getByText('Paused')).toBeDefined();
      expect(screen.getByText('Subscription Paused')).toBeDefined();
    });
    unmount();
  });

  it('handles cancel subscription action', async () => {
    const { unmount } = render(<CustomerSubscriptionPortal />);

    await act(async () => {
      vi.advanceTimersByTime(500);
    });

    await waitFor(() => {
      expect(screen.getByText('Manage Subscription')).toBeDefined();
    });

    const cancelButton = screen.getByText('Cancel Subscription');
    fireEvent.click(cancelButton);

    await act(async () => {
      vi.advanceTimersByTime(1000);
    });

    await waitFor(() => {
      expect(screen.getByText('Your subscription has been cancelled.')).toBeDefined();
      expect(screen.getByText('Cancelled')).toBeDefined();
      expect(screen.getByText('You have cancelled this subscription.')).toBeDefined();
      expect(screen.queryByText('Skip Next Delivery')).toBeNull();
    });
    unmount();
  });
});
