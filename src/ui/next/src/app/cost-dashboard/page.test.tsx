import React from 'react';
import { render, screen } from '@testing-library/react';
import CostDashboardPage from './page';

describe('Cost Dashboard Page', () => {
  it('renders the Cost Dashboard header', () => {
    render(<CostDashboardPage />);
    expect(screen.getByText('Business Advisory Dashboard')).toBeInTheDocument();
  });
});
