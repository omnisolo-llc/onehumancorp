import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import ChangelogPage from './page';

describe('ChangelogPage', () => {
  it('renders changelog fallback correctly', () => {
    render(<ChangelogPage />);
    expect(screen.getByText(/Release Notes & Changelog/i)).toBeInTheDocument();
  });
});
