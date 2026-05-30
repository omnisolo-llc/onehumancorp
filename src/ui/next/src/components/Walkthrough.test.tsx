import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { InteractiveWalkthrough } from './Walkthrough';
import { describe, it, expect, vi, beforeEach } from 'vitest';

describe('Walkthrough Component', () => {
  beforeEach(() => {
    vi.clearAllMocks();
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

  it('renders correctly and handles next and skip', async () => {
    const mockScrollIntoView = vi.fn();
    const mockGetBoundingClientRect = vi.fn(() => ({
      width: 100, height: 50, top: 10, left: 10, bottom: 60, right: 110, x: 10, y: 10, toJSON: () => {}
    }));

    const mockElement = document.createElement('div');
    mockElement.scrollIntoView = mockScrollIntoView;
    mockElement.getBoundingClientRect = mockGetBoundingClientRect;

    vi.spyOn(document, 'getElementById').mockReturnValue(mockElement);

    const onClose = vi.fn();
    const onComplete = vi.fn();

    const steps = [
      { targetId: 'step1', title: 'Step 1 Title', content: 'Step 1 Content', position: 'bottom' as const },
      { targetId: 'step2', title: 'Step 2 Title', content: 'Step 2 Content', position: 'top' as const },
    ];

    render(
      <InteractiveWalkthrough
        steps={steps}
        isOpen={true}
        onClose={onClose}
        onComplete={onComplete}
      />
    );

    await waitFor(() => {
      expect(screen.getByText('Step 1 Title')).toBeInTheDocument();
    });

    expect(screen.getByText('Step 1 Content')).toBeInTheDocument();
    expect(screen.getByText('Step 1 of 2')).toBeInTheDocument();

    fireEvent.click(screen.getByText('Next'));

    await waitFor(() => {
      expect(screen.getByText('Step 2 Title')).toBeInTheDocument();
    });

    expect(screen.getByText('Step 2 Content')).toBeInTheDocument();
    expect(screen.getByText('Step 2 of 2')).toBeInTheDocument();
    expect(screen.getByText('Finish')).toBeInTheDocument();

    fireEvent.click(screen.getByText('Finish'));

    expect(onComplete).toHaveBeenCalled();
    expect(onClose).toHaveBeenCalled();
  });

  it('handles skip button', async () => {
    const mockScrollIntoView = vi.fn();
    const mockGetBoundingClientRect = vi.fn(() => ({
      width: 100, height: 50, top: 10, left: 10, bottom: 60, right: 110, x: 10, y: 10, toJSON: () => {}
    }));

    const mockElement = document.createElement('div');
    mockElement.scrollIntoView = mockScrollIntoView;
    mockElement.getBoundingClientRect = mockGetBoundingClientRect;

    vi.spyOn(document, 'getElementById').mockReturnValue(mockElement);

    const onClose = vi.fn();

    render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'step1', title: 'Step 1', content: 'Content 1' }]}
        isOpen={true}
        onClose={onClose}
      />
    );

    await waitFor(() => {
      expect(screen.getByText('Step 1')).toBeInTheDocument();
    });

    // Find the skip button by searching all buttons and finding the one without text
    const allButtons = screen.getAllByRole('button');
    const closeBtn = allButtons.find(b => !b.textContent || (!b.textContent.includes('Finish') && !b.textContent.includes('Next')));

    if (closeBtn) {
        fireEvent.click(closeBtn);
    }

    expect(onClose).toHaveBeenCalled();
  });
});
