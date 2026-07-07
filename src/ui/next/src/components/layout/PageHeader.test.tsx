import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { PageHeader } from './PageHeader';
import '@testing-library/jest-dom';

describe('PageHeader', () => {
  it('renders correctly with title and description', () => {
    render(<PageHeader title="My Page" description="Page description here" />);
    expect(screen.getByText('My Page')).toBeInTheDocument();
    expect(screen.getByText('Page description here')).toBeInTheDocument();
  });

  it('renders correctly with title only', () => {
    render(<PageHeader title="Title Only" />);
    expect(screen.getByText('Title Only')).toBeInTheDocument();
    // Verify no paragraph element is rendered for description
    expect(screen.queryByText(/Page description/)).not.toBeInTheDocument();
  });

  it('contains proper translucent classes', () => {
    const { container } = render(<PageHeader title="Style Check" />);
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
