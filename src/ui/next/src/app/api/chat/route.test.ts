import { describe, expect, it, vi, afterEach } from 'vitest';
import { POST } from './route';

describe('chat API', () => {
  afterEach(() => {
    vi.restoreAllMocks();
  });
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
    // If backend is running, it returns 200 with a valid reply.
    // If not, it falls through to the catch block.
    // We mock fetch so that the test is deterministic.
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({ reply: "I can help with that." })
    });

    const response = await POST(new Request('http://localhost/api/chat', {
      method: 'POST',
      body: JSON.stringify({ message: 'How do I add a product?' }),
    }));

    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data).toHaveProperty('reply');
    expect(data.reply).toContain("I can help with that.");
  });
});
