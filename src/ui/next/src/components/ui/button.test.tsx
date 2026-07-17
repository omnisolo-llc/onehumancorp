import React from 'react';
import { render, screen } from '@testing-library/react';
import { Button } from './button';

describe('Button Component', () => {
  it('renders default correctly', () => {
    const { getByText } = render(<Button>Click me</Button>);
    expect(getByText('Click me')).toBeInTheDocument();
  });

  it('renders different variants correctly', () => {
    render(<Button variant="destructive">Destructive</Button>);
    expect(screen.getByText('Destructive')).toHaveClass('bg-red-500 text-white hover:bg-red-600');

    render(<Button variant="outline">Outline</Button>);
    expect(screen.getByText('Outline')).toHaveClass('border border-input bg-background hover:bg-accent hover:text-accent-foreground');

    render(<Button variant="secondary">Secondary</Button>);
    expect(screen.getByText('Secondary')).toHaveClass('bg-secondary text-secondary-foreground hover:bg-secondary/80');

    render(<Button variant="ghost">Ghost</Button>);
    expect(screen.getByText('Ghost')).toHaveClass('hover:bg-accent hover:text-accent-foreground');

    render(<Button variant="link">Link</Button>);
    expect(screen.getByText('Link')).toHaveClass('text-primary underline-offset-4 hover:underline');
  });

  it('renders different sizes correctly', () => {
    render(<Button size="sm">Small</Button>);
    expect(screen.getByText('Small')).toHaveClass('h-9 px-3');

    render(<Button size="lg">Large</Button>);
    expect(screen.getByText('Large')).toHaveClass('h-11 px-8');

    render(<Button size="icon">Icon</Button>);
    expect(screen.getByText('Icon')).toHaveClass('h-10 w-10');
  });

  it('applies custom className', () => {
    render(<Button className="custom-class">Custom</Button>);
    expect(screen.getByText('Custom')).toHaveClass('custom-class');
  });
});
