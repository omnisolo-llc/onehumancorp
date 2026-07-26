import { describe, expect, it } from 'vitest';
import { GET } from './route';

describe('Community Goal Widget Embed API', () => {
  it('should render HTML with correct parameters', async () => {
    const req = new Request('http://localhost:3000/api/v1/growth/community-goal-widget/embed?target=2000&reward=Super%20Secret%20Menu&tenant=test-tenant');
    const res = await GET(req);

    expect(res.status).toBe(200);
    expect(res.headers.get('Content-Type')).toBe('text/html');

    const html = await res.text();
    expect(html).toContain('Community Goal');
    expect(html).toContain('Super Secret Menu');
    expect(html).toContain('Goal: 2000');
    expect(html).toContain('test-tenant');
  });

  it('should fallback to defaults when parameters are missing', async () => {
    const req = new Request('http://localhost:3000/api/v1/growth/community-goal-widget/embed');
    const res = await GET(req);

    expect(res.status).toBe(200);

    const html = await res.text();
    expect(html).toContain('1000'); // default target
    expect(html).toContain('Free shipping for everyone!'); // default reward
    expect(html).toContain('default'); // default tenant
  });
});
