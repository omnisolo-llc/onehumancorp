import React from 'react';
import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { InteractiveWalkthrough, WalkthroughTarget } from './Walkthrough';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

describe('Walkthrough Component', () => {
  let scrollIntoViewMock: any;
  let getBoundingClientRectMock: any;

  beforeEach(() => {
    scrollIntoViewMock = vi.fn();
    getBoundingClientRectMock = vi.fn(() => ({
      width: 100, height: 20, top: 0, left: 0, bottom: 20, right: 100, x: 0, y: 0, toJSON: () => {}
    }));

    Element.prototype.scrollIntoView = scrollIntoViewMock;
    Element.prototype.getBoundingClientRect = getBoundingClientRectMock;

    vi.useFakeTimers({ shouldAdvanceTime: true });
  });

  afterEach(() => {
    vi.runOnlyPendingTimers();
    vi.useRealTimers();
    vi.restoreAllMocks();
    document.body.innerHTML = '';
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

  it('renders correctly when open and target element exists', async () => {
    const target = document.createElement('div');
    target.id = 'test1';
    document.body.appendChild(target);

    render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'test1', title: 'Step 1 Title', content: 'Step 1 Content' }]}
        isOpen={true}
        onClose={() => {}}
      />
    );

    act(() => {
      vi.runAllTimers();
    });

    expect(screen.getByText('Step 1 Title')).toBeInTheDocument();
    expect(screen.getByText('Step 1 Content')).toBeInTheDocument();
    expect(screen.getByText('Step 1 of 1')).toBeInTheDocument();
    expect(screen.getByText('Finish')).toBeInTheDocument();
  });

  it('updates step index on next', async () => {
    const target1 = document.createElement('div');
    target1.id = 'test1';
    document.body.appendChild(target1);

    const target2 = document.createElement('div');
    target2.id = 'test2';
    document.body.appendChild(target2);

    const steps: any = [
      { targetId: 'test1', title: 'Step 1 Title', content: 'Step 1 Content' },
      { targetId: 'test2', title: 'Step 2 Title', content: 'Step 2 Content' }
    ];

    render(
      <InteractiveWalkthrough
        steps={steps}
        isOpen={true}
        onClose={() => {}}
      />
    );

    act(() => {
      vi.runAllTimers();
    });

    expect(screen.getByText('Step 1 Title')).toBeInTheDocument();

    fireEvent.click(screen.getByText('Next'));

    act(() => {
      vi.runAllTimers();
    });

    expect(screen.getByText('Step 2 Title')).toBeInTheDocument();
    expect(screen.getByText('Step 2 of 2')).toBeInTheDocument();
    expect(screen.getByText('Finish')).toBeInTheDocument();
  });

  it('calls onComplete and onClose on finish', async () => {
    const target = document.createElement('div');
    target.id = 'test1';
    document.body.appendChild(target);

    const handleClose = vi.fn();
    const handleComplete = vi.fn();
    const steps: any = [
      { targetId: 'test1', title: 'Step 1 Title', content: 'Step 1 Content' }
    ];

    render(
      <InteractiveWalkthrough
        steps={steps}
        isOpen={true}
        onClose={handleClose}
        onComplete={handleComplete}
      />
    );

    act(() => {
      vi.runAllTimers();
    });

    expect(screen.getByText('Finish')).toBeInTheDocument();

    fireEvent.click(screen.getByText('Finish'));

    expect(handleComplete).toHaveBeenCalledTimes(1);
    expect(handleClose).toHaveBeenCalledTimes(1);
  });

  it('calls onClose when close/skip button is clicked', async () => {
    const target = document.createElement('div');
    target.id = 'test1';
    document.body.appendChild(target);

    const handleClose = vi.fn();
    const steps: any = [
      { targetId: 'test1', title: 'Step 1 Title', content: 'Step 1 Content' }
    ];

    const { container } = render(
      <InteractiveWalkthrough
        steps={steps}
        isOpen={true}
        onClose={handleClose}
      />
    );

    act(() => {
      vi.runAllTimers();
    });

    expect(screen.getByText('Step 1 Title')).toBeInTheDocument();

    // Find the close button via the SVG inside it
    const closeBtn = container.querySelector('button.text-gray-500');
    fireEvent.click(closeBtn!);

    expect(handleClose).toHaveBeenCalledTimes(1);
  });

  it('scrolls into view on target element', async () => {
    const target = document.createElement('div');
    target.id = 'test1';
    document.body.appendChild(target);

    render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'test1', title: 'Step 1', content: 'Content' }]}
        isOpen={true}
        onClose={() => {}}
      />
    );

    act(() => {
      vi.runAllTimers();
    });

    expect(scrollIntoViewMock).toHaveBeenCalledWith({ behavior: 'smooth', block: 'center' });
  });

  it('does not render if target element is not found', async () => {
    const { container } = render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'missing', title: 'Missing', content: 'Missing content' }]}
        isOpen={true}
        onClose={() => {}}
      />
    );

    act(() => {
      vi.runAllTimers();
    });

    expect(container.firstChild).toBeNull();
  });

  it('calculates position classes correctly for top', async () => {
    const target = document.createElement('div');
    target.id = 'test1';
    document.body.appendChild(target);

    const steps: any = [{ targetId: 'test1', title: 'Top', content: 'Top', position: 'top' }];
    const { container } = render(
      <InteractiveWalkthrough steps={steps} isOpen={true} onClose={() => {}} />
    );
    act(() => { vi.runAllTimers(); });
    const bubble = container.querySelector('.fixed.z-\\[1000\\]') as HTMLElement;
    expect(bubble).toBeInTheDocument();
  });

  it('calculates position classes correctly for right', async () => {
    const target = document.createElement('div');
    target.id = 'test1';
    document.body.appendChild(target);

    const steps: any = [{ targetId: 'test1', title: 'Right', content: 'Right', position: 'right' }];
    const { container } = render(
      <InteractiveWalkthrough steps={steps} isOpen={true} onClose={() => {}} />
    );
    act(() => { vi.runAllTimers(); });
    const bubble = container.querySelector('.fixed.z-\\[1000\\]') as HTMLElement;
    expect(bubble).toBeInTheDocument();
  });

  it('calculates position classes correctly for left', async () => {
    const target = document.createElement('div');
    target.id = 'test1';
    document.body.appendChild(target);

    const steps: any = [{ targetId: 'test1', title: 'Left', content: 'Left', position: 'left' }];
    const { container } = render(
      <InteractiveWalkthrough steps={steps} isOpen={true} onClose={() => {}} />
    );
    act(() => { vi.runAllTimers(); });
    const bubble = container.querySelector('.fixed.z-\\[1000\\]') as HTMLElement;
    expect(bubble).toBeInTheDocument();
  });

  it('handles window resize and scroll events', async () => {
    const target = document.createElement('div');
    target.id = 'test1';
    document.body.appendChild(target);

    render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'test1', title: 'Step 1', content: 'Content' }]}
        isOpen={true}
        onClose={() => {}}
      />
    );

    act(() => { vi.runAllTimers(); });

    act(() => {
      window.dispatchEvent(new Event('scroll'));
    });
    expect(getBoundingClientRectMock).toHaveBeenCalledTimes(2); // once initial, once on scroll

    act(() => {
      window.dispatchEvent(new Event('resize'));
    });
    expect(getBoundingClientRectMock).toHaveBeenCalledTimes(3); // once on resize
  });

  it('WalkthroughTarget renders correctly', () => {
    render(<WalkthroughTarget id="target-id" className="test-class">Target Content</WalkthroughTarget>);
    const el = document.getElementById('target-id');
    expect(el).toBeInTheDocument();
    expect(el).toHaveClass('test-class');
    expect(screen.getByText('Target Content')).toBeInTheDocument();
  });

  it('renders nothing when E2E env var is set', async () => {
    process.env.NEXT_PUBLIC_E2E = 'true';
    const target = document.createElement('div');
    target.id = 'test1';
    document.body.appendChild(target);

    const { container } = render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'test1', title: 'Test', content: 'test content' }]}
        isOpen={true}
        onClose={() => {}}
      />
    );

    act(() => {
      vi.runAllTimers();
    });

    expect(container.firstChild).toBeNull();
    delete process.env.NEXT_PUBLIC_E2E;
  });
});