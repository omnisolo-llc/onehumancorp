import { describe, it, expect } from 'vitest';
import { GET } from './route';

describe('/api/videos GET', () => {
  it('returns the list of videos', async () => {
    const response = await GET();
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(Array.isArray(data)).toBe(true);
    expect(data.length).toBe(10);
    expect(data[0]).toHaveProperty('title');
    expect(data[0]).toHaveProperty('duration');
  });
});
