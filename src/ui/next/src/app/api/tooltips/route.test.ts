import { GET } from './route';
import { describe, it, expect } from 'vitest';

describe('GET /api/tooltips', () => {
  it('returns a dictionary of tooltips', async () => {
    const response = await GET();
    const data = await response.json();
    expect(response.status).toBe(200);
    expect(data['help-btn-tooltip']).toBeDefined();
    expect(typeof data['help-btn-tooltip']).toBe('string');
  });
});
