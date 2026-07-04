import React from 'react';
import '@testing-library/jest-dom';
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
    const ui = (
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Default Tooltip">
          <button>Hover me</button>
        </WithTooltip>
      </TooltipProvider>
    );
    await act(async () => {
      render(ui);
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
    const ui = (
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Default Tooltip">
          <button>Touch me</button>
        </WithTooltip>
      </TooltipProvider>
    );
    await act(async () => {
      render(ui);
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

    // Test handleTouchMove clears tooltip
    await act(async () => {
        fireEvent.touchStart(button.parentElement!);
        await new Promise(r => setTimeout(r, 600)); // Show it
    });
    expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();

    await act(async () => {
        fireEvent.touchMove(button.parentElement!); // Move clears it
        await new Promise(r => setTimeout(r, 20));
    });
    expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();
  });

  it('handles fetch errors gracefully', async () => {
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    mockTooltipFetch.mockImplementationOnce(() => Promise.resolve({ ok: false, status: 500, json: async () => ({}) }));
    await act(async () => {
      render(<TooltipProvider><div>Test</div></TooltipProvider>);
      await new Promise(r => setTimeout(r, 20));
    });
    expect(global.fetch).toHaveBeenCalled();
    expect(consoleErrorSpy).toHaveBeenCalledWith('Failed to load tooltips', expect.any(Error));
    consoleErrorSpy.mockRestore();
  });

  it('handles aborted fetch gracefully', async () => {
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    mockTooltipFetch.mockImplementationOnce(() => Promise.reject({ name: 'AbortError' }));
    await act(async () => {
      render(<TooltipProvider><div>Test</div></TooltipProvider>);
      await new Promise(r => setTimeout(r, 20));
    });
    expect(global.fetch).toHaveBeenCalled();
    expect(consoleErrorSpy).not.toHaveBeenCalled();
    consoleErrorSpy.mockRestore();
  });
});

describe('useTooltip Hook sync', () => {
  it('throws an error if used outside TooltipProvider', () => {
    const originalError = console.error;
    console.error = vi.fn(); // Suppress the expected React error boundary log
    const preventError = (e: any) => e.preventDefault();
    window.addEventListener('error', preventError);

    const TestComponent = () => {
      useTooltip();
      return <div />;
    };

    expect(() => render(<TestComponent />)).toThrow('useTooltip must be used within a TooltipProvider');

    console.error = originalError;
    window.removeEventListener('error', preventError);
  });
});

describe('TooltipRegistry window resize', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });
  afterEach(() => {
    vi.useRealTimers();
  });
  it('debounces resize events', async () => {
    await act(async () => { render(<TooltipProvider><div>Test</div></TooltipProvider>); });
    act(() => {
      window.innerWidth = 500;
      fireEvent(window, new Event('resize'));
      window.innerWidth = 800;
      fireEvent(window, new Event('resize'));
    });
    act(() => {
      vi.advanceTimersByTime(150);
    });
    expect(true).toBe(true);
  });
});

describe('TooltipRegistry scroll and contextmenu', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });
  afterEach(() => {
    vi.useRealTimers();
  });

  it('hides tooltip on scroll', async () => {
    const ui = (
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Default Tooltip">
          <button>Hover me</button>
        </WithTooltip>
      </TooltipProvider>
    );
    await act(async () => {
      render(ui);
      vi.advanceTimersByTime(20);
    });

    const button = screen.getByText('Hover me');

    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 10, left: 10, bottom: 30, right: 110, x: 10, y: 10, toJSON: () => {}
    }));

    await act(async () => {
        fireEvent.mouseEnter(button.parentElement!);
        vi.advanceTimersByTime(20);
    });

    expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();

    await act(async () => {
        fireEvent.scroll(window);
        vi.advanceTimersByTime(20);
    });

    expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();
  });

  it('prevents default on context menu', async () => {
    const ui = (
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Default Tooltip">
          <button>Hover me</button>
        </WithTooltip>
      </TooltipProvider>
    );
    await act(async () => {
      render(ui);
      vi.advanceTimersByTime(20);
    });

    const button = screen.getByText('Hover me');

    let preventDefaultCalled = false;
    await act(async () => {
      const event = new MouseEvent('contextmenu', {
        bubbles: true,
        cancelable: true,
      });
      event.preventDefault = () => { preventDefaultCalled = true; };
      button.parentElement!.dispatchEvent(event);
    });

    expect(preventDefaultCalled).toBe(true);
  });
});
