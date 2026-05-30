import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import ChangelogPage from './page';
import { describe, it, expect } from 'vitest';

describe('ChangelogPage', () => {
  it('renders the changelog title and sections', () => {
    render(<ChangelogPage />);
    expect(screen.getByText('Release Notes & Changelog')).toBeInTheDocument();
    expect(screen.getByText('Version 1.0 (Latest)')).toBeInTheDocument();
  });

  it('renders different content line types (headers, bullets, paragraphs)', () => {
    render(<ChangelogPage />);
    expect(screen.getByText('🌟 New Features')).toBeInTheDocument();
    // Use getByText with a partial match or string since there are multiple bullets
    expect(screen.getByText((content) => content.includes('Interactive AI Store Builder:'))).toBeInTheDocument();
    expect(screen.getByText('Faster loading times for product images.')).toBeInTheDocument();
  });

  it('renders the external link', () => {
    render(<ChangelogPage />);
    const link = screen.getByRole('link', { name: /Read the full technical changelog/i });
    expect(link).toHaveAttribute('href', 'https://onehumancorp.com/changelog');
  });
});
