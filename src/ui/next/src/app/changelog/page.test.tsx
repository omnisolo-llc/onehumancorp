import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import '@testing-library/jest-dom';
import ChangelogPage from './page';

describe('ChangelogPage', () => {
  it('renders the changelog title and sections', () => {
    render(<ChangelogPage />);

    expect(screen.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeInTheDocument();
    expect(screen.getByText('Version 1.0 (Latest)')).toBeInTheDocument();
    expect(screen.getByText('🌟 New Features')).toBeInTheDocument();
    expect(screen.getByText('🛠️ Improvements')).toBeInTheDocument();
  });

  it('renders a link to the full changelog', () => {
     render(<ChangelogPage />);
     const link = screen.getByRole('link', { name: /Read the full technical changelog/i });
     expect(link).toBeInTheDocument();
     expect(link).toHaveAttribute('href', 'https://onehumancorp.com/changelog');
  });
});
