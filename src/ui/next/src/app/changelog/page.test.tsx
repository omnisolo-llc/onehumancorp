import '@testing-library/jest-dom';
import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import ChangelogPage from './page';
import { TooltipProvider } from '../../components/TooltipRegistry';

describe('ChangelogPage', () => {
  it('renders the release notes page correctly', () => {
    render(
      <TooltipProvider>
        <ChangelogPage />
      </TooltipProvider>
    );

    expect(screen.getByText('Release Notes & Changelog')).toBeInTheDocument();
    expect(screen.getByText('Version 1.0 (Latest)')).toBeInTheDocument();
    expect(screen.getByText('Read the full technical changelog on our website →')).toBeInTheDocument();
  });
});
