import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import ChaosReportPage from './page';

describe('ChaosReportPage', () => {
  it('renders the failure report correctly', () => {
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

    render(<ChaosReportPage />);
    expect(screen.getByText('System Reliability Report')).toBeInTheDocument();
    expect(screen.getByText('Latency Distribution')).toBeInTheDocument();
    expect(screen.getByText('Error Rate Over Time')).toBeInTheDocument();
  });
});
