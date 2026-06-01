import { GET } from './route';

describe('GET /api/tooltips', () => {
  it('returns tooltips data', async () => {
    const response = await GET();
    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data).toHaveProperty('bio-input-tooltip');
    expect(data['bio-input-tooltip']).toBe('Tell us what you sell and who your customers are. Keep it simple!');
  });
});
