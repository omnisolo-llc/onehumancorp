import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { InteractiveWalkthrough, WalkthroughTarget } from './Walkthrough';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import userEvent from '@testing-library/user-event';

describe('Walkthrough Component', () => {
  let removeEventListenerSpy: any;

  beforeEach(() => {
    vi.useFakeTimers();
    // Mock getBoundingClientRect
    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 50, top: 10, left: 10, bottom: 60, right: 110, x: 10, y: 10, toJSON: () => {}
    }));
    Element.prototype.scrollIntoView = vi.fn();

    removeEventListenerSpy = vi.spyOn(window, 'removeEventListener');

    // Setup a mock root element for tests
    document.body.innerHTML = '';
  });

  afterEach(() => {
    vi.clearAllTimers();
    vi.useRealTimers();
    document.body.innerHTML = '';
    vi.restoreAllMocks();
  });

  it('renders nothing when not open', () => {
    const { container } = render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'test-target', title: 'Test Title', content: 'test content' }]}
        isOpen={false}
        onClose={() => {}}
      />
    );
    expect(container.firstChild).toBeNull();
  });

  it('renders step when open', async () => {
    render(
      <div>
        <div id="test-target">Target</div>
        <InteractiveWalkthrough
          steps={[{ targetId: 'test-target', title: 'Test Title', content: 'test content' }]}
          isOpen={true}
          onClose={() => {}}
        />
      </div>
    );

    act(() => {
      vi.advanceTimersByTime(350);
    });

    expect(screen.getByText('Test Title')).toBeInTheDocument();
    expect(screen.getByText('test content')).toBeInTheDocument();
  });

  it('handles next step and close', async () => {
    const onClose = vi.fn();
    const onComplete = vi.fn();

    render(
      <div>
        <div id="test-target">Target 1</div>
        <div id="test-target-2">Target 2</div>
        <InteractiveWalkthrough
          steps={[
            { targetId: 'test-target', title: 'Step 1', content: 'Content 1' },
            { targetId: 'test-target-2', title: 'Step 2', content: 'Content 2' }
          ]}
          isOpen={true}
          onClose={onClose}
          onComplete={onComplete}
        />
      </div>
    );

    act(() => {
      vi.advanceTimersByTime(350);
    });

    expect(screen.getByText('Step 1')).toBeInTheDocument();

    const nextBtn = screen.getByText('Next');
    fireEvent.click(nextBtn);

    act(() => {
      vi.advanceTimersByTime(350);
    });

    expect(screen.getByText('Step 2')).toBeInTheDocument();

    const finishBtn = screen.getByText('Finish');
    fireEvent.click(finishBtn);

    expect(onComplete).toHaveBeenCalled();
    expect(onClose).toHaveBeenCalled();
  });

  it('handles skip', async () => {
    const onClose = vi.fn();
    render(
      <div>
        <div id="test-target">Target</div>
        <InteractiveWalkthrough
          steps={[{ targetId: 'test-target', title: 'Step 1', content: 'Content 1' }]}
          isOpen={true}
          onClose={onClose}
        />
      </div>
    );

    act(() => {
      vi.advanceTimersByTime(350);
    });

    const button = document.querySelector('button.text-gray-500');
    fireEvent.click(button!);

    expect(onClose).toHaveBeenCalled();
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

  it('renders nothing if target element is not found', () => {
    const { container } = render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'nonexistent', title: 'Test', content: 'test content' }]}
        isOpen={true}
        onClose={() => {}}
      />
    );
    expect(container.firstChild).toBeNull();
  });

  it('renders correctly when target is found', () => {
    render(
      <div>
        <div id="target1">Target 1</div>
        <InteractiveWalkthrough
          steps={[{ targetId: 'target1', title: 'Test Title', content: 'Test Content' }]}
          isOpen={true}
          onClose={() => {}}
        />
      </div>
    );

    act(() => {
        vi.advanceTimersByTime(350);
    });

    expect(screen.getByText('Test Title')).toBeInTheDocument();
    expect(screen.getByText('Test Content')).toBeInTheDocument();
    expect(screen.getByText('Step 1 of 1')).toBeInTheDocument();
    expect(screen.getByText('Finish')).toBeInTheDocument();
  });

  it('handles Next button click to progress to next step', () => {
    render(
      <div>
        <div id="target1">Target 1</div>
        <div id="target2">Target 2</div>
        <InteractiveWalkthrough
          steps={[
            { targetId: 'target1', title: 'Step 1', content: 'Content 1' },
            { targetId: 'target2', title: 'Step 2', content: 'Content 2' }
          ]}
          isOpen={true}
          onClose={() => {}}
        />
      </div>
    );

    act(() => {
        vi.advanceTimersByTime(350);
    });

    expect(screen.getByText('Step 1')).toBeInTheDocument();

    fireEvent.click(screen.getByText('Next'));

    act(() => {
        vi.advanceTimersByTime(350);
    });

    expect(screen.getByText('Step 2')).toBeInTheDocument();
    expect(screen.getByText('Step 2 of 2')).toBeInTheDocument();
  });

  it('handles Finish button click to complete walkthrough', () => {
    const handleClose = vi.fn();
    const handleComplete = vi.fn();

    render(
      <div>
        <div id="target1">Target 1</div>
        <InteractiveWalkthrough
          steps={[{ targetId: 'target1', title: 'Step 1', content: 'Content 1' }]}
          isOpen={true}
          onClose={handleClose}
          onComplete={handleComplete}
        />
      </div>
    );

    act(() => {
        vi.advanceTimersByTime(350);
    });

    fireEvent.click(screen.getByText('Finish'));

    expect(handleComplete).toHaveBeenCalledTimes(1);
    expect(handleClose).toHaveBeenCalledTimes(1);
  });

  it('handles skip/close button click', () => {
    const handleClose = vi.fn();

    render(
      <div>
        <div id="target1">Target 1</div>
        <InteractiveWalkthrough
          steps={[{ targetId: 'target1', title: 'Step 1', content: 'Content 1' }]}
          isOpen={true}
          onClose={handleClose}
        />
      </div>
    );

    act(() => {
        vi.advanceTimersByTime(350);
    });

    const button = document.querySelector('button.text-gray-500');
    fireEvent.click(button!);

    expect(handleClose).toHaveBeenCalledTimes(1);
  });

  it('renders different positions correctly', () => {
    const { unmount } = render(
      <div>
        <div id="target1">Target 1</div>
        <InteractiveWalkthrough
          steps={[{ targetId: 'target1', title: 'Step 1', content: 'Content 1', position: 'top' }]}
          isOpen={true}
          onClose={() => {}}
        />
      </div>
    );
    act(() => { vi.advanceTimersByTime(350); });
    expect(screen.getByText('Step 1')).toBeInTheDocument();
    unmount();

    const { unmount: unmount2 } = render(
      <div>
        <div id="target1">Target 1</div>
        <InteractiveWalkthrough
          steps={[{ targetId: 'target1', title: 'Step 1', content: 'Content 1', position: 'left' }]}
          isOpen={true}
          onClose={() => {}}
        />
      </div>
    );
    act(() => { vi.advanceTimersByTime(350); });
    expect(screen.getByText('Step 1')).toBeInTheDocument();
    unmount2();

    const { unmount: unmount3 } = render(
      <div>
        <div id="target1">Target 1</div>
        <InteractiveWalkthrough
          steps={[{ targetId: 'target1', title: 'Step 1', content: 'Content 1', position: 'right' }]}
          isOpen={true}
          onClose={() => {}}
        />
      </div>
    );
    act(() => { vi.advanceTimersByTime(350); });
    expect(screen.getByText('Step 1')).toBeInTheDocument();
    unmount3();
  });

  it('handles window scroll and resize events and unmounting clears listeners', () => {
      const { unmount } = render(
      <div>
        <div id="target1">Target 1</div>
        <InteractiveWalkthrough
          steps={[{ targetId: 'target1', title: 'Step 1', content: 'Content 1' }]}
          isOpen={true}
          onClose={() => {}}
        />
      </div>
    );

    act(() => { vi.advanceTimersByTime(350); });
    expect(screen.getByText('Step 1')).toBeInTheDocument();

    act(() => {
        fireEvent.scroll(window);
        fireEvent.resize(window);
    });

    unmount();
    expect(removeEventListenerSpy).toHaveBeenCalled();
  });

  it('renders nothing when E2E mode is on', () => {
      process.env.NEXT_PUBLIC_E2E = 'true';
      const { container } = render(
        <div>
          <div id="target1">Target 1</div>
          <InteractiveWalkthrough
            steps={[{ targetId: 'target1', title: 'Step 1', content: 'Content 1' }]}
            isOpen={true}
            onClose={() => {}}
          />
        </div>
      );
      expect(container.firstChild?.childNodes.length).toBe(1);
      process.env.NEXT_PUBLIC_E2E = undefined;
  });
});

describe('WalkthroughTarget Component', () => {
    it('renders children with given id and className', () => {
        render(
            <WalkthroughTarget id="my-target" className="my-class">
                <span>Child content</span>
            </WalkthroughTarget>
        );

        const target = document.getElementById('my-target');
        expect(target).toBeInTheDocument();
        expect(target).toHaveClass('my-class');
        expect(target).toHaveTextContent('Child content');
    });
});
