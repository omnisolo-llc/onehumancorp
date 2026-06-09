import React from 'react';
import { render, screen, act } from '@testing-library/react';
import '@testing-library/jest-dom';
import AgentAuditDashboard from './page';
import { AppRouterContext } from "next/dist/shared/lib/app-router-context.shared-runtime";

global.fetch = vi.fn(() => Promise.resolve({
  ok: true,
  json: () => Promise.resolve({entries: []})
})) as any;

describe('Agent Audit Dashboard', () => {
  it('renders Agent Audit Dashboard heading', async () => {
    await act(async () => {
        render(<AppRouterContext.Provider value={{} as any}><AgentAuditDashboard /></AppRouterContext.Provider>);
    });
    expect(screen.getByRole('heading', { name: 'Agent Audit Dashboard' })).toBeInTheDocument();
  });

  it('renders Cost Tracker', async () => {
    await act(async () => {
        render(<AppRouterContext.Provider value={{} as any}><AgentAuditDashboard /></AppRouterContext.Provider>);
    });
    expect(screen.getByText('Cost Tracker')).toBeInTheDocument();
  });

  it('renders Operations', async () => {
    await act(async () => {
        render(<AppRouterContext.Provider value={{} as any}><AgentAuditDashboard /></AppRouterContext.Provider>);
    });
    expect(screen.getByText('Operations')).toBeInTheDocument();
  });

  it('renders Marketing & Advertising', async () => {
    await act(async () => {
        render(<AppRouterContext.Provider value={{} as any}><AgentAuditDashboard /></AppRouterContext.Provider>);
    });
    expect(screen.getByText('Marketing & Advertising')).toBeInTheDocument();
  });

  it('renders Violation Feed', async () => {
    await act(async () => {
        render(<AppRouterContext.Provider value={{} as any}><AgentAuditDashboard /></AppRouterContext.Provider>);
    });
    expect(screen.getByRole('heading', { name: 'Cross-Agent Feed' })).toBeInTheDocument();
  });
});
