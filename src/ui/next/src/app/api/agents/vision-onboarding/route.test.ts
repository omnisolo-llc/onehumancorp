import { describe, it, expect } from 'vitest';
import { POST } from './route';

describe('Vision Onboarding API', () => {
  it('returns extracted text', async () => {
    const req = new Request('http://localhost/api/agents/vision-onboarding', {
      method: 'POST'
    });
    const res = await POST(req);
    const data = await res.json();
    expect(res.status).toBe(200);
    expect(data.extracted_text).toBe('Chicken Shawarma Plate $12\nVegan Wrap $9');
  });
});
