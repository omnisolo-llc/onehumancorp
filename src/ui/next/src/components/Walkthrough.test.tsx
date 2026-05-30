import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { InteractiveWalkthrough } from './Walkthrough';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

describe('Walkthrough Component', () => {
  beforeEach(() => {
    vi.useFakeTimers();
    // Mock getBoundingClientRect
    Element.prototype.getBoundingClientRect = vi.fn(() => ({
      width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));
    // Mock scrollIntoView
    Element.prototype.scrollIntoView = vi.fn();
  });

  afterEach(() => {
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

  it('renders steps and handles "Next" and "Finish" actions', async () => {
    // Create a mock target element in the DOM
    const targetDiv = document.createElement('div');
    targetDiv.id = 'target-1';
    document.body.appendChild(targetDiv);

    const targetDiv2 = document.createElement('div');
    targetDiv2.id = 'target-2';
    document.body.appendChild(targetDiv2);

    const handleClose = vi.fn();
    const handleComplete = vi.fn();

    render(
      <InteractiveWalkthrough
        steps={[
          { targetId: 'target-1', title: 'Step 1', content: 'Content 1' },
          { targetId: 'target-2', title: 'Step 2', content: 'Content 2' }
        ]}
        isOpen={true}
        onClose={handleClose}
        onComplete={handleComplete}
      />
    );

    // Use act to wrap runAllTimers
    act(() => {
      vi.advanceTimersByTime(400); // 300ms timeout + buffer
    });

    expect(screen.getByText('Step 1')).toBeInTheDocument();
    expect(screen.getByText('Content 1')).toBeInTheDocument();
    expect(screen.getByText('Next')).toBeInTheDocument();

    act(() => {
      fireEvent.click(screen.getByText('Next'));
    });

    // Should now show step 2
    act(() => {
      vi.advanceTimersByTime(400);
    });

    expect(screen.getByText('Step 2')).toBeInTheDocument();
    expect(screen.getByText('Content 2')).toBeInTheDocument();
    expect(screen.getByText('Finish')).toBeInTheDocument();

    act(() => {
      fireEvent.click(screen.getByText('Finish'));
    });

    expect(handleComplete).toHaveBeenCalled();
    expect(handleClose).toHaveBeenCalled();

    // Cleanup DOM
    document.body.removeChild(targetDiv);
    document.body.removeChild(targetDiv2);
  });

  it('calls onClose when "Skip" button is clicked', async () => {
    const targetDiv = document.createElement('div');
    targetDiv.id = 'target-skip';
    document.body.appendChild(targetDiv);

    const handleClose = vi.fn();

    render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'target-skip', title: 'Step Skip', content: 'Content Skip' }]}
        isOpen={true}
        onClose={handleClose}
      />
    );

    act(() => {
      vi.advanceTimersByTime(400);
    });

    expect(screen.getByText('Step Skip')).toBeInTheDocument();

    const buttons = screen.getAllByRole('button');
    const closeBtn = buttons.find(b => b.className.includes('text-gray-500'));

    if (closeBtn) {
       act(() => {
         fireEvent.click(closeBtn);
       });
    }

    expect(handleClose).toHaveBeenCalled();

    document.body.removeChild(targetDiv);
  });
});
