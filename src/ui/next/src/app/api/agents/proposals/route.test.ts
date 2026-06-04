import { GET } from './route';
import { describe, it, expect } from 'vitest';

describe('GET /api/agents/proposals', () => {
  it('returns a list of proposals', async () => {
    const response = await GET();
    const data = await response.json();

    expect(response.status).toBe(200);
    expect(data.proposals).toBeDefined();
    expect(data.proposals.length).toBeGreaterThan(0);
    expect(data.proposals[0].title).toBe('Launch Summer Promo Campaign');
    expect(data.proposals[1].title).toBe('Low Stock: Premium Fertilizer');
  });
});
