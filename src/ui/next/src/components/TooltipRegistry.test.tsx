import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, fireEvent, act } from '@testing-library/react';
import { TooltipProvider, WithTooltip, useTooltip } from './TooltipRegistry';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

const mockTooltipFetch = vi.fn().mockImplementation((url) => {
    if (url === '/api/tooltips' || url.toString().includes('/api/tooltips')) {
        return Promise.resolve({ ok: true, json: async () => ({ "test-id": "Fetched tooltip text" }) });
    }
    return Promise.resolve({ ok: true, json: async () => ({}) });
});

describe('TooltipRegistry', () => {
  beforeEach(() => {
    mockTooltipFetch.mockClear();
    global.fetch = mockTooltipFetch as any;
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.runOnlyPendingTimers();
    vi.useRealTimers();
  });

  const flushPromises = async () => {
    await act(async () => {
      await Promise.resolve();
      await Promise.resolve();
    });
  };

  it('renders default text on hover', async () => {
    await act(async () => {
      render(<TooltipProvider><WithTooltip id="test-id" defaultText="Default Tooltip"><button>Hover me</button></WithTooltip></TooltipProvider>);
    });

    await flushPromises();

    const button = screen.getByText('Hover me');

    await act(async () => {
      vi.advanceTimersByTime(10);
    });

    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 10, left: 10, bottom: 30, right: 110, x: 10, y: 10, toJSON: () => {}
    }));

    await act(async () => {
      fireEvent.mouseEnter(button.parentElement!);
    });

    await act(async () => {
      vi.advanceTimersByTime(10);
    });

    expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();

    await act(async () => {
      fireEvent.mouseLeave(button.parentElement!);
    });

    await act(async () => {
      vi.advanceTimersByTime(10);
    });

    expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();
  });

  it('handles touch events (long press) for mobile', async () => {
    await act(async () => {
      render(<TooltipProvider><WithTooltip id="test-id" defaultText="Default Tooltip"><button>Touch me</button></WithTooltip></TooltipProvider>);
    });

    await flushPromises();

    const button = screen.getByText('Touch me');

    await act(async () => {
      vi.advanceTimersByTime(10);
    });

    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 10, left: 10, bottom: 30, right: 110, x: 10, y: 10, toJSON: () => {}
    }));

    await act(async () => {
      fireEvent.touchStart(button.parentElement!);
    });

    await act(async () => {
      vi.advanceTimersByTime(500);
      await flushPromises();
    });

    expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();

    await act(async () => {
      fireEvent.touchEnd(button.parentElement!);
    });

    await act(async () => {
      vi.advanceTimersByTime(2000);
      vi.advanceTimersByTime(10);
      await flushPromises();
    });

    expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();

    // Also test touchCancel to clear timer
    await act(async () => {
      fireEvent.touchStart(button.parentElement!);
    });
    await act(async () => {
      vi.advanceTimersByTime(200);
      await flushPromises();
    });
    await act(async () => {
      fireEvent.touchCancel(button.parentElement!);
    });
    await act(async () => {
      vi.advanceTimersByTime(300);
      await flushPromises();
    });
    // Should not show because it was cancelled before 500ms
    expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();
  });

  it('handles fetch errors gracefully', async () => {
    mockTooltipFetch.mockImplementationOnce(() => Promise.resolve({ ok: false }));
    await act(async () => {
      render(<TooltipProvider><div>Test</div></TooltipProvider>);
    });
    await flushPromises();
    await act(async () => {
      vi.advanceTimersByTime(10);
    });
    expect(global.fetch).toHaveBeenCalled();
  });
});

describe('useTooltip Hook sync', () => {
  it('throws an error if used outside TooltipProvider', () => {
    const originalError = console.error;
    console.error = vi.fn();

    const TestComponent = () => {
      useTooltip();
      return <div />;
    };

    expect(() => render(<TestComponent />)).toThrow('useTooltip must be used within a TooltipProvider');

    console.error = originalError;
  });
});
