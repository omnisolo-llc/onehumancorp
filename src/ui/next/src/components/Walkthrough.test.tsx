import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { InteractiveWalkthrough } from './Walkthrough';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

describe('Walkthrough Component', () => {
  beforeEach(() => {
    vi.useFakeTimers();
    vi.spyOn(document, 'getElementById').mockImplementation((id) => {
      if (id === 'test-target') {
        return {
          scrollIntoView: vi.fn(),
          getBoundingClientRect: () => ({
            top: 100,
            left: 100,
            bottom: 200,
            right: 200,
            width: 100,
            height: 100,
          }),
        } as any;
      }
      return null;
    });
  });

  afterEach(() => {
    vi.useRealTimers();
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
      <InteractiveWalkthrough
        steps={[{ targetId: 'test-target', title: 'Test Title', content: 'test content' }]}
        isOpen={true}
        onClose={() => {}}
      />
    );

    act(() => {
      vi.advanceTimersByTime(300);
    });

    expect(screen.getByText('Test Title')).toBeInTheDocument();
    expect(screen.getByText('test content')).toBeInTheDocument();
  });

  it('handles next step and close', async () => {
    const onClose = vi.fn();
    const onComplete = vi.fn();

    render(
      <InteractiveWalkthrough
        steps={[
          { targetId: 'test-target', title: 'Step 1', content: 'Content 1' },
          { targetId: 'test-target', title: 'Step 2', content: 'Content 2' }
        ]}
        isOpen={true}
        onClose={onClose}
        onComplete={onComplete}
      />
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

    const finishBtn = screen.getByText('Finish');
    fireEvent.click(finishBtn);

    expect(onComplete).toHaveBeenCalled();
    expect(onClose).toHaveBeenCalled();
  });

  it('handles skip', async () => {
    const onClose = vi.fn();
    render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'test-target', title: 'Step 1', content: 'Content 1' }]}
        isOpen={true}
        onClose={onClose}
      />
    );

    act(() => {
      vi.advanceTimersByTime(300);
    });

    const skipBtn = screen.getByRole('button', { name: '' }); // the close 'X' button
    fireEvent.click(skipBtn);

    expect(onClose).toHaveBeenCalled();
  });

});
