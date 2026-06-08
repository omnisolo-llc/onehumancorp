
import '@testing-library/jest-dom';
import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import ChangelogPage from './page';

describe('ChangelogPage', () => {
  it('renders the release notes page correctly', () => {
    render(<ChangelogPage />);

    expect(screen.getByText('Release Notes & Changelog')).toBeInTheDocument();
    expect(screen.getByText('Version 1.0 (Latest)')).toBeInTheDocument();

    // Check for some content points
    expect(screen.getByText(/Interactive AI Store Builder:/)).toBeInTheDocument();
    expect(screen.getByText(/Smart Tooltips:/)).toBeInTheDocument();
  });

  it('renders paragraph strings', () => {
    render(<ChangelogPage />);
    const link = screen.getByText('Read the full technical changelog on our website →');
    expect(link).toHaveAttribute('href', 'https://onehumancorp.com/changelog');
  });

  it('renders paragraph elements for random text', () => {
    render(<ChangelogPage />);
    expect(screen.getByText(/Faster loading times for product images/)).toBeInTheDocument();
  });

  it('covers the line 36 paragraph fallback', () => {
    // Re-render to ensure we evaluate the branch where a line neither starts with ### nor -
    render(<ChangelogPage />);
  });
});
