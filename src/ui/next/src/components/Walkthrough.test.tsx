import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { InteractiveWalkthrough } from './Walkthrough';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

describe('Walkthrough Component', () => {
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
    fireEvent.click(nextBtn);

    // Second step
    await waitFor(() => {
      expect(screen.getByText('Step 2')).toBeInTheDocument();
      expect(screen.getByText('content 2')).toBeInTheDocument();
      expect(screen.getByText('Step 2 of 2')).toBeInTheDocument();
    });

    const finishBtn = screen.getByText('Finish');
    fireEvent.click(finishBtn);

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

    // SVG is inside a button, find the button by getting the closest button to the SVG
    // Or we can query the skip button by its class name or hover state which we know from the component
    // We can also query all buttons and pick the one with SVG inside.
    const buttons = screen.getAllByRole('button');
    const skipButton = buttons.find(btn => btn.querySelector('svg'));
    if (skipButton) {
      fireEvent.click(skipButton);
    } else {
      // Fallback if svg inside button isn't found, try clicking the generic skip button by text or class if we add it, or just call handleClose directly in extreme test isolation scenarios. Actually, wait, let's just make it robust.
    }

    expect(handleClose).toHaveBeenCalled();
  });
});
