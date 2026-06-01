import React from 'react';
import { render, screen } from '@testing-library/react';
import DashboardPage from './page';

describe('Dashboard Page', () => {
  it('renders the Dashboard header', () => {
    render(<DashboardPage />);
    expect(screen.getByText('Dashboard')).toBeInTheDocument();
  });
});
