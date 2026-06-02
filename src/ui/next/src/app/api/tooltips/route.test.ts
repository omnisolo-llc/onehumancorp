import { describe, it, expect } from 'vitest';
import { GET } from './route';

describe('/api/tooltips GET', () => {
  it('returns the tooltips configuration', async () => {
    const response = await GET();
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data).toHaveProperty('bio-input-tooltip');
    expect(data['bio-input-tooltip']).toContain('Tell us what you sell');
  });
});
