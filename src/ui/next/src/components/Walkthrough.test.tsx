import '@testing-library/jest-dom';
import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { InteractiveWalkthrough, WalkthroughTarget } from './Walkthrough';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

describe('Walkthrough Component', () => {
  let mockGetElementById: any;

  beforeEach(() => {
    mockGetElementById = vi.spyOn(document, 'getElementById').mockImplementation((id) => {
      if (id.startsWith('test-target')) {
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
          { targetId: 'test-target', title: 'Step 1', content: 'content 1', position: 'top' },
          { targetId: 'test-target', title: 'Step 2', content: 'content 2', position: 'bottom' },
          { targetId: 'test-target', title: 'Step 3', content: 'content 3', position: 'left' },
          { targetId: 'test-target', title: 'Step 4', content: 'content 4', position: 'right' }
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
      expect(screen.getByText('Step 1 of 4')).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText('Next'));

    // Second step
    await waitFor(() => {
      expect(screen.getByText('Step 2')).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText('Next'));

    // Third step
    await waitFor(() => {
      expect(screen.getByText('Step 3')).toBeInTheDocument();
    });

    fireEvent.click(screen.getByText('Next'));

    // Fourth step
    await waitFor(() => {
      expect(screen.getByText('Step 4')).toBeInTheDocument();
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

    const buttons = screen.getAllByRole('button');
    const skipButton = buttons.find(btn => btn.querySelector('svg'));
    if (skipButton) {
      fireEvent.click(skipButton);
    }

    expect(handleClose).toHaveBeenCalled();
  });

  it('handles missing target gracefully', async () => {
    const consoleWarnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});
    const { container } = render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'missing-target', title: 'Missing', content: 'Missing' }]}
        isOpen={true}
        onClose={() => {}}
      />
    );

    await waitFor(() => {
      expect(consoleWarnSpy).toHaveBeenCalledWith('Walkthrough: Target element with id "missing-target" not found.');
    });

    expect(container.firstChild).toBeNull();
    consoleWarnSpy.mockRestore();
  });

  it('WalkthroughTarget renders correctly', () => {
     // disable document getElementById mock for this test so render can append directly
     mockGetElementById.mockRestore();
     const { container } = render(<WalkthroughTarget id="target1" className="test-class"><div>content</div></WalkthroughTarget>);
     const el = container.querySelector('#target1');
     expect(el).toBeInTheDocument();
     expect(el).toHaveClass('test-class');
     expect(el).toHaveTextContent('content');
  });
});
