import { render, screen, act, waitFor } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import ChaosReportPage from './page';

describe('ChaosReportPage', () => {
  it('renders the failure report correctly', async () => {
    Object.defineProperty(window, 'matchMedia', {
      writable: true,
      value: vi.fn().mockImplementation(query => ({
        matches: false,
        media: query,
        onchange: null,
        addListener: vi.fn(),
        removeListener: vi.fn(),
        addEventListener: vi.fn(),
        removeEventListener: vi.fn(),
        dispatchEvent: vi.fn(),
      })),
    });

    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ latencyHistograms: [10, 20], errorRate: [0.1, 0.2] }),
      })
    ) as any;

    await act(async () => {
      render(<ChaosReportPage />);
    });

    await waitFor(() => {
      expect(screen.getByText('System Reliability Report')).toBeInTheDocument();
      expect(screen.getByText('Latency Distribution')).toBeInTheDocument();
      expect(screen.getByText('Error Rate Over Time')).toBeInTheDocument();
    });
  });
});
