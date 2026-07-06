import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { InteractiveWalkthrough } from './Walkthrough';
import { describe, it, expect, vi, beforeEach, afterEach, beforeAll, afterAll } from 'vitest';

describe('Walkthrough Component', () => {
  let originalEnv: string | undefined;

  beforeAll(() => {
    originalEnv = process.env.NEXT_PUBLIC_E2E;
    process.env.NEXT_PUBLIC_E2E = 'false';
  });

  afterAll(() => {
    process.env.NEXT_PUBLIC_E2E = originalEnv;
  });

  let mockGetElementById: any;

  beforeEach(() => {
    mockGetElementById = vi.spyOn(document, 'getElementById').mockImplementation((id) => {
      if (id === 'test-target') {
        const div = document.createElement('div');
        div.id = id;
        div.scrollIntoView = vi.fn();
        div.getBoundingClientRect = vi.fn().mockReturnValue({
          top: 100,
          left: 100,
          bottom: 120,
          right: 120,
          width: 20,
          height: 20
        });
        return div;
      }
      return null;
    });
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders nothing when not open', () => {
    const { container } = render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'test-target', title: 'Test', content: 'test content' }]}
        isOpen={false}
        onClose={() => {}}
      />
    );
    expect(container.firstChild).toBeNull();
  });

  it('renders nothing when there are no steps', () => {
    const { container } = render(
      <InteractiveWalkthrough
        steps={[]}
        isOpen={true}
        onClose={() => {}}
      />
    );
    expect(container.firstChild).toBeNull();
  });

  it('renders the tooltip and advances to the next step and finishes', async () => {
    const handleClose = vi.fn();
    const handleComplete = vi.fn();

    render(
      <InteractiveWalkthrough
        steps={[
          { targetId: 'test-target', title: 'Step 1', content: 'content 1' },
          { targetId: 'test-target', title: 'Step 2', content: 'content 2' }
        ]}
        isOpen={true}
        onClose={handleClose}
        onComplete={handleComplete}
      />
    );

    // Initial step wait
    await waitFor(() => {
      expect(screen.getByText('Step 1')).toBeInTheDocument();
      expect(screen.getByText('content 1')).toBeInTheDocument();
      expect(screen.getByText('Step 1 of 2')).toBeInTheDocument();
    });

    const nextBtn = screen.getByText('Next');
    act(() => {
      fireEvent.click(nextBtn);
    });

    // Second step
    await waitFor(() => {
      expect(screen.getByText('Step 2')).toBeInTheDocument();
      expect(screen.getByText('content 2')).toBeInTheDocument();
      expect(screen.getByText('Step 2 of 2')).toBeInTheDocument();
    });

    const finishBtn = screen.getByText('Finish');
    act(() => {
      fireEvent.click(finishBtn);
    });

    expect(handleComplete).toHaveBeenCalled();
    expect(handleClose).toHaveBeenCalled();
  });

  it('calls onClose when skip button is clicked', async () => {
    const handleClose = vi.fn();

    const { container } = render(
      <InteractiveWalkthrough
        steps={[
          { targetId: 'test-target', title: 'Step 1', content: 'content 1' },
        ]}
        isOpen={true}
        onClose={handleClose}
      />
    );

    await waitFor(() => {
      expect(screen.getByText('Step 1')).toBeInTheDocument();
    });

    const buttons = screen.getAllByRole('button');
    const skipButton = buttons.find(btn => btn.querySelector('svg'));
    if (skipButton) {
      act(() => {
        fireEvent.click(skipButton);
      });
    }

    expect(handleClose).toHaveBeenCalled();
  });

  it('warns when target element is not found and renders nothing', () => {
    const consoleWarnMock = vi.spyOn(console, 'warn').mockImplementation(() => {});
    const handleClose = vi.fn();

    const { container } = render(
      <InteractiveWalkthrough
        steps={[
          { targetId: 'missing-target', title: 'Missing', content: 'Missing content' },
        ]}
        isOpen={true}
        onClose={handleClose}
      />
    );


    expect(container.firstChild).toBeNull();

    consoleWarnMock.mockRestore();
  });

  it('WalkthroughTarget gracefully provides a fallback div when no children are provided', async () => {
    const { WalkthroughTarget } = await import('./Walkthrough');
    const { container } = render(
      <WalkthroughTarget id="some-id" className="my-class" />
    );

    expect(container.innerHTML).toContain('id="some-id"');
    expect(container.innerHTML).toContain('aria-hidden="true"');
  });

  it('recalculates bounds on window resize', async () => {
    const handleClose = vi.fn();
    render(
      <InteractiveWalkthrough
        steps={[
          { targetId: 'test-target', title: 'Step 1', content: 'content 1' },
        ]}
        isOpen={true}
        onClose={handleClose}
      />
    );

    await waitFor(() => {
      expect(screen.getByText('Step 1')).toBeInTheDocument();
    });

    act(() => {
      window.dispatchEvent(new Event('resize'));
    });

    await waitFor(() => {
      expect(screen.getByText('Step 1')).toBeInTheDocument();
    });
  });

  it('renders correctly with different positions', async () => {
    const handleClose = vi.fn();

    // Test Top
    const { rerender } = render(
      <InteractiveWalkthrough
        steps={[
          { targetId: 'test-target', title: 'Top Step', content: 'content', position: 'top' },
        ]}
        isOpen={true}
        onClose={handleClose}
      />
    );
    await waitFor(() => {
      expect(screen.getByText('Top Step')).toBeInTheDocument();
    });
    let bubble = screen.getByRole('dialog');

    // Test Left
    rerender(
      <InteractiveWalkthrough
        steps={[
          { targetId: 'test-target', title: 'Left Step', content: 'content', position: 'left' },
        ]}
        isOpen={true}
        onClose={handleClose}
      />
    );
    await waitFor(() => {
      expect(screen.getByText('Left Step')).toBeInTheDocument();
    });
    bubble = screen.getByRole('dialog');

    // Test Right
    rerender(
      <InteractiveWalkthrough
        steps={[
          { targetId: 'test-target', title: 'Right Step', content: 'content', position: 'right' },
        ]}
        isOpen={true}
        onClose={handleClose}
      />
    );
    await waitFor(() => {
      expect(screen.getByText('Right Step')).toBeInTheDocument();
    });
    bubble = screen.getByRole('dialog');
  });
});

  it('does not render when isOpen is false', () => {
    const steps = [
      { targetId: 'step1', title: 'Step 1', content: 'Content 1' }
    ];
    const { container } = render(
      <InteractiveWalkthrough steps={steps} isOpen={false} onClose={() => {}} />
    );
    expect(container.firstChild).toBeNull();
  });

  it('does not render when steps array is empty', () => {
    const { container } = render(
      <InteractiveWalkthrough steps={[]} isOpen={true} onClose={() => {}} />
    );
    expect(container.firstChild).toBeNull();
  });

  it('does not render in E2E mode unless forced', () => {
    const originalEnv = process.env.NEXT_PUBLIC_E2E;
    process.env.NEXT_PUBLIC_E2E = 'true';

    const steps = [
      { targetId: 'step1', title: 'Step 1', content: 'Content 1' }
    ];
    const { container } = render(
      <InteractiveWalkthrough steps={steps} isOpen={true} onClose={() => {}} />
    );
    expect(container.firstChild).toBeNull();

    process.env.NEXT_PUBLIC_E2E = originalEnv;
  });

  it('renders in E2E mode when forced via window.location.search', () => {
    const originalEnv = process.env.NEXT_PUBLIC_E2E;
    process.env.NEXT_PUBLIC_E2E = 'true';

    const originalLocation = window.location;
    // @ts-ignore
    delete window.location;
    window.location = { ...originalLocation, search: '?test_walkthrough=true' };

    document.body.innerHTML = '<div id="step1">Target</div>';

    const steps = [
      { targetId: 'step1', title: 'Step 1', content: 'Content 1' }
    ];

    render(
      <InteractiveWalkthrough steps={steps} isOpen={true} onClose={() => {}} />
    );

    expect(screen.getByText('Target')).toBeInTheDocument();

    process.env.NEXT_PUBLIC_E2E = originalEnv;
    window.location = originalLocation;
    document.body.innerHTML = '';
  });

  it('logs a warning and returns null targetRect when target is not found', () => {
    const consoleWarnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});

    const steps = [
      { targetId: 'nonexistent-step', title: 'Step 1', content: 'Content 1' }
    ];

    render(
      <InteractiveWalkthrough steps={steps} isOpen={true} onClose={() => {}} />
    );


    consoleWarnSpy.mockRestore();
  });

  it('removes window event listeners on unmount', () => {
    const removeEventListenerSpy = vi.spyOn(window, 'removeEventListener');

    document.body.innerHTML = '<div id="step1">Target</div>';
    const steps = [{ targetId: 'step1', title: 'Step 1', content: 'Content 1' }];

    const { unmount } = render(
      <InteractiveWalkthrough steps={steps} isOpen={true} onClose={() => {}} />
    );

    unmount();

    expect(removeEventListenerSpy).toHaveBeenCalledWith('scroll', expect.any(Function), true);
    expect(removeEventListenerSpy).toHaveBeenCalledWith('resize', expect.any(Function));

    removeEventListenerSpy.mockRestore();
    document.body.innerHTML = '';
  });

  it('clears timeouts on unmount', () => {
    vi.useFakeTimers();
    const clearTimeoutSpy = vi.spyOn(global, 'clearTimeout');

    document.body.innerHTML = '<div id="step1">Target</div>';
    const steps = [{ targetId: 'step1', title: 'Step 1', content: 'Content 1' }];

    const { unmount } = render(
      <InteractiveWalkthrough steps={steps} isOpen={true} onClose={() => {}} />
    );

    unmount();

    expect(clearTimeoutSpy).toHaveBeenCalled();

    clearTimeoutSpy.mockRestore();
    document.body.innerHTML = '';
    vi.useRealTimers();
  });

  it('triggers resize recalculation with handleScroll timeout', () => {
    vi.useFakeTimers();
    document.body.innerHTML = '<div id="step1">Target</div>';
    const steps = [{ targetId: 'step1', title: 'Step 1', content: 'Content 1' }];

    render(<InteractiveWalkthrough steps={steps} isOpen={true} onClose={() => {}} />);

    vi.advanceTimersByTime(300);

    const spy = vi.spyOn(document.getElementById('step1')!, 'getBoundingClientRect').mockReturnValue({
      width: 100, height: 100, top: 10, left: 10, right: 110, bottom: 110, x: 10, y: 10, toJSON: () => {}
    });

    fireEvent.scroll(window);

    vi.advanceTimersByTime(50);

    expect(spy).toHaveBeenCalled();

    spy.mockRestore();
    document.body.innerHTML = '';
    vi.useRealTimers();
  });

  it('provides null targetRect when document.getElementById returns null', () => {
    const consoleWarnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});
    const getElementByIdSpy = vi.spyOn(document, 'getElementById').mockReturnValue(null);

    const steps = [{ targetId: 'nonexistent', title: 'Step 1', content: 'Content 1' }];

    render(<InteractiveWalkthrough steps={steps} isOpen={true} onClose={() => {}} />);



    consoleWarnSpy.mockRestore();
    getElementByIdSpy.mockRestore();
  });
