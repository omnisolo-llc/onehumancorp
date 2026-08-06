import '@testing-library/jest-dom';
import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import HelpLayout from './layout';

describe('HelpLayout', () => {
  it('renders children and applies premium layout classes', () => {
    const { container } = render(
      <HelpLayout>
        <div data-testid="dummy-content">Dummy Content</div>
      </HelpLayout>
    );

    // Verify children are rendered
    expect(screen.getByTestId('dummy-content')).toBeInTheDocument();

    // Verify layout class
    const layoutWrapper = container.firstChild as HTMLElement;
    expect(layoutWrapper).toHaveClass('help-layout');
    expect(layoutWrapper).toHaveClass('bg-gradient-to-b');
    expect(layoutWrapper).toHaveClass('min-h-screen');
  });
});
