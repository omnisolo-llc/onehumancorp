import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { TooltipProvider, WithTooltip } from './TooltipRegistry';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

global.fetch = vi.fn().mockImplementation((url) => {
    if (url === '/api/tooltips') {
        return Promise.resolve({ ok: true, json: async () => ({ "test-id": "Fetched tooltip text" }) });
    }
    return Promise.resolve({ ok: true, json: async () => ({}) });
}) as any;

describe('TooltipRegistry', () => {

  it('renders default text on hover', async () => {
    await act(async () => {
      render(
        <TooltipProvider>
          <WithTooltip id="test-id" defaultText="Default Tooltip">
            <button>Hover me</button>
          </WithTooltip>
        </TooltipProvider>
      );
    });

    const button = screen.getByText('Hover me');

    // Wait for context to be populated via fetch call in TooltipProvider
    await waitFor(() => {
      expect(global.fetch).toHaveBeenCalled();
    });

    // Create a mock getBoundingClientRect
    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));

    await act(async () => {
      fireEvent.mouseEnter(button.parentElement!);
    });

    await waitFor(() => {
      expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();
    }, { timeout: 1000 });

    await act(async () => {
      fireEvent.mouseLeave(button.parentElement!);
    });

    await waitFor(() => {
      expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();
    });
  });

  it('handles long press for mobile', async () => {
    vi.useFakeTimers();

    await act(async () => {
      render(
        <TooltipProvider>
          <WithTooltip id="test-id" defaultText="Default Tooltip">
            <button>Touch me</button>
          </WithTooltip>
        </TooltipProvider>
      );
    });

    const button = screen.getByText('Touch me');

    await act(async () => {
      await vi.runAllTimersAsync();
    });

    expect(global.fetch).toHaveBeenCalled();

    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));

    await act(async () => {
      fireEvent.touchStart(button.parentElement!);
    });

    // Should not show immediately
    expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();

    // Advance by 500ms for long press
    await act(async () => {
      vi.advanceTimersByTime(600);
      await Promise.resolve(); // flush microtasks
    });

    expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();

    await act(async () => {
      fireEvent.touchEnd(button.parentElement!);
    });

    // It should stay for 2 seconds after touch end
    expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();

    await act(async () => {
      vi.advanceTimersByTime(2000);
      await Promise.resolve(); // flush microtasks
    });

    expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();

    vi.useRealTimers();
  });
});
