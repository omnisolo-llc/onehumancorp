import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { InteractiveWalkthrough, WalkthroughTarget } from './Walkthrough';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

describe('Walkthrough Component', () => {
  const originalEnv = process.env;

  beforeEach(() => {
    vi.clearAllMocks();
    process.env = { ...originalEnv };
  });

  afterEach(() => {
    process.env = originalEnv;
  });

  it('renders nothing when closed', () => {
    const { container } = render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'test', title: 'Test', content: 'Content' }]}
        isOpen={false}
        onClose={vi.fn()}
      />
    );
    expect(container).toBeEmptyDOMElement();
  });

  it('renders nothing when there are no steps', () => {
    const { container } = render(
      <InteractiveWalkthrough
        steps={[]}
        isOpen={true}
        onClose={vi.fn()}
      />
    );
    expect(container).toBeEmptyDOMElement();
  });

  it('renders the tooltip and advances to the next step and finishes', async () => {
    const onClose = vi.fn();
    const onComplete = vi.fn();

    // Setup dom target
    document.body.innerHTML = '<div id="step1">Step 1 Target</div><div id="step2">Step 2 Target</div>';

    // Mock getBoundingClientRect
    Element.prototype.getBoundingClientRect = vi.fn(() => ({
        width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));
    Element.prototype.scrollIntoView = vi.fn();

    render(
      <InteractiveWalkthrough
        steps={[
          { targetId: 'step1', title: 'Step 1', content: 'First step content' },
          { targetId: 'step2', title: 'Step 2', content: 'Second step content', position: 'top' },
          { targetId: 'step1', title: 'Step 3', content: 'Third step content', position: 'left' },
          { targetId: 'step2', title: 'Step 4', content: 'Fourth step content', position: 'right' }
        ]}
        isOpen={true}
        onClose={onClose}
        onComplete={onComplete}
      />
    );

    // Initial step 1
    await waitFor(() => {
        expect(screen.getByText('Step 1')).toBeInTheDocument();
        expect(screen.getByText('First step content')).toBeInTheDocument();
    });

    const nextButton = screen.getByText('Next');
    fireEvent.click(nextButton);

    // Step 2
    await waitFor(() => {
        expect(screen.getByText('Step 2')).toBeInTheDocument();
    });
    fireEvent.click(screen.getByText('Next'));

    // Step 3
    await waitFor(() => {
        expect(screen.getByText('Step 3')).toBeInTheDocument();
    });
    fireEvent.click(screen.getByText('Next'));

    // Step 4
    await waitFor(() => {
        expect(screen.getByText('Step 4')).toBeInTheDocument();
    });

    // Finish
    const finishButton = screen.getByText('Finish');
    fireEvent.click(finishButton);

    expect(onComplete).toHaveBeenCalled();
    expect(onClose).toHaveBeenCalled();
  });

  it('calls onClose when skip button is clicked', async () => {
    const onClose = vi.fn();

    document.body.innerHTML = '<div id="step1">Step 1 Target</div>';
    Element.prototype.getBoundingClientRect = vi.fn(() => ({
        width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));

    render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'step1', title: 'Step 1', content: 'First step' }]}
        isOpen={true}
        onClose={onClose}
      />
    );

    await waitFor(() => {
        expect(screen.getByText('Step 1')).toBeInTheDocument();
    });

    // Need a specific selector for the skip button which is just an SVG icon
    // Using closest matching button on the SVG
    const skipSvg = document.querySelector('svg path[d="M6 18L18 6M6 6l12 12"]');
    const skipButton = skipSvg?.closest('button');

    expect(skipButton).not.toBeNull();
    fireEvent.click(skipButton!);

    expect(onClose).toHaveBeenCalled();
  });

  it('renders WalkthroughTarget helper correctly', () => {
      const { container } = render(
          <WalkthroughTarget id="my-target" className="custom-class">
              <div>Target Content</div>
          </WalkthroughTarget>
      );

      const targetDiv = container.firstChild as HTMLElement;
      expect(targetDiv.id).toBe('my-target');
      expect(targetDiv.className).toContain('custom-class');
      expect(screen.getByText('Target Content')).toBeInTheDocument();
  });

  it('handles missing target elements gracefully', async () => {
      // Clear document body so target is missing
      document.body.innerHTML = '';

      const { container } = render(
        <InteractiveWalkthrough
          steps={[{ targetId: 'missing-step', title: 'Step 1', content: 'First step' }]}
          isOpen={true}
          onClose={vi.fn()}
        />
      );

      // We need to wait for the effect to run
      await new Promise(r => setTimeout(r, 500));

      // Should log a warning and render null
      expect(container).toBeEmptyDOMElement();
  });

  it('returns null when NEXT_PUBLIC_E2E is true', () => {
    process.env.NEXT_PUBLIC_E2E = 'true';
    document.body.innerHTML = '<div id="step1">Step 1 Target</div>';

    const { container } = render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'step1', title: 'Step 1', content: 'First step' }]}
        isOpen={true}
        onClose={vi.fn()}
      />
    );
    expect(container).toBeEmptyDOMElement();
  });

  it('removes event listeners on unmount', () => {
    const addEventListenerSpy = vi.spyOn(window, 'addEventListener');
    const removeEventListenerSpy = vi.spyOn(window, 'removeEventListener');

    document.body.innerHTML = '<div id="step1">Step 1 Target</div>';
    Element.prototype.getBoundingClientRect = vi.fn(() => ({
        width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));

    const { unmount } = render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'step1', title: 'Step 1', content: 'First step' }]}
        isOpen={true}
        onClose={vi.fn()}
      />
    );

    // Initial mount shouldn't immediately add listeners until effect runs
    // But let's verify removeEventListener is called on unmount
    unmount();

    expect(removeEventListenerSpy).toHaveBeenCalledWith('scroll', expect.any(Function), true);
    expect(removeEventListenerSpy).toHaveBeenCalledWith('resize', expect.any(Function));

    addEventListenerSpy.mockRestore();
    removeEventListenerSpy.mockRestore();
  });

  it('removes event listeners for scroll and resize', async () => {
    // Similar to unmount test, but let's test the effect cleanup more directly
    const addEventListenerSpy = vi.spyOn(window, 'addEventListener');
    const removeEventListenerSpy = vi.spyOn(window, 'removeEventListener');

    document.body.innerHTML = '<div id="step1">Step 1 Target</div>';
    Element.prototype.getBoundingClientRect = vi.fn(() => ({
        width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));

    const { unmount } = render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'step1', title: 'Step 1', content: 'First step' }]}
        isOpen={true}
        onClose={vi.fn()}
      />
    );

    // Give the effect time to run and attach listeners
    await waitFor(() => {
       expect(addEventListenerSpy).toHaveBeenCalledWith('scroll', expect.any(Function), true);
       expect(addEventListenerSpy).toHaveBeenCalledWith('resize', expect.any(Function));
    });

    unmount();

    expect(removeEventListenerSpy).toHaveBeenCalledWith('scroll', expect.any(Function), true);
    expect(removeEventListenerSpy).toHaveBeenCalledWith('resize', expect.any(Function));

    addEventListenerSpy.mockRestore();
    removeEventListenerSpy.mockRestore();
  });

  it('handles space key for navigation', async () => {
    const onClose = vi.fn();

    document.body.innerHTML = '<div id="step1">Step 1 Target</div><div id="step2">Step 2 Target</div>';
    Element.prototype.getBoundingClientRect = vi.fn(() => ({
        width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));

    render(
      <InteractiveWalkthrough
        steps={[
          { targetId: 'step1', title: 'Step 1', content: 'First step content' },
          { targetId: 'step2', title: 'Step 2', content: 'Second step content' }
        ]}
        isOpen={true}
        onClose={onClose}
      />
    );

    await waitFor(() => {
        expect(screen.getByText('Step 1')).toBeInTheDocument();
    });

    const dialog = screen.getByRole('dialog');

    fireEvent.keyDown(dialog, { key: ' ', code: 'Space' });

    await waitFor(() => {
        expect(screen.getByText('Step 2')).toBeInTheDocument();
    });
  });

  it('handles keyboard navigation (Enter to next, Escape to skip)', async () => {
    const onClose = vi.fn();
    const onComplete = vi.fn();

    document.body.innerHTML = '<div id="step1">Step 1 Target</div><div id="step2">Step 2 Target</div>';
    Element.prototype.getBoundingClientRect = vi.fn(() => ({
        width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));

    render(
      <InteractiveWalkthrough
        steps={[
          { targetId: 'step1', title: 'Step 1', content: 'First step content' },
          { targetId: 'step2', title: 'Step 2', content: 'Second step content' }
        ]}
        isOpen={true}
        onClose={onClose}
        onComplete={onComplete}
      />
    );

    await waitFor(() => {
        expect(screen.getByText('Step 1')).toBeInTheDocument();
    });

    const dialog = screen.getByRole('dialog');

    // Press Enter to go to next step
    fireEvent.keyDown(dialog, { key: 'Enter', code: 'Enter' });

    await waitFor(() => {
        expect(screen.getByText('Step 2')).toBeInTheDocument();
    });

    // Press Escape to skip/close
    fireEvent.keyDown(dialog, { key: 'Escape', code: 'Escape' });

    expect(onClose).toHaveBeenCalled();
  });
});
