import React from 'react';
import { render, screen, act } from '@testing-library/react';
import { describe, it, expect, vi, beforeAll, afterAll } from 'vitest';
import '@testing-library/jest-dom';
import { NetworkStatusIndicator } from './NetworkStatusIndicator';

// Mock SyncManager to avoid complex dependencies
vi.mock('../lib/sync/SyncManager', () => ({
  SyncManager: {
    getInstance: vi.fn(() => ({
      getQueueLength: vi.fn().mockResolvedValue(0),
    })),
  },
}));

// Mock WithTooltip since we just want to test NetworkStatusIndicator wrapper
vi.mock('./TooltipRegistry', () => ({
  WithTooltip: ({ children }: { children: React.ReactNode }) => <div data-testid="tooltip-mock">{children}</div>,
}));

describe('NetworkStatusIndicator', () => {
  let originalOnLine: boolean;

  beforeAll(() => {
    // Save original navigator.onLine value
    originalOnLine = navigator.onLine;
  });

  afterAll(() => {
    // Restore original navigator.onLine value
    Object.defineProperty(navigator, 'onLine', {
      value: originalOnLine,
      configurable: true,
    });
  });

  it('renders correctly when offline and syncQueueLength is 0', async () => {
    Object.defineProperty(navigator, 'onLine', {
      value: false,
      configurable: true,
    });

    await act(async () => {
      render(<NetworkStatusIndicator />);
    });

    // Check if the offline text is present
    expect(screen.getByText('Offline - Changes saved locally')).toBeInTheDocument();
  });

  it('applies the new Translucent Glass CSS classes', async () => {
    Object.defineProperty(navigator, 'onLine', {
      value: false,
      configurable: true,
    });

    await act(async () => {
      render(<NetworkStatusIndicator />);
    });

    // Verify the div contains the specific Translucent Glass classes
    const container = screen.getByText('Offline - Changes saved locally').closest('div');
    expect(container).toHaveClass('bg-white/65');
    expect(container).toHaveClass('backdrop-blur-[30px]');
    expect(container).toHaveClass('saturate-[210%]');
    expect(container).toHaveClass('border-white/40');
  });
});
