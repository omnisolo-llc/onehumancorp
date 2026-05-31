import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { TooltipProvider, WithTooltip } from './TooltipRegistry';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

global.fetch = vi.fn() as any;

describe('TooltipRegistry', () => {

  beforeEach(() => {
    (global.fetch as any).mockImplementation((url: string) => {
        if (url === '/api/tooltips') {
            return Promise.resolve({ ok: true, json: async () => ({ "test-id": "Fetched tooltip text" }) });
        }
        return Promise.resolve({ ok: true, json: async () => ({}) });
    });
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('renders default text on hover', async () => {
    render(
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Default Tooltip">
          <button>Hover me</button>
        </WithTooltip>
      </TooltipProvider>
    );

    const button = screen.getByText('Hover me');

    await waitFor(() => {
        expect(global.fetch).toHaveBeenCalled();
    });

    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));

    fireEvent.mouseEnter(button.parentElement!);

    await waitFor(() => {
        expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();
    });

    fireEvent.mouseLeave(button.parentElement!);

    await waitFor(() => {
        expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();
    });
  });

  it('renders default text if fetch fails or returns invalid data', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.resolve({ ok: false }));

    render(
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Default Tooltip">
          <button>Hover me</button>
        </WithTooltip>
      </TooltipProvider>
    );

    const button = screen.getByText('Hover me');

    await waitFor(() => {
        expect(global.fetch).toHaveBeenCalled();
    });

    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));

    fireEvent.mouseEnter(button.parentElement!);

    await waitFor(() => {
        expect(screen.getByText('Default Tooltip')).toBeInTheDocument();
    });
  });

  it('shows tooltip on mobile long press and hides after timeout', async () => {
    render(
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Default Tooltip">
          <button>Touch me</button>
        </WithTooltip>
      </TooltipProvider>
    );

    const button = screen.getByText('Touch me');

    await waitFor(() => {
        expect(global.fetch).toHaveBeenCalled();
    });

    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));

    fireEvent.touchStart(button.parentElement!);

    await waitFor(() => {
        expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();
    }, { timeout: 1000 });

    fireEvent.touchEnd(button.parentElement!);

    await waitFor(() => {
        expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();
    }, { timeout: 3000 });
  });

  it('cancels touch start if released early', async () => {
    render(
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Default Tooltip">
          <button>Touch me</button>
        </WithTooltip>
      </TooltipProvider>
    );

    const button = screen.getByText('Touch me');

    await waitFor(() => {
        expect(global.fetch).toHaveBeenCalled();
    });

    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));

    fireEvent.touchStart(button.parentElement!);

    // Release early before the 500ms timeout
    setTimeout(() => {
        fireEvent.touchCancel(button.parentElement!);
    }, 100);

    // Wait a bit to ensure it doesn't appear
    await new Promise(r => setTimeout(r, 600));

    expect(screen.queryByText('Default Tooltip')).not.toBeInTheDocument();
    expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();
  });
});
