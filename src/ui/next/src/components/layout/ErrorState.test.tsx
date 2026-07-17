import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { ErrorState } from './ErrorState';
import '@testing-library/jest-dom';

describe('ErrorState', () => {
  it('renders correctly with title and message', () => {
    render(<ErrorState title="Error Title" message="Something went wrong." />);
    expect(screen.getByText('Error Title')).toBeInTheDocument();
    expect(screen.getByText('Something went wrong.')).toBeInTheDocument();
  });

  it('renders correctly with message only', () => {
    render(<ErrorState message="Only a message." />);
    expect(screen.queryByRole('heading')).not.toBeInTheDocument();
    expect(screen.getByText('Only a message.')).toBeInTheDocument();
  });

  it('contains proper translucent classes', () => {
    const { container } = render(<ErrorState message="Check styles" />);
    const divElement = container.firstChild as HTMLElement;
    expect(divElement).toHaveClass('backdrop-blur-[30px]');
    expect(divElement).toHaveClass('backdrop-saturate-[210%]');
    expect(divElement).toHaveClass('bg-[rgba(255,255,255,0.65)]');
    expect(divElement).toHaveClass('dark:bg-[rgba(22,22,26,0.7)]');
    expect(divElement).toHaveClass('border');
    expect(divElement).toHaveClass('border-[rgba(255,255,255,0.4)]');
    expect(divElement).toHaveClass('dark:border-[rgba(255,255,255,0.1)]');
  });
});
