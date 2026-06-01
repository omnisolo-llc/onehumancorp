import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import AgentAuditDashboard from './page';

describe('Agent Audit Dashboard', () => {
  it('renders Agent Audit Dashboard heading', () => {
    render(<AgentAuditDashboard />);
    expect(screen.getByRole('heading', { name: 'Agent Audit Dashboard' })).toBeTruthy();
  });

  it('renders Cost Tracker', () => {
    render(<AgentAuditDashboard />);
    expect(screen.getByText('Cost Tracker')).toBeTruthy();
  });

  it('renders Operations', () => {
    render(<AgentAuditDashboard />);
    expect(screen.getByText('Operations')).toBeTruthy();
  });

  it('renders Marketing & Advertising', () => {
    render(<AgentAuditDashboard />);
    expect(screen.getByText('Marketing & Advertising')).toBeTruthy();
  });

  it('renders Violation Feed', () => {
    render(<AgentAuditDashboard />);
    expect(screen.getByText('Violation Feed')).toBeTruthy();
  });
});
