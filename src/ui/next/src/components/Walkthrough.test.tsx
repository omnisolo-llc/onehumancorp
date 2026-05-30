import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { InteractiveWalkthrough, WalkthroughTarget } from './Walkthrough';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

describe('Walkthrough Component', () => {

  beforeEach(() => {
    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));
    Element.prototype.scrollIntoView = vi.fn();
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.clearAllTimers();
    vi.useRealTimers();
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

  it('renders walkthrough with target', async () => {
    const steps = [
      { targetId: 'test1', title: 'Step 1', content: 'Content 1', position: 'bottom' as const },
      { targetId: 'test2', title: 'Step 2', content: 'Content 2', position: 'top' as const },
      { targetId: 'test3', title: 'Step 3', content: 'Content 3', position: 'left' as const },
      { targetId: 'test4', title: 'Step 4', content: 'Content 4', position: 'right' as const },
    ];

    const handleClose = vi.fn();
    const handleComplete = vi.fn();

    render(
      <div>
        <WalkthroughTarget id="test1"><div>Target 1</div></WalkthroughTarget>
        <WalkthroughTarget id="test2"><div>Target 2</div></WalkthroughTarget>
        <WalkthroughTarget id="test3"><div>Target 3</div></WalkthroughTarget>
        <WalkthroughTarget id="test4"><div>Target 4</div></WalkthroughTarget>
        <InteractiveWalkthrough
          steps={steps}
          isOpen={true}
          onClose={handleClose}
          onComplete={handleComplete}
        />
      </div>
    );

    act(() => {
      vi.advanceTimersByTime(350);
    });

    expect(screen.getByText('Step 1')).toBeInTheDocument();
    expect(screen.getByText('Content 1')).toBeInTheDocument();

    const nextButton = screen.getByRole('button', { name: 'Next' });

    act(() => {
      fireEvent.click(nextButton);
      vi.advanceTimersByTime(350);
    });

    expect(screen.getByText('Step 2')).toBeInTheDocument();

    act(() => {
      fireEvent.click(screen.getByRole('button', { name: 'Next' }));
      vi.advanceTimersByTime(350);
    });

    expect(screen.getByText('Step 3')).toBeInTheDocument();

    act(() => {
      fireEvent.click(screen.getByRole('button', { name: 'Next' }));
      vi.advanceTimersByTime(350);
    });

    expect(screen.getByText('Step 4')).toBeInTheDocument();

    const finishButton = screen.getByRole('button', { name: 'Finish' });

    act(() => {
      fireEvent.click(finishButton);
    });

    expect(handleComplete).toHaveBeenCalled();
    expect(handleClose).toHaveBeenCalled();
  });

  it('handles skip', async () => {
     const handleClose = vi.fn();

     render(
       <div>
         <WalkthroughTarget id="test1"><div>Target 1</div></WalkthroughTarget>
         <InteractiveWalkthrough
           steps={[{ targetId: 'test1', title: 'Step 1', content: 'Content 1' }]}
           isOpen={true}
           onClose={handleClose}
         />
       </div>
     );

     act(() => {
       vi.advanceTimersByTime(350);
     });

     const skipButton = document.querySelector('button.text-gray-500'); // Close icon

     act(() => {
       fireEvent.click(skipButton!);
     });

     expect(handleClose).toHaveBeenCalled();
  });

  it('handles missing target gracefully', async () => {
      const { container } = render(
        <InteractiveWalkthrough
          steps={[{ targetId: 'missing-target', title: 'Step 1', content: 'Content 1' }]}
          isOpen={true}
          onClose={() => {}}
        />
      );

      act(() => {
          vi.advanceTimersByTime(350);
      });

      // Should not render bubble if target is missing
      expect(screen.queryByText('Step 1')).not.toBeInTheDocument();
  });
});
