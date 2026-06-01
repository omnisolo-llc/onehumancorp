import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import ChangelogPage from './page';

describe('ChangelogPage', () => {
  it('renders the release notes page correctly', () => {
    render(<ChangelogPage />);

    expect(screen.getByText('Release Notes & Changelog')).toBeTruthy();
    expect(screen.getByText('Version 1.0 (Latest)')).toBeTruthy();

    // Check for some content points
    expect(screen.getByText(/Interactive AI Store Builder:/)).toBeTruthy();
    expect(screen.getByText(/Smart Tooltips:/)).toBeTruthy();
  });
});
