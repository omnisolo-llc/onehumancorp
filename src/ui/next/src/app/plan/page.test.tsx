import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import MyPlanPage from './page';
import { useRouter } from 'next/navigation';

vi.mock('next/navigation', () => ({
  useRouter: vi.fn(),
}));

describe('MyPlanPage', () => {
  const mockPush = vi.fn();
  const mockFetch = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    (useRouter as any).mockReturnValue({ push: mockPush });
    global.fetch = mockFetch;
    // Suppress console.error in tests
    vi.spyOn(console, 'error').mockImplementation(() => {});
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders loading state initially', () => {
    mockFetch.mockImplementation(() => new Promise(() => {})); // pending promise
    render(<MyPlanPage />);
    expect(screen.getByText('Loading...')).toBeDefined();
  });

  it('renders plan data after fetching', async () => {
    const mockData = {
      current_plan: 'Pro',
      next_bill_estimated: 7900,
      ai_actions_used: 50,
      ai_actions_limit: 1000,
      storage_used_bytes: 1024 * 1024 * 500, // 500 MB
      storage_limit_bytes: 1024 * 1024 * 1024 * 5, // 5 GB
    };

    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => mockData,
    });

    render(<MyPlanPage />);

    await waitFor(() => {
      expect(screen.queryByText('Loading...')).toBeNull();
    });

    expect(screen.getByText('Pro')).toBeDefined();
    expect(screen.getByText('$79.00')).toBeDefined();
    expect(screen.getByText('50 / 1000')).toBeDefined();
    expect(screen.getByText('500 MB / 5 GB')).toBeDefined();
  });

  it('navigates to dashboard when clicking "Back to Dashboard"', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ current_plan: 'Free', next_bill_estimated: 0, ai_actions_used: 0, storage_used_bytes: 0 }),
    });

    render(<MyPlanPage />);
    await waitFor(() => expect(screen.queryByText('Loading...')).toBeNull());

    fireEvent.click(screen.getByText('Back to Dashboard'));
    expect(mockPush).toHaveBeenCalledWith('/dashboard');
  });

  it('navigates to pricing when clicking "Upgrade Plan"', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ current_plan: 'Free', next_bill_estimated: 0, ai_actions_used: 0, storage_used_bytes: 0 }),
    });

    render(<MyPlanPage />);
    await waitFor(() => expect(screen.queryByText('Loading...')).toBeNull());

    const upgradeButtons = screen.getAllByText(/Upgrade Plan/i);
    fireEvent.click(upgradeButtons[0]);
    expect(mockPush).toHaveBeenCalledWith('/pricing');
  });

  it('renders soft limit warning when limits are reached', async () => {
    const mockData = {
      current_plan: 'Free',
      next_bill_estimated: 0,
      ai_actions_used: 100,
      ai_actions_limit: 100,
      storage_used_bytes: 0,
      storage_limit_bytes: 0,
    };
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => mockData,
    });

    render(<MyPlanPage />);

    await waitFor(() => {
      expect(screen.getByText(/You've reached your free action limit/)).toBeDefined();
    });
  });

  it('navigates to cost-dashboard when clicking "View Cost Details"', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ current_plan: 'Free', next_bill_estimated: 0, ai_actions_used: 0, storage_used_bytes: 0 }),
    });

    render(<MyPlanPage />);
    await waitFor(() => expect(screen.queryByText('Loading...')).toBeNull());

    fireEvent.click(screen.getByText('View Cost Details'));
    expect(mockPush).toHaveBeenCalledWith('/cost-dashboard');
  });

  it('displays action message when clicking Download Invoice', async () => {
    mockFetch.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ current_plan: 'Free', next_bill_estimated: 0, ai_actions_used: 0, storage_used_bytes: 0 }),
    });

    render(<MyPlanPage />);

    await waitFor(() => expect(screen.queryByText('Loading...')).toBeNull());

    const button = screen.getByText('Download Invoice');
    fireEvent.click(button);

    await waitFor(() => {
        expect(screen.getByText('Invoice download is ready for your current billing period.')).toBeDefined();
    });
  });
});
