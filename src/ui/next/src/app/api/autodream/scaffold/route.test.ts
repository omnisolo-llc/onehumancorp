import { expect, test, describe } from 'vitest';
import { POST } from './route';

describe('AutoDream Scaffold API', () => {
  test('returns 400 if prompt is missing', async () => {
    const req = new Request('http://localhost/api/autodream/scaffold', {
      method: 'POST',
      body: JSON.stringify({})
    });
    const res = await POST(req);
    expect(res.status).toBe(400);
    const json = await res.json();
    expect(json.error).toBe('Prompt is required');
  });

  test('successfully scaffolds based on prompt', async () => {
    const req = new Request('http://localhost/api/autodream/scaffold', {
      method: 'POST',
      body: JSON.stringify({ prompt: 'I bake cakes' })
    });
    const res = await POST(req);
    expect(res.status).toBe(200);
    const json = await res.json();
    expect(json.success).toBe(true);
    expect(json.business_type).toBe('Home Bakery');
    expect(json.business_name).toBe("Maya's Cakes");
    expect(json.products.length).toBeGreaterThan(0);
  });
});
