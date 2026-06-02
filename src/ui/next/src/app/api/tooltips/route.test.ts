import { describe, it, expect } from 'vitest';
import { GET } from './route';

describe('GET /api/tooltips', () => {
  it('returns a JSON response with tooltips', async () => {
    const response = await GET();
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data['bio-input-tooltip']).toBeDefined();
    expect(data['generate-btn-tooltip']).toBeDefined();
  });
});
