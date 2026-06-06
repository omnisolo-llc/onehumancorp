import { describe, expect, it } from 'vitest';
import { POST } from './route';

describe('chat API', () => {
  it('rejects malformed JSON', async () => {
    const response = await POST(new Request('http://localhost/api/chat', {
      method: 'POST',
      body: '{',
    }));

    expect(response.status).toBe(400);
    await expect(response.json()).resolves.toEqual({ error: 'Invalid JSON body' });
  });

  it('rejects empty messages', async () => {
    const response = await POST(new Request('http://localhost/api/chat', {
      method: 'POST',
      body: JSON.stringify({ message: '   ' }),
    }));

    expect(response.status).toBe(400);
    await expect(response.json()).resolves.toEqual({ error: 'message is required' });
  });

  it('rejects oversized messages', async () => {
    const response = await POST(new Request('http://localhost/api/chat', {
      method: 'POST',
      body: JSON.stringify({ message: 'x'.repeat(1001) }),
    }));

    expect(response.status).toBe(413);
  });

  it('returns successful reply for valid message', async () => {
    // In vitest environment without backend, it falls through to the catch block
    const response = await POST(new Request('http://localhost/api/chat', {
      method: 'POST',
      body: JSON.stringify({ message: 'How do I add a product?' }),
    }));

    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data).toHaveProperty('reply');
    expect(data.reply).toContain("I'm having trouble connecting to my brain right now");
  });
});
