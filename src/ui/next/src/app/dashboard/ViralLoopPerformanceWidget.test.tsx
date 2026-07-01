import { render, screen, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { ViralLoopPerformanceWidget } from './ViralLoopPerformanceWidget';
import React from 'react';

// Mock Next.js Link component
vi.mock('next/link', () => ({
  default: ({ children, href }: { children: React.ReactNode, href: string }) => {
    return React.createElement('a', { href }, children);
  }
}));

describe('ViralLoopPerformanceWidget', () => {
  beforeEach(() => {
    vi.spyOn(console, 'error').mockImplementation(() => {});
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders heading and default values initially', () => {
    global.fetch = vi.fn().mockImplementation(() => new Promise(() => {})); // pending promise

    render(React.createElement(ViralLoopPerformanceWidget));

    expect(screen.getByText('Viral Loop Performance')).toBeDefined();

    expect(screen.getByText('Invites Sent')).toBeDefined();
    expect(screen.getByText('Active Referrals')).toBeDefined();
    expect(screen.getByText('Revenue from Referrals')).toBeDefined();
    expect(screen.getByText('Pending Rewards')).toBeDefined();

    // Default 0 values
    const zeros = screen.getAllByText('0');
    expect(zeros.length).toBeGreaterThanOrEqual(2);

    const formattedZeros = screen.getAllByText('$0.00');
    expect(formattedZeros.length).toBeGreaterThanOrEqual(2);
  });

  it('fetches and displays metrics successfully', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        total_invites: 42,
        metrics: {
          active_referrals: 15,
          revenue: 1250.50,
          pending_rewards: 150.00
        }
      })
    });

    render(React.createElement(ViralLoopPerformanceWidget));

    await waitFor(() => {
      expect(screen.getByText('42')).toBeDefined();
      expect(screen.getByText('15')).toBeDefined();
      expect(screen.getByText('$1250.50')).toBeDefined();
      expect(screen.getByText('$150.00')).toBeDefined();
    });
  });

  it('handles fetch failure gracefully and defaults to 0', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 500
    });

    render(React.createElement(ViralLoopPerformanceWidget));

    await waitFor(() => {
      const zeros = screen.getAllByText('0');
      expect(zeros.length).toBeGreaterThanOrEqual(2);

      const formattedZeros = screen.getAllByText('$0.00');
      expect(formattedZeros.length).toBeGreaterThanOrEqual(2);
    });
  });

  it('handles network error gracefully and defaults to 0', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error('Network error'));

    render(React.createElement(ViralLoopPerformanceWidget));

    await waitFor(() => {
      const zeros = screen.getAllByText('0');
      expect(zeros.length).toBeGreaterThanOrEqual(2);

      const formattedZeros = screen.getAllByText('$0.00');
      expect(formattedZeros.length).toBeGreaterThanOrEqual(2);
    });
  });
});
