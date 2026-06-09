import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import AgentAuditDashboard from './page';
import { vi } from 'vitest';

describe('Agent Audit Dashboard', () => {
  beforeEach(() => {
    global.fetch = vi.fn(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ entries: [] }),
      })
    ) as any;
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('renders Agent Audit Dashboard heading', async () => {
    render(<AgentAuditDashboard />);
    expect(await screen.findByRole('heading', { name: 'Agent Audit Dashboard' })).toBeInTheDocument();
  });

  it('renders Cost Tracker', async () => {
    render(<AgentAuditDashboard />);
    expect(await screen.findByText('Cost Tracker')).toBeInTheDocument();
  });

  it('renders Operations', async () => {
    render(<AgentAuditDashboard />);
    expect(await screen.findByText('Operations')).toBeInTheDocument();
  });

  it('renders Marketing & Advertising', async () => {
    render(<AgentAuditDashboard />);
    expect(await screen.findByText('Marketing & Advertising')).toBeInTheDocument();
  });

  it('renders Violation Feed', () => {
    render(<AgentAuditDashboard />);
    expect(screen.getByText('Cross-Agent Feed')).toBeInTheDocument();
  });
});
