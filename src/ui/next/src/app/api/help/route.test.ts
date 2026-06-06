import { describe, it, expect } from 'vitest';
import { GET } from './route';

describe('GET /api/help', () => {
  it('returns static help articles', async () => {
    const res = await GET();
    expect(res.status).toBe(200);
    const data = await res.json();
    expect(data.length).toBeGreaterThan(0);
    expect(data[0].title).toBe('Getting Started');
  });
});
