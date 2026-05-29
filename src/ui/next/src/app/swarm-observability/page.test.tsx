import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import SwarmObservabilityDashboard from './page';

// Mock fetch
global.fetch = vi.fn() as any;

describe('Swarm Observability Dashboard', () => {
  beforeEach(() => {
    (global.fetch as any).mockClear();
    (global.fetch as any).mockResolvedValue({
      ok: true,
      json: async () => ({
        active_agents: 42,
        pending_missions: 0,
        avg_task_latency_ms: 120,
        db_mode: 'cloud'
      })
    });
  });

  it('renders heading and back link', async () => {
    render(<SwarmObservabilityDashboard />);
    expect(screen.getByRole('heading', { name: 'Swarm Observability Dashboard' })).toBeInTheDocument();
    expect(screen.getByText('< Back to Dashboard')).toBeInTheDocument();
  });

  it('renders metrics data after fetching', async () => {
    render(<SwarmObservabilityDashboard />);

    await waitFor(() => {
      expect(screen.getByText('42')).toBeInTheDocument();
      expect(screen.getByText('0')).toBeInTheDocument();
      expect(screen.getByText('120')).toBeInTheDocument();
      expect(screen.getByText('cloud')).toBeInTheDocument();
    });

    expect(screen.getByText('Active Agents')).toBeInTheDocument();
    expect(screen.getByText('Pending Missions')).toBeInTheDocument();
    expect(screen.getByText('Avg Task Latency')).toBeInTheDocument();
    expect(screen.getByText('Database Mode')).toBeInTheDocument();
  });
});
