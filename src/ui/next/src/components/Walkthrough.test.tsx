import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { InteractiveWalkthrough, WalkthroughTarget } from './Walkthrough';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { act } from 'react-dom/test-utils';

describe('Walkthrough Component', () => {
  beforeEach(() => {
    // Mock getBoundingClientRect
    window.HTMLElement.prototype.getBoundingClientRect = function() {
      return {
        width: 100,
        height: 100,
        top: 10,
        left: 10,
        bottom: 110,
        right: 110,
        x: 10,
        y: 10,
        toJSON: () => {}
      };
    };
    window.HTMLElement.prototype.scrollIntoView = vi.fn();
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.runOnlyPendingTimers();
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  it('renders nothing when not open', () => {
    const { container } = render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'test', title: 'Test', content: 'test content' }]}
        isOpen={false}
        onClose={() => {}}
      />
    );
    expect(container.firstChild).toBeNull();
  });

  it('renders correctly when open and targets exist', async () => {
    render(
      <div>
        <div id="test-target">Target</div>
        <InteractiveWalkthrough
          steps={[{ targetId: 'test-target', title: 'Step 1 Title', content: 'Step 1 Content' }]}
          isOpen={true}
          onClose={() => {}}
        />
      </div>
    );

    act(() => {
      vi.advanceTimersByTime(300);
    });

    expect(screen.getByText('Step 1 Title')).toBeInTheDocument();
    expect(screen.getByText('Step 1 Content')).toBeInTheDocument();
    expect(screen.getByText('Step 1 of 1')).toBeInTheDocument();
  });

  it('navigates through steps correctly', async () => {
    const onClose = vi.fn();
    const onComplete = vi.fn();

    render(
      <div>
        <div id="target-1">Target 1</div>
        <div id="target-2">Target 2</div>
        <InteractiveWalkthrough
          steps={[
            { targetId: 'target-1', title: 'Step 1', content: 'Content 1' },
            { targetId: 'target-2', title: 'Step 2', content: 'Content 2' }
          ]}
          isOpen={true}
          onClose={onClose}
          onComplete={onComplete}
        />
      </div>
    );

    act(() => {
      vi.advanceTimersByTime(300);
    });

    expect(screen.getByText('Step 1')).toBeInTheDocument();

    const nextBtn = screen.getByText('Next');
    fireEvent.click(nextBtn);

    act(() => {
      vi.advanceTimersByTime(300);
    });

    expect(screen.getByText('Step 2')).toBeInTheDocument();
    expect(screen.getByText('Finish')).toBeInTheDocument();

    const finishBtn = screen.getByText('Finish');
    fireEvent.click(finishBtn);

    expect(onComplete).toHaveBeenCalled();
    expect(onClose).toHaveBeenCalled();
  });

  it('closes when skip is clicked', async () => {
    const onClose = vi.fn();

    render(
      <div>
        <div id="target-1">Target 1</div>
        <InteractiveWalkthrough
          steps={[{ targetId: 'target-1', title: 'Step 1', content: 'Content 1' }]}
          isOpen={true}
          onClose={onClose}
        />
      </div>
    );

    act(() => {
      vi.advanceTimersByTime(300);
    });

    expect(screen.getByText('Step 1')).toBeInTheDocument();

    const skipBtns = document.querySelectorAll('button');
    const skipBtn = Array.from(skipBtns).find(btn => !btn.textContent || btn.textContent === '');
    if(skipBtn) {
        fireEvent.click(skipBtn);
    }

    expect(onClose).toHaveBeenCalled();
  });

  it('does not crash if target is missing', async () => {
    render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'missing-target', title: 'Step 1', content: 'Content 1' }]}
        isOpen={true}
        onClose={() => {}}
      />
    );

    act(() => {
      vi.advanceTimersByTime(300);
    });

    expect(screen.queryByText('Step 1')).toBeNull();
  });
});

describe('WalkthroughTarget', () => {
  it('renders children with id', () => {
    const { container } = render(
      <WalkthroughTarget id="test-id" className="test-class">
        <span>Child</span>
      </WalkthroughTarget>
    );

    const div = container.querySelector('#test-id');
    expect(div).toBeInTheDocument();
    expect(div).toHaveClass('test-class');
    expect(div).toHaveTextContent('Child');
  });
});
