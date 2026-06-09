import React from 'react';
import { render, screen, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import AgentAuditDashboard from './page';

describe('Agent Audit Dashboard', () => {
  let fetchMock: any;

  beforeEach(() => {
    fetchMock = vi.spyOn(global, 'fetch').mockImplementation(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ entries: [] }),
      } as Response)
    );
  });

  afterEach(() => {
    fetchMock.mockRestore();
  });

  it('renders Agent Audit Dashboard heading', async () => {
    await act(async () => {
      render(<AgentAuditDashboard />);
    });
    expect(screen.getByRole('heading', { name: 'Agent Audit Dashboard' })).toBeInTheDocument();
  });

  it('renders Cost Tracker', async () => {
    await act(async () => {
      render(<AgentAuditDashboard />);
    });
    expect(screen.getByText('Cost Tracker')).toBeInTheDocument();
  });

  it('renders Operations', async () => {
    await act(async () => {
      render(<AgentAuditDashboard />);
    });
    expect(screen.getByText('Operations')).toBeInTheDocument();
  });

  it('renders Marketing & Advertising', async () => {
    await act(async () => {
      render(<AgentAuditDashboard />);
    });
    expect(screen.getByText('Marketing & Advertising')).toBeInTheDocument();
  });

  it('renders Violation Feed', async () => {
    await act(async () => {
      render(<AgentAuditDashboard />);
    });
    expect(screen.getByRole('heading', { name: 'Cross-Agent Feed' })).toBeInTheDocument();
  });
});
