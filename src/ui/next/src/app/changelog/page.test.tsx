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
});
