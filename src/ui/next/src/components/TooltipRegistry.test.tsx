import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, fireEvent, act } from '@testing-library/react';
import { TooltipProvider, WithTooltip, useTooltip } from './TooltipRegistry';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

const mockTooltipFetch = vi.fn((url) => {
    if (url && (url === '/api/tooltips' || url.toString().includes('/api/tooltips'))) {
        return Promise.resolve({ ok: true, json: async () => ({ "test-id": "Fetched tooltip text" }) });
    }
    return Promise.resolve({ ok: true, json: async () => ({}) });
});

describe('TooltipRegistry', () => {
  beforeEach(() => {
    mockTooltipFetch.mockClear();
    global.fetch = mockTooltipFetch as any;
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders default text on hover', async () => {
    let button: any;
    await act(async () => {
      render(<TooltipProvider><WithTooltip id="test-id" defaultText="Default Tooltip"><button>Hover me</button></WithTooltip></TooltipProvider>);
      await new Promise(r => setTimeout(r, 20));
    });

    button = screen.getByText('Hover me');

    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 10, left: 10, bottom: 30, right: 110, x: 10, y: 10, toJSON: () => {}
    }));

    await act(async () => {
        fireEvent.mouseEnter(button.parentElement!);
        await new Promise(r => setTimeout(r, 20));
    });

    expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();

    await act(async () => {
        fireEvent.mouseLeave(button.parentElement!);
        await new Promise(r => setTimeout(r, 20));
    });

    expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();
  });

  it('handles touch events (long press) for mobile', async () => {
    let button: any;
    await act(async () => {
      render(<TooltipProvider><WithTooltip id="test-id" defaultText="Default Tooltip"><button>Touch me</button></WithTooltip></TooltipProvider>);
      await new Promise(r => setTimeout(r, 20));
    });

    button = screen.getByText('Touch me');

    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 10, left: 10, bottom: 30, right: 110, x: 10, y: 10, toJSON: () => {}
    }));

    await act(async () => {
        fireEvent.touchStart(button.parentElement!);
        await new Promise(r => setTimeout(r, 550));
    });

    expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();

    await act(async () => {
        fireEvent.touchEnd(button.parentElement!);
        await new Promise(r => setTimeout(r, 2050));
    });

    expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();

    await act(async () => {
        fireEvent.touchStart(button.parentElement!);
        await new Promise(r => setTimeout(r, 200));
        fireEvent.touchCancel(button.parentElement!);
        await new Promise(r => setTimeout(r, 350));
    });

    expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();
  });

  it('handles fetch errors gracefully', async () => {
    mockTooltipFetch.mockImplementationOnce(() => Promise.resolve({ ok: false, json: async () => ({}) }));
    await act(async () => {
      render(<TooltipProvider><div>Test</div></TooltipProvider>);
      await new Promise(r => setTimeout(r, 20));
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
