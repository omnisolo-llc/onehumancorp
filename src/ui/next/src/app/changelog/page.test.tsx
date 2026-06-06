import '@testing-library/jest-dom';
import React from 'react';
import { render, screen } from '@testing-library/react';
import ChangelogPage from './page';
import { describe, it, expect } from 'vitest';

describe('ChangelogPage', () => {
  it('renders correctly', () => {
    render(<ChangelogPage />);
    expect(screen.getByText('Release Notes & Changelog')).toBeInTheDocument();
    expect(screen.getByText('Version 1.0 (Latest)')).toBeInTheDocument();
    expect(screen.getByText('🌟 New Features')).toBeInTheDocument();
  });
});
