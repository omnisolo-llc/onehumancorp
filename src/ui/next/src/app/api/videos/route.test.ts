import { describe, it, expect } from 'vitest';
import { GET } from './route';

describe('Videos API Route', () => {
  it('returns a list of video objects', async () => {
    const res = await GET();
    const data = await res.json();

    expect(Array.isArray(data)).toBe(true);
    expect(data.length).toBeGreaterThan(0);
    expect(data[0]).toHaveProperty('id');
    expect(data[0]).toHaveProperty('title');
    expect(data[0]).toHaveProperty('duration');
    expect(data[0]).toHaveProperty('description');
    expect(data[0]).toHaveProperty('url');
  });
});