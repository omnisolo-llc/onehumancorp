import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import AgentAuditDashboard from './page';

vi.mock('next/link', () => {
  return {
    default: ({ children, href }: { children: React.ReactNode; href: string }) => {
      return <a href={href}>{children}</a>;
    }
  };
});

describe('Agent Audit Dashboard', () => {
  it('renders Agent Audit Dashboard heading', () => {
    render(<AgentAuditDashboard />);
    expect(screen.getByRole('heading', { name: 'Agent Audit Dashboard' })).toBeInTheDocument();
  });

  it('renders Cost Tracker', () => {
    render(<AgentAuditDashboard />);
    expect(screen.getByText('Cost Tracker')).toBeInTheDocument();
  });

  it('renders Operations', () => {
    render(<AgentAuditDashboard />);
    expect(screen.getByText('Operations')).toBeInTheDocument();
  });

  it('renders Marketing & Advertising', () => {
    render(<AgentAuditDashboard />);
    expect(screen.getByText('Marketing & Advertising')).toBeInTheDocument();
  });

  it('renders Violation Feed', () => {
    render(<AgentAuditDashboard />);
    expect(screen.getByText('Violation Feed')).toBeInTheDocument();
  });
});
