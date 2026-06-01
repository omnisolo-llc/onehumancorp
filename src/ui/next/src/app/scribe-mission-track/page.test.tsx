import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import MissionTrackPage from './page';
import { describe, it, expect } from 'vitest';

describe('MissionTrackPage', () => {
  it('renders correctly', () => {
    render(<MissionTrackPage />);
    expect(screen.getByText('Mission Control')).toBeInTheDocument();
    expect(screen.getByText('Identify undocumented endpoints')).toBeInTheDocument();
  });

  it('filters missions based on tabs', () => {
    render(<MissionTrackPage />);

    expect(screen.getByText('Identify undocumented endpoints')).toBeInTheDocument();
    expect(screen.getByText('Generate missing OpenAPI specs')).toBeInTheDocument();

    const activeTab = screen.getByRole('button', { name: 'active' });
    fireEvent.click(activeTab);

    expect(screen.queryByText('Identify undocumented endpoints')).not.toBeInTheDocument();
    expect(screen.getByText('Generate missing OpenAPI specs')).toBeInTheDocument();

    const completedTab = screen.getByRole('button', { name: 'completed' });
    fireEvent.click(completedTab);

    expect(screen.getByText('Identify undocumented endpoints')).toBeInTheDocument();
    expect(screen.queryByText('Generate missing OpenAPI specs')).not.toBeInTheDocument();
  });
});
