import React from 'react';
import { render, screen, fireEvent, act } from '@testing-library/react';
import { TooltipProvider, WithTooltip } from './TooltipRegistry';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

describe('TooltipRegistry', () => {

  beforeEach(() => {
    vi.useFakeTimers();
    global.fetch = vi.fn().mockImplementation((url) => {
        if (url === '/api/tooltips') {
            return Promise.resolve({ ok: true, json: async () => ({ "test-id": "Fetched tooltip text" }) });
        }
        return Promise.resolve({ ok: true, json: async () => ({}) });
    }) as any;

    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));
  });

  afterEach(() => {
    vi.clearAllTimers();
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  const renderAndResolve = async (ui: React.ReactElement) => {
      render(ui);
      // Wait for fetch to resolve and microtasks to complete
      await act(async () => {
         await Promise.resolve();
      });
  };

  it('renders default text on hover', async () => {
    await renderAndResolve(
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Default Tooltip">
          <button>Hover me</button>
        </WithTooltip>
      </TooltipProvider>
    );

    const button = screen.getByText('Hover me');

    fireEvent.mouseEnter(button.parentElement!);

    // Act to allow state updates
    act(() => {});

    expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();

    fireEvent.mouseLeave(button.parentElement!);

    act(() => {});

    expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();
  });

  it('does not crash if wrapperRef is null on enter', async () => {
      // Mocking useRef directly is hard without modifying the component.
      // Instead, we just call the mouseEnter on a component that immediately unmounts itself.
      const TestComponent = () => {
         const [show, setShow] = React.useState(true);
         return (
           <TooltipProvider>
             {show && (
                <WithTooltip id="test-id">
                   <button onMouseEnter={() => {
                        setShow(false);
                   }}>Hover me</button>
                </WithTooltip>
             )}
           </TooltipProvider>
         )
      };

      await renderAndResolve(<TestComponent />);
      const button = screen.getByText('Hover me');

      // Simulate state update making it unmount right before or during mouse enter dispatch
      act(() => {
          fireEvent.mouseEnter(button);
      });

      // It just shouldn't throw.
      expect(screen.queryByText('Hover me')).not.toBeInTheDocument();
  });

  it('handles fetch errors gracefully without breaking', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error("Network error"));

    await renderAndResolve(
      <TooltipProvider>
        <WithTooltip id="error-id" defaultText="Fallback text">
          <button>Error Hover</button>
        </WithTooltip>
      </TooltipProvider>
    );

    const button = screen.getByText('Error Hover');

    fireEvent.mouseEnter(button.parentElement!);
    act(() => {});

    expect(screen.getByText('Fallback text')).toBeInTheDocument();
  });

  it('handles non-ok fetch response', async () => {
    global.fetch = vi.fn().mockResolvedValue({ ok: false });

    await renderAndResolve(
      <TooltipProvider>
        <WithTooltip id="error-id" defaultText="Fallback text">
          <button>Error Hover</button>
        </WithTooltip>
      </TooltipProvider>
    );

    const button = screen.getByText('Error Hover');

    fireEvent.mouseEnter(button.parentElement!);
    act(() => {});

    expect(screen.getByText('Fallback text')).toBeInTheDocument();
  });

  it('handles invalid json format', async () => {
    global.fetch = vi.fn().mockResolvedValue({ ok: true, json: async () => ([ "invalid array" ]) });

    await renderAndResolve(
      <TooltipProvider>
        <WithTooltip id="error-id" defaultText="Fallback text">
          <button>Error Hover</button>
        </WithTooltip>
      </TooltipProvider>
    );

    const button = screen.getByText('Error Hover');

    fireEvent.mouseEnter(button.parentElement!);
    act(() => {});

    expect(screen.getByText('Fallback text')).toBeInTheDocument();
  });

  it('handles contextmenu event properly', async () => {
     await renderAndResolve(
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Default Tooltip">
          <button>Right click me</button>
        </WithTooltip>
      </TooltipProvider>
    );

    const button = screen.getByText('Right click me');

    // Create an event that can have its preventDefault called
    const event = new MouseEvent('contextmenu', {
      bubbles: true,
      cancelable: true,
    });

    const preventDefaultSpy = vi.spyOn(event, 'preventDefault');

    fireEvent(button.parentElement!, event);

    expect(preventDefaultSpy).toHaveBeenCalled();
  });

  it('handles touch events for mobile', async () => {
     await renderAndResolve(
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Default Tooltip">
          <button>Touch me</button>
        </WithTooltip>
      </TooltipProvider>
    );

    const button = screen.getByText('Touch me');

    fireEvent.touchStart(button.parentElement!);

    act(() => {
        vi.advanceTimersByTime(500);
    });

    expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();

    fireEvent.touchEnd(button.parentElement!);

    act(() => {
        vi.advanceTimersByTime(2000);
    });

    expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();
  });

  it('handles touch events clearing timeout on multiple touches', async () => {
     await renderAndResolve(
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Default Tooltip">
          <button>Touch me again</button>
        </WithTooltip>
      </TooltipProvider>
    );

    const button = screen.getByText('Touch me again');

    fireEvent.touchStart(button.parentElement!);

    act(() => {
        vi.advanceTimersByTime(250);
    });

    // Touch again to clear and reset the timer
    fireEvent.touchStart(button.parentElement!);

    act(() => {
        vi.advanceTimersByTime(250);
    });

    // First timeout should be cancelled, tooltip not shown yet
    expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();

    act(() => {
        vi.advanceTimersByTime(250);
    });

    expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();
  });


  it('handles touch cancel correctly', async () => {
      await renderAndResolve(
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Default Tooltip">
          <button>Touch cancel</button>
        </WithTooltip>
      </TooltipProvider>
    );

    const button = screen.getByText('Touch cancel');

    fireEvent.touchStart(button.parentElement!);

    act(() => {
        vi.advanceTimersByTime(500);
    });

    expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();

    fireEvent.touchCancel(button.parentElement!);

    act(() => {
        vi.advanceTimersByTime(2000);
    });

    expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();
  });

  it('clears timeout if component unmounts', async () => {
     const { unmount } = render(
      <TooltipProvider>
        <WithTooltip id="test-id" defaultText="Default Tooltip">
          <button>Touch me</button>
        </WithTooltip>
      </TooltipProvider>
    );

    await act(async () => { await Promise.resolve(); });

    const button = screen.getByText('Touch me');

    fireEvent.touchStart(button.parentElement!);

    unmount();

    act(() => {
        vi.advanceTimersByTime(500);
    });

    expect(screen.queryByText('Fetched tooltip text')).not.toBeInTheDocument();
  });


  it('throws an error if used outside provider', () => {
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    expect(() => render(
      <WithTooltip id="test-id" defaultText="Default Tooltip">
        <button>Hover me</button>
      </WithTooltip>
    )).toThrow('useTooltip must be used within a TooltipProvider');

    consoleSpy.mockRestore();
  });
});
