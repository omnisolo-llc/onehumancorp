import '@testing-library/jest-dom';
import { render,screen,fireEvent,act } from '@testing-library/react';
import { TooltipProvider,WithTooltip,useTooltip } from './TooltipRegistry';
import { describe,it,expect,vi,beforeEach,afterEach } from 'vitest';

const navigationMocks = vi.hoisted(() => ({ pathname: '/' }));

vi.mock('next/navigation', () => ({
  usePathname: () => navigationMocks.pathname,
}));

vi.mock("framer-motion", () => {
  return {
    motion: {
      div: ({ children, ...props }: import('react').HTMLAttributes<HTMLDivElement> & {
        initial?: unknown; animate?: unknown; exit?: unknown; transition?: unknown;
      }) => {
        for (const key of ['initial', 'animate', 'exit', 'transition'] as const) delete props[key];
        return <div {...props}>{children}</div>;
      },
    },
    AnimatePresence: ({ children }: { children?: import('react').ReactNode }) => <>{children}</>,
  };
});

const mockTooltipFetch = vi.fn<typeof fetch>((url) => {
    if (url && (url === '/api/v1/tooltips' || url.toString().includes('/api/v1/tooltips'))) {
        return Promise.resolve(Response.json({ "test-id": "Fetched tooltip text" }));
    }
    return Promise.resolve(Response.json({}));
});

describe('TooltipRegistry', () => {
  beforeEach(() => {
    navigationMocks.pathname = '/';
    mockTooltipFetch.mockClear();
    global.fetch = mockTooltipFetch;
  });

  afterEach(() => {
    vi.restoreAllMocks();
    vi.useRealTimers();
  });

  it('renders default text on hover', async () => {
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

    const button = screen.getByText('Hover me');

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
    vi.useFakeTimers();
    const advance = async (milliseconds: number) => {
      await act(async () => { await vi.advanceTimersByTimeAsync(milliseconds); });
    };
    await act(async () => {
      render(<TooltipProvider><WithTooltip id="test-id" defaultText="Default Tooltip">
        <button>Touch me</button>
      </WithTooltip></TooltipProvider>);
    });
    const wrapper = screen.getByText('Touch me').parentElement!;
    vi.spyOn(wrapper, 'getBoundingClientRect').mockReturnValue(new DOMRect(10, 10, 100, 20));
    fireEvent.touchStart(wrapper);
    await advance(499);
    expect(screen.queryByRole('tooltip')).not.toBeInTheDocument();
    await advance(1);
    expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();

    fireEvent.touchEnd(wrapper);
    await advance(1999);
    expect(screen.getByRole('tooltip')).toBeInTheDocument();
    await advance(1);
    expect(screen.queryByRole('tooltip')).not.toBeInTheDocument();

    fireEvent.touchStart(wrapper);
    await advance(200);
    fireEvent.touchCancel(wrapper);
    await advance(350);
    expect(screen.queryByRole('tooltip')).not.toBeInTheDocument();

    fireEvent.touchStart(wrapper);
    await advance(500);
    expect(screen.getByRole('tooltip')).toBeInTheDocument();
    fireEvent.touchMove(wrapper);
    expect(screen.queryByRole('tooltip')).not.toBeInTheDocument();
  });

  it('falls back silently when the optional tooltip service is unavailable', async () => {
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    mockTooltipFetch.mockResolvedValueOnce(Response.json({}, { status: 500 }));
    await act(async () => {
      render(<TooltipProvider><div>Test</div></TooltipProvider>);
      await new Promise(r => setTimeout(r, 20));
    });
    expect(global.fetch).toHaveBeenCalled();
    expect(consoleErrorSpy).not.toHaveBeenCalled();
    consoleErrorSpy.mockRestore();
  });

  it('does not fetch optional tooltips on the public login route', async () => {
    navigationMocks.pathname = '/login';

    await act(async () => {
      render(<TooltipProvider><div>Login</div></TooltipProvider>);
      await new Promise(r => setTimeout(r, 20));
    });

    expect(mockTooltipFetch).not.toHaveBeenCalled();
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
    const preventError = (e: ErrorEvent) => e.preventDefault();
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
  it('debounces resize events and uses the latest viewport width', async () => {
    const originalWidth = window.innerWidth;
    try {
      window.innerWidth = 1024;
      await act(async () => {
        render(<TooltipProvider><WithTooltip id="resize" defaultText="Resize tooltip">
          <button>Resize target</button>
        </WithTooltip></TooltipProvider>);
      });
      const wrapper = screen.getByText('Resize target').parentElement!;
      vi.spyOn(wrapper, 'getBoundingClientRect').mockReturnValue(new DOMRect(900, 10, 100, 20));
      fireEvent.mouseEnter(wrapper);
      expect(screen.getByRole('tooltip')).toHaveStyle({ left: '880px' });
      act(() => {
        window.innerWidth = 500;
        fireEvent(window, new Event('resize'));
        window.innerWidth = 800;
        fireEvent(window, new Event('resize'));
        vi.advanceTimersByTime(149);
      });
      expect(screen.getByRole('tooltip')).toHaveStyle({ left: '880px' });
      act(() => { vi.advanceTimersByTime(1); });
      expect(screen.getByRole('tooltip')).toHaveStyle({ left: '656px' });
    } finally { window.innerWidth = originalWidth; }
  });

  it('cancels pending scroll work when the provider unmounts', async () => {
    let view: ReturnType<typeof render>;
    await act(async () => { view = render(<TooltipProvider><div>Test</div></TooltipProvider>); });
    fireEvent.scroll(window);
    expect(vi.getTimerCount()).toBe(1);
    view!.unmount();
    expect(vi.getTimerCount()).toBe(0);
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
      vi.advanceTimersByTime(200);
    });

    const button = screen.getByText('Hover me');

    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 10, left: 10, bottom: 30, right: 110, x: 10, y: 10, toJSON: () => {}
    }));

    await act(async () => {
        fireEvent.mouseEnter(button.parentElement!);
        vi.advanceTimersByTime(200);
    });

    expect(screen.getByText('Fetched tooltip text')).toBeInTheDocument();

    await act(async () => {
        fireEvent.scroll(window);
        vi.advanceTimersByTime(200);
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
      vi.advanceTimersByTime(200);
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
