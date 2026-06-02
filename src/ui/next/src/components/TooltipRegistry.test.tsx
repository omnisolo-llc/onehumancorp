import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { TooltipProvider, WithTooltip } from './TooltipRegistry';
import { describe, it, expect, vi, beforeEach } from 'vitest';

global.fetch = vi.fn().mockImplementation((url) => {
    if (url === '/api/tooltips') {
        return Promise.resolve({ ok: true, json: async () => ({ "test-id": "Fetched tooltip text" }) });
    }
    return Promise.resolve({ ok: true, json: async () => ({}) });
}) as any;

describe('TooltipRegistry', () => {

  beforeEach(() => {
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

    // Wait for context to be populated via fetch call in TooltipProvider
    await waitFor(() => {
        expect(global.fetch).toHaveBeenCalled();
    })

    // Create a mock getBoundingClientRect
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

  it('handles touch events for mobile', async () => {
    render(
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Default Tooltip">
          <button>Touch me</button>
        </WithTooltip>
      </TooltipProvider>
    );

    const button = screen.getByText('Touch me');

    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));

    fireEvent.touchStart(button.parentElement!);

    await waitFor(() => {
      expect(screen.getByText('Default Tooltip')).toBeInTheDocument();
    }, { timeout: 1000 });

    fireEvent.touchEnd(button.parentElement!);

    await waitFor(() => {
      expect(screen.queryByText('Default Tooltip')).not.toBeInTheDocument();
    }, { timeout: 2500 });

    // Test touchCancel
    fireEvent.touchStart(button.parentElement!);
    fireEvent.touchCancel(button.parentElement!);

    // Test context menu
    fireEvent.contextMenu(button.parentElement!);
  });

  it('handles fetch failure gracefully', async () => {
    (global.fetch as any).mockImplementationOnce(() => Promise.reject(new Error('Network error')));

    render(
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Fallback Tooltip">
          <button>Hover me</button>
        </WithTooltip>
      </TooltipProvider>
    );

    const button = screen.getByText('Hover me');

    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));

    fireEvent.mouseEnter(button.parentElement!);

    await waitFor(() => {
      expect(screen.getByText('Fallback Tooltip')).toBeInTheDocument();
    });
  });

  it('throws error when useTooltip is used outside provider', () => {
    const consoleError = vi.spyOn(console, 'error').mockImplementation(() => {});

    expect(() => render(
        <WithTooltip id="test" defaultText="test">
            <div>Test</div>
        </WithTooltip>
    )).toThrow('useTooltip must be used within a TooltipProvider');

    consoleError.mockRestore();
  });
});
