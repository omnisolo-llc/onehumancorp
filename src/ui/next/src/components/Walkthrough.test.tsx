import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { InteractiveWalkthrough } from './Walkthrough';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';

describe('Walkthrough Component', () => {
  beforeEach(() => {
    // Mock getBoundingClientRect
    window.HTMLElement.prototype.getBoundingClientRect = function() {
      return {
        width: 100,
        height: 100,
        top: 0,
        left: 0,
        bottom: 100,
        right: 100,
        x: 0,
        y: 0,
        toJSON: () => {}
      };
    };
    // Mock scrollIntoView
    window.HTMLElement.prototype.scrollIntoView = vi.fn();
  });

  afterEach(() => {
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

  it('renders walkthrough step when open and target element exists', async () => {
    // Create target element
    const targetElement = document.createElement('div');
    targetElement.id = 'test-target';
    document.body.appendChild(targetElement);

    render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'test-target', title: 'Test Title', content: 'test content' }]}
        isOpen={true}
        onClose={() => {}}
      />
    );

    await waitFor(() => {
      expect(screen.getByText('Test Title')).toBeDefined();
      expect(screen.getByText('test content')).toBeDefined();
    });
  });

  it('handles Next and Finish actions correctly', async () => {
    const targetElement1 = document.createElement('div');
    targetElement1.id = 'target-1';
    document.body.appendChild(targetElement1);

    const targetElement2 = document.createElement('div');
    targetElement2.id = 'target-2';
    document.body.appendChild(targetElement2);

    const onComplete = vi.fn();
    const onClose = vi.fn();

    render(
      <InteractiveWalkthrough
        steps={[
          { targetId: 'target-1', title: 'Step 1', content: 'Content 1' },
          { targetId: 'target-2', title: 'Step 2', content: 'Content 2' }
        ]}
        isOpen={true}
        onClose={onClose}
        onComplete={onComplete}
      />
    );

    // Wait for step 1 to render
    await waitFor(() => {
      expect(screen.getByText('Step 1')).toBeDefined();
    });

    // Click next
    const nextButton = screen.getByText('Next');
    fireEvent.click(nextButton);

    // Wait for step 2 to render
    await waitFor(() => {
      expect(screen.getByText('Step 2')).toBeDefined();
    });

    // Click finish
    const finishButton = screen.getByText('Finish');
    fireEvent.click(finishButton);

    expect(onComplete).toHaveBeenCalled();
    expect(onClose).toHaveBeenCalled();
  });

  it('handles Skip action correctly', async () => {
    const targetElement = document.createElement('div');
    targetElement.id = 'test-target';
    document.body.appendChild(targetElement);

    const onClose = vi.fn();

    render(
      <InteractiveWalkthrough
        steps={[{ targetId: 'test-target', title: 'Test Title', content: 'test content' }]}
        isOpen={true}
        onClose={onClose}
      />
    );

    await waitFor(() => {
      expect(screen.getByText('Test Title')).toBeDefined();
    });

    // Assuming the close button is the SVG inside a button tag
    // An easy way to find it is to use the generic SVG query if there's only one
    // But since testing library doesn't easily find SVGs, we'll click the button containing it
    const closeButtons = screen.getAllByRole('button');
    // In our component, Skip is the button with the X icon, which is the first button in the div
    const skipButton = closeButtons[0];
    fireEvent.click(skipButton);

    expect(onClose).toHaveBeenCalled();
  });
});