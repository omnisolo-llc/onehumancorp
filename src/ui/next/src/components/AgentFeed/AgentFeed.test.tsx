import { render, screen, waitFor } from '@testing-library/react';
import { AgentFeed } from './AgentFeed';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import React from 'react';

describe('AgentFeed', () => {
  beforeEach(() => {
    vi.stubGlobal('fetch', vi.fn());
  });

  it('shows loading state initially', () => {
    (fetch as any).mockReturnValue(new Promise(() => {}));
    render(<AgentFeed />);
    expect(screen.getByText('Consulting agent council...')).toBeDefined();
  });

  it('renders proposals after fetch', async () => {
    const mockProposals = [{
      id: '1',
      agent_id: 'marketing-agent',
      title: 'Post',
      description: 'Desc',
      status: 'pending',
      created_at: new Date().toISOString(),
    }];

    (fetch as any).mockResolvedValue({
      ok: true,
      json: async () => mockProposals,
    });

    render(<AgentFeed />);
    await waitFor(() => expect(screen.getByText('Post')).toBeDefined());
  });

  it('shows error state on fetch failure', async () => {
    (fetch as any).mockRejectedValue(new Error('Fetch failed'));
    render(<AgentFeed />);
    await waitFor(() => expect(screen.getByText('Failed to load agent proposals')).toBeDefined());
  });
});
