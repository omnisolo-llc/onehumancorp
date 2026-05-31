import { describe, expect, it } from 'vitest';
import { POST } from './route';

describe('post-purchase share API', () => {
  it('successfully tracks share', async () => {
    const response = await POST(new Request('http://localhost/api/v1/growth/post-purchase-share', {
      method: 'POST',
      body: JSON.stringify({ platform: 'whatsapp' }),
    }));

    expect(response.status).toBe(200);
    const body = await response.json();
    expect(body.success).toBe(true);
    expect(body.tracked).toBe('whatsapp');
  });
});
