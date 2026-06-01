import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect } from 'vitest';
import MissionTrackPage from './page';
import userEvent from '@testing-library/user-event';

describe('MissionTrackPage', () => {
  it('renders correctly and shows all missions by default', () => {
    render(<MissionTrackPage />);

    expect(screen.getByText('Mission Control')).toBeInTheDocument();

    // Default missions from component state
    expect(screen.getByText('Identify undocumented endpoints')).toBeInTheDocument();
    expect(screen.getByText('Generate missing OpenAPI specs')).toBeInTheDocument();
    expect(screen.getByText('Review inline code comments')).toBeInTheDocument();
  });

  it('filters missions when tabs are clicked', async () => {
    const user = userEvent.setup();
    render(<MissionTrackPage />);

    const activeTab = screen.getByRole('button', { name: /active/i });
    await user.click(activeTab);

    expect(screen.queryByText('Identify undocumented endpoints')).not.toBeInTheDocument(); // completed
    expect(screen.getByText('Generate missing OpenAPI specs')).toBeInTheDocument(); // active
    expect(screen.queryByText('Review inline code comments')).not.toBeInTheDocument(); // pending

    const completedTab = screen.getByRole('button', { name: /completed/i });
    await user.click(completedTab);

    expect(screen.getByText('Identify undocumented endpoints')).toBeInTheDocument(); // completed
    expect(screen.queryByText('Generate missing OpenAPI specs')).not.toBeInTheDocument(); // active
  });
});
