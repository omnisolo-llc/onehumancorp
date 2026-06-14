import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import { PageHeader } from './PageHeader';

describe('PageHeader Component (macOS Translucent Glass)', () => {
  it('renders the title and description correctly', () => {
    render(<PageHeader title="Test Title" description="Test Description" />);
    expect(screen.getByText('Test Title')).toBeDefined();
    expect(screen.getByText('Test Description')).toBeDefined();
  });

  it('applies the macOS Translucent Glass styling', () => {
    const { container } = render(<PageHeader title="Test Title" />);
    const headerDiv = container.querySelector('div');

    expect(headerDiv?.className).toContain('backdrop-blur-[30px]');
    expect(headerDiv?.className).toContain('backdrop-saturate-[2.1]');
    expect(headerDiv?.className).toContain('bg-white/65');
    expect(headerDiv?.className).toContain('dark:bg-[#16161a]/70');
    expect(headerDiv?.className).toContain('border-white/40');
    expect(headerDiv?.className).toContain('dark:border-white/10');
  });
});
