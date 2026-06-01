import { describe, it, expect } from 'vitest';
import { GET } from './route';

describe('/api/help GET', () => {
  it('returns the help articles list', async () => {
    const response = await GET();
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(Array.isArray(data)).toBe(true);
    expect(data.length).toBeGreaterThan(0);
    expect(data[0]).toHaveProperty('title');
    expect(data[0]).toHaveProperty('desc');
    expect(data[0]).toHaveProperty('link');
  });
});
