import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import '@testing-library/jest-dom';
import { PromoterCard } from './PromoterCard';

describe('PromoterCard', () => {
  it('renders correctly with title and description', () => {
    render(<PromoterCard />);

    expect(screen.getByText('The Promoter Agent')).toBeInTheDocument();
    expect(screen.getByText(/Let OHC's AI write engaging social media posts/)).toBeInTheDocument();
  });

  it('applies the correct Translucent Glass CSS classes to the container', () => {
    render(<PromoterCard />);

    // Find the main container by finding a unique element and going up
    const title = screen.getByText('The Promoter Agent');
    const container = title.closest('div.rounded-\\[24px\\]');

    expect(container).toBeInTheDocument();
    expect(container).toHaveClass('bg-[rgba(255,255,255,0.65)]');
    expect(container).toHaveClass('backdrop-blur-[30px]');
    expect(container).toHaveClass('saturate-[210%]');
    expect(container).toHaveClass('border-[rgba(255,255,255,0.4)]');
    expect(container).toHaveClass('dark:bg-[rgba(22,22,26,0.7)]');
    expect(container).not.toHaveClass('bg-white'); // ensure opaque bg is gone
  });

  it('applies dark mode typography classes', () => {
    render(<PromoterCard />);

    const title = screen.getByText('The Promoter Agent');
    expect(title).toHaveClass('text-[#1D1D1F]');
    expect(title).toHaveClass('dark:text-[#F5F5F7]');

    const desc = screen.getByText(/Let OHC's AI write engaging social media posts/);
    expect(desc).toHaveClass('dark:text-gray-300');
  });

  it('renders a valid link pointing to /promoter', () => {
    render(<PromoterCard />);

    const link = screen.getByRole('link', { name: 'Create Posts' });
    expect(link).toBeInTheDocument();
    expect(link).toHaveAttribute('href', '/promoter');
  });

  it('includes interactive and visual elements for accessibility', () => {
    render(<PromoterCard />);

    const link = screen.getByRole('link', { name: 'Create Posts' });
    expect(link).toHaveClass('hover:bg-indigo-700');
    expect(link).toHaveClass('transition-colors');

    const container = screen.getByText('The Promoter Agent').closest('div.rounded-\\[24px\\]');
    expect(container).toHaveClass('shadow-sm');
  });
});
