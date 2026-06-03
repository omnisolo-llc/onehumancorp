import '@testing-library/jest-dom';
import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import MissionTrackPage from './page';

describe('MissionTrackPage', () => {
  it('renders the Mission Control header', () => {
    render(<MissionTrackPage />);
    expect(screen.getByText('Mission Control')).toBeInTheDocument();
  });

  it('renders all missions by default', () => {
    render(<MissionTrackPage />);
    expect(screen.getByText('Identify undocumented endpoints')).toBeInTheDocument();
    expect(screen.getByText('Generate missing OpenAPI specs')).toBeInTheDocument();
    expect(screen.getByText('Write tutorial walkthroughs')).toBeInTheDocument();
  });
});
