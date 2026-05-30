import { describe, expect, it } from 'vitest';
import { GET } from './route';

describe('videos API', () => {
  it('returns a list of videos', async () => {
    const response = await GET();

    expect(response.status).toBe(200);
    const data = await response.json();
    expect(Array.isArray(data)).toBe(true);
    expect(data.length).toBeGreaterThan(0);
    expect(data[0]).toHaveProperty('id');
    expect(data[0]).toHaveProperty('title');
    expect(data[0]).toHaveProperty('duration');
  });
});
