import React from 'react';
import { render, screen, fireEvent, act, waitFor } from '@testing-library/react';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

// We must use dynamic imports to mock fetch BEFORE the module evaluates.
describe('RateLimitWarning', () => {
  let originalFetch: typeof global.fetch;
  let RateLimitWarningProvider: any;
  let useRateLimitWarning: any;

  beforeEach(async () => {
    originalFetch = global.fetch;
    // Mock fetch for tests BEFORE importing
    global.fetch = vi.fn().mockImplementation((url) => {
      if (url === '/api/tooltips') {
        return Promise.resolve(new Response(JSON.stringify({ "test-id": "Tooltip text" })));
      }
      return Promise.resolve(new Response('{}'));
    });

    // Use vitest's vi.resetModules to ensure fresh evaluation
    vi.resetModules();

    const module = await import('./RateLimitWarning');
    RateLimitWarningProvider = module.RateLimitWarningProvider;
    useRateLimitWarning = module.useRateLimitWarning;
  });

  afterEach(() => {
    global.fetch = originalFetch;
    vi.restoreAllMocks();
  });

  it('renders children correctly without warning initially', () => {
    render(
      <RateLimitWarningProvider>
        <div data-testid="child">Child Content</div>
      </RateLimitWarningProvider>
    );

    expect(screen.getByTestId('child')).toBeDefined();
    expect(screen.queryByText('Limit Reached')).toBeNull();
  });

  it('shows warning when context method is called', async () => {
    function TestComponent() {
      const { showWarning } = useRateLimitWarning();
      return (
        <button onClick={() => showWarning("You've hit your Free tier limit")}>
          Trigger Warning
        </button>
      );
    }

    const { TooltipProvider } = await import('./TooltipRegistry');

    render(
      <TooltipProvider>
        <RateLimitWarningProvider>
          <TestComponent />
        </RateLimitWarningProvider>
      </TooltipProvider>
    );

    fireEvent.click(screen.getByText('Trigger Warning'));

    expect(screen.getByText('Limit Reached')).toBeDefined();
    expect(screen.getByText("You've hit your Free tier limit")).toBeDefined();
  });

  it('hides warning when close button is clicked', async () => {
    function TestComponent() {
      const { showWarning } = useRateLimitWarning();
      return (
        <button onClick={() => showWarning("You've hit your Free tier limit")}>
          Trigger Warning
        </button>
      );
    }

    const { TooltipProvider } = await import('./TooltipRegistry');

    render(
      <TooltipProvider>
        <RateLimitWarningProvider>
          <TestComponent />
        </RateLimitWarningProvider>
      </TooltipProvider>
    );

    // Show warning
    fireEvent.click(screen.getByText('Trigger Warning'));
    expect(screen.getByText('Limit Reached')).toBeDefined();

    // Hide warning
    fireEvent.click(screen.getByRole('button', { name: 'Close warning' }));
    expect(screen.queryByText('Limit Reached')).toBeNull();
  });


  it('applies the new Translucent Glass CSS classes', async () => {
    function TestComponent() {
      const { showWarning } = useRateLimitWarning();
      return (
        <button onClick={() => showWarning("You've hit your Free tier limit")}>
          Trigger Warning
        </button>
      );
    }

    const { TooltipProvider } = await import('./TooltipRegistry');

    render(
      <TooltipProvider>
        <RateLimitWarningProvider>
          <TestComponent />
        </RateLimitWarningProvider>
      </TooltipProvider>
    );

    fireEvent.click(screen.getByText('Trigger Warning'));

    const container = screen.getByText('Limit Reached').closest('div')?.parentElement;
    expect(container).toHaveClass('bg-white/65');
    expect(container).toHaveClass('backdrop-blur-[30px]');
    expect(container).toHaveClass('backdrop-saturate-[210%]');
    expect(container).toHaveClass('border-white/40');
  });

  it('renders dark mode styling correctly', async () => {
    function TestComponent() {
      const { showWarning } = useRateLimitWarning();
      return (
        <button onClick={() => showWarning("Dark mode limit reached")}>
          Trigger Warning
        </button>
      );
    }
    const { TooltipProvider } = await import('./TooltipRegistry');
    render(
      <TooltipProvider>
        <RateLimitWarningProvider>
          <TestComponent />
        </RateLimitWarningProvider>
      </TooltipProvider>
    );
    fireEvent.click(screen.getByText('Trigger Warning'));
    const container = screen.getByText('Dark mode limit reached').closest('div')?.parentElement;
    expect(container).toHaveClass('dark:bg-[#16161a]/70');
    expect(container).toHaveClass('dark:border-white/10');
  });

  it('provides accessible alert role', async () => {
    function TestComponent() {
      const { showWarning } = useRateLimitWarning();
      return (
        <button onClick={() => showWarning("Accessible warning")}>
          Trigger Warning
        </button>
      );
    }
    const { TooltipProvider } = await import('./TooltipRegistry');
    render(
      <TooltipProvider>
        <RateLimitWarningProvider>
          <TestComponent />
        </RateLimitWarningProvider>
      </TooltipProvider>
    );
    fireEvent.click(screen.getByText('Trigger Warning'));
    const alert = screen.getByRole('alert');
    expect(alert).toBeDefined();
    expect(alert).toHaveAttribute('aria-live', 'polite');
  });

  it('provides dismiss button with aria-label', async () => {
    function TestComponent() {
      const { showWarning } = useRateLimitWarning();
      return (
        <button onClick={() => showWarning("Button warning")}>
          Trigger Warning
        </button>
      );
    }
    const { TooltipProvider } = await import('./TooltipRegistry');
    render(
      <TooltipProvider>
        <RateLimitWarningProvider>
          <TestComponent />
        </RateLimitWarningProvider>
      </TooltipProvider>
    );
    fireEvent.click(screen.getByText('Trigger Warning'));
    const button = screen.getByRole('button', { name: 'Close warning' });
    expect(button).toBeDefined();
  });

  it('hides after state is manually reset by component', async () => {
    function TestComponent() {
      const { showWarning, hideWarning } = useRateLimitWarning();
      return (
        <div>
          <button onClick={() => showWarning("Warning")}>Show</button>
          <button onClick={() => hideWarning()}>Hide</button>
        </div>
      );
    }
    const { TooltipProvider } = await import('./TooltipRegistry');
    render(
      <TooltipProvider>
        <RateLimitWarningProvider>
          <TestComponent />
        </RateLimitWarningProvider>
      </TooltipProvider>
    );
    fireEvent.click(screen.getByText('Show'));
    expect(screen.getByText('Limit Reached')).toBeDefined();
    fireEvent.click(screen.getByText('Hide'));
    expect(screen.queryByText('Limit Reached')).toBeNull();
  });
});
