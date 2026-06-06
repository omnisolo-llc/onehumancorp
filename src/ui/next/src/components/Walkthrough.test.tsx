import '@testing-library/jest-dom';

import React from 'react';
import { render, screen, fireEvent, act } from '@testing-library/react';
import { InteractiveWalkthrough } from './Walkthrough';
import { describe, it, expect, vi } from 'vitest';

describe('Walkthrough Component', () => {
  beforeEach(() => {
    // Add a dummy target element to the DOM
    const target = document.createElement('div');
    target.id = 'test-target';
    // Add this style to ensure it occupies space
    target.style.width = '100px';
    target.style.height = '100px';

    // We must return a DOMRect so InteractiveWalkthrough can measure it
    target.getBoundingClientRect = vi.fn(() => ({
      top: 100, left: 100, width: 200, height: 50, right: 300, bottom: 150, x: 100, y: 100, toJSON: () => {}
    }));
    document.body.appendChild(target);
    window.HTMLElement.prototype.scrollIntoView = vi.fn();
    vi.useFakeTimers();
  });

  afterEach(() => {
    document.body.innerHTML = '';
    vi.clearAllMocks();
    vi.useRealTimers();
  });

  it('does not render if not open', () => {
    const { container } = render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'test-target', title: 'Test', content: 'Test Content' }]}
        isOpen={false}
        onClose={() => {}}
      />
    );
    expect(container).toBeEmptyDOMElement();
  });

  it('does not render if no steps provided', () => {
    const { container } = render(
      <InteractiveWalkthrough
        steps={[]}
        isOpen={true}
        onClose={() => {}}
      />
    );
    expect(container).toBeEmptyDOMElement();
  });

  it('renders step content when open', async () => {
    act(() => {
      render(
        <InteractiveWalkthrough
          steps={[{ targetId: 'test-target', title: 'Step 1', content: 'Content 1' }]}
          isOpen={true}
          onClose={() => {}}
        />
      );
    });

    act(() => {
      vi.advanceTimersByTime(350);
    });

    expect(screen.getByText('Step 1')).toBeInTheDocument();
    expect(screen.getByText('Content 1')).toBeInTheDocument();
  });

  it('advances to next step', async () => {
    const steps = [
      { targetId: 'test-target', title: 'Step 1', content: 'Content 1' },
      { targetId: 'test-target', title: 'Step 2', content: 'Content 2' }
    ];

    act(() => {
      render(
        <InteractiveWalkthrough
          steps={steps}
          isOpen={true}
          onClose={() => {}}
        />
      );
    });

    act(() => {
      vi.advanceTimersByTime(350);
    });

    const nextBtn = screen.getByText('Next');
    act(() => {
      fireEvent.click(nextBtn);
    });

    act(() => {
      vi.advanceTimersByTime(350);
    });

    expect(screen.getByText('Step 2')).toBeInTheDocument();
  });

  it('calls onComplete and onClose when finished', async () => {
    const onComplete = vi.fn();
    const onClose = vi.fn();

    act(() => {
      render(
        <InteractiveWalkthrough
          steps={[{ targetId: 'test-target', title: 'Step 1', content: 'Content 1' }]}
          isOpen={true}
          onClose={onClose}
          onComplete={onComplete}
        />
      );
    });

    act(() => {
      vi.advanceTimersByTime(350);
    });

    const finishBtn = screen.getByText('Finish');
    act(() => {
      fireEvent.click(finishBtn);
    });

    expect(onComplete).toHaveBeenCalled();
    expect(onClose).toHaveBeenCalled();
  });

  it('calls onClose when skipped', async () => {
    const onClose = vi.fn();

    act(() => {
      render(
        <InteractiveWalkthrough
          steps={[{ targetId: 'test-target', title: 'Step 1', content: 'Content 1' }]}
          isOpen={true}
          onClose={onClose}
        />
      );
    });

    act(() => {
      vi.advanceTimersByTime(350);
    });

    // The skip button is the SVG icon X button
    const closeBtn = screen.getAllByRole('button')[0];
    act(() => {
      fireEvent.click(closeBtn);
    });

    expect(onClose).toHaveBeenCalled();
  });

  it('handles window resize and scroll gracefully', async () => {
    const steps = [{ targetId: 'test-target', title: 'Step 1', content: 'Content 1' }];

    act(() => {
      render(
        <InteractiveWalkthrough
          steps={steps}
          isOpen={true}
          onClose={() => {}}
        />
      );
    });

    act(() => {
      vi.advanceTimersByTime(350);
    });

    act(() => {
      fireEvent.scroll(window);
      fireEvent.resize(window);
    });

    expect(screen.getByText('Step 1')).toBeInTheDocument();
  });

  it('does not throw when target element is not found', () => {
    act(() => {
      render(
        <InteractiveWalkthrough
          steps={[{ targetId: 'non-existent', title: 'Step 1', content: 'Content 1' }]}
          isOpen={true}
          onClose={() => {}}
        />
      );
    });

    act(() => {
      vi.advanceTimersByTime(350);
    });

    expect(screen.queryByText('Step 1')).not.toBeInTheDocument();
  });

  it('renders correctly with different positions', () => {
    act(() => {
      render(
        <InteractiveWalkthrough
          steps={[
            { targetId: 'test-target', title: 'TopTitle', content: 'Top', position: 'top' },
            { targetId: 'test-target', title: 'RightTitle', content: 'Right', position: 'right' },
            { targetId: 'test-target', title: 'LeftTitle', content: 'Left', position: 'left' }
          ]}
          isOpen={true}
          onClose={() => {}}
        />
      );
    });

    act(() => {
      vi.advanceTimersByTime(350);
    });
    expect(screen.getByText('TopTitle')).toBeInTheDocument();
  });
});
