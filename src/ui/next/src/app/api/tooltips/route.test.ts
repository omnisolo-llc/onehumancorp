import { describe, expect, it } from 'vitest';
import { GET } from './route';

describe('tooltips API', () => {
  it('returns a map of tooltips', async () => {
    const response = await GET();

    expect(response.status).toBe(200);
    const data = await response.json();
    expect(typeof data).toBe('object');
    expect(data).not.toBeNull();
    expect(Object.keys(data).length).toBeGreaterThan(0);
    expect(data['bio-input-tooltip']).toBeDefined();
    expect(typeof data['bio-input-tooltip']).toBe('string');
  });
});
