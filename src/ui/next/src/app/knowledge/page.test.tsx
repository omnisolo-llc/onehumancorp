import { render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('@/components/layout/PageHeader', () => ({ PageHeader: ({ title }: { title: string }) => <h1>{title}</h1> }));

import KnowledgePage from './page';

describe('KnowledgePage', () => {
  beforeEach(() => { global.fetch = vi.fn(); });

  it('renders only status and timestamps returned by the real memory API', async () => {
    vi.mocked(global.fetch).mockResolvedValue(new Response(JSON.stringify([{
      id: 'memory-1', content: 'Policy', source_type: 'policy.md', reliability_score: 73,
      last_referenced_at: '2026-07-18T05:00:00Z',
    }]), { status: 200 }));
    render(<KnowledgePage />);
    expect(await screen.findByText('policy.md')).toBeDefined();
    expect(screen.getByText('Reliability 73%')).toBeDefined();
    expect(screen.queryByText('Active')).toBeNull();
    expect(screen.queryByText('Updated just now')).toBeNull();
  });

  it('fails closed on an invalid response', async () => {
    vi.mocked(global.fetch).mockResolvedValue(new Response(JSON.stringify({ memories: [{ id: null }] }), { status: 200 }));
    render(<KnowledgePage />);
    expect(await screen.findByRole('alert')).toHaveTextContent('Document data is unavailable.');
  });
});
