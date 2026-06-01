import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import ChangelogPage from './page';

describe('ChangelogPage', () => {
  it('renders the changelog title', () => {
    render(<ChangelogPage />);
    expect(screen.getByText('Release Notes & Changelog')).toBeInTheDocument();
  });

  it('renders the latest version header', () => {
    render(<ChangelogPage />);
    expect(screen.getByText('Version 1.0 (Latest)')).toBeInTheDocument();
  });

  it('renders the full changelog link', () => {
    render(<ChangelogPage />);
    expect(screen.getByText(/Read the full technical changelog/)).toBeInTheDocument();
  });
});
