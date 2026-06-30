/** @jsxImportSource react */
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { GrowBusinessCard } from './GrowBusinessCard';
import React from 'react';

describe('GrowBusinessCard', () => {
  it('renders correctly and has all required links', () => {
    const { container } = render(<GrowBusinessCard />);

    expect(screen.getByText('Grow Business')).toBeInTheDocument();

    // Check links
    const promoterLink = screen.getByText('Promoter Agent');
    expect(promoterLink).toHaveAttribute('href', '/viral-post-generator');

    const widgetLink = screen.getByText('Viral Widget');
    expect(widgetLink).toHaveAttribute('href', '/viral-powered-by-ohc-widget');

    const storefrontLink = screen.getByText('Review Storefront');
    expect(storefrontLink).toHaveAttribute('href', '/edge-storefront-setup');

    // Visual styles check - transulcent glassmorphism class check
    const wrapper = container.firstChild as HTMLElement;
    expect(wrapper).toHaveClass('glassmorphism');
    expect(wrapper).toHaveClass('bg-white');
  });
});
