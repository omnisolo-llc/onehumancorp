import React from 'react';
import { render, screen } from '@testing-library/react';
import { Footer } from './Footer';

describe('Footer', () => {
  it('renders correctly with default props', () => {
    render(<Footer />);
    const link = screen.getByRole('link', { name: /powered by ohc/i });
    expect(link).toBeInTheDocument();
    expect(link).toHaveAttribute('href', 'https://ohc.store/join');
  });

  it('includes tenantId in the referral link if provided', () => {
    render(<Footer tenantId="test-tenant-123" />);
    const link = screen.getByRole('link', { name: /powered by ohc/i });
    expect(link).toHaveAttribute('href', 'https://ohc.store/join?ref=test-tenant-123');
  });

  it('applies correct styling for dark theme', () => {
    render(<Footer theme="dark" />);
    const link = screen.getByRole('link', { name: /powered by ohc/i });
    expect(link).toHaveClass('text-gray-400');
    expect(link).toHaveClass('hover:text-white');
  });

  it('applies correct styling for gradient theme', () => {
    render(<Footer theme="gradient" />);
    const link = screen.getByRole('link', { name: /powered by ohc/i });
    expect(link).toHaveClass('text-white/80');
    expect(link).toHaveClass('hover:text-white');
  });
});
