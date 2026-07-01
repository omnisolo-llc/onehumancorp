import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { ErrorState } from './ErrorState';

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
    expect(divElement).toHaveClass('saturate-[210%]');
    expect(divElement).toHaveClass('bg-white/65');
  });
});
