import React from 'react';
import { render, screen } from '@testing-library/react';
import { describe, it, expect, vi } from 'vitest';
import AgentAuditDashboard from './page';

vi.mock('next/link', () => {
  return {
    default: ({ children, href }: any) => <a href={href}>{children}</a>
  };
});

describe('AgentAuditDashboard', () => {
  it('renders correctly', () => {
    render(<AgentAuditDashboard />);
    expect(screen.getByText('Agent Audit Dashboard')).toBeInTheDocument();
  });
});
