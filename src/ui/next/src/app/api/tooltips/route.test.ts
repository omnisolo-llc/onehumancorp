import { describe, it, expect } from 'vitest';
import { GET } from './route';

describe('/api/tooltips GET', () => {
  it('returns a JSON object containing tooltips', async () => {
    const response = await GET();
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data).toHaveProperty('bio-input-tooltip');
    expect(data['bio-input-tooltip']).toBe('Tell us what you sell and who your customers are. Keep it simple!');
    expect(data).toHaveProperty('generate-btn-tooltip');
  });
});
