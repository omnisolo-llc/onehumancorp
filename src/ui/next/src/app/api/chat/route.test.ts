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
    global.fetch = async (url: RequestInfo | URL, init?: RequestInit) => {
      return new Response(JSON.stringify({
        reply: 'Based on our help center: To add a product...',
        link: { url: '/help/my-store', title: 'Read the full article →' }
      }), {
        status: 200,
        headers: { 'Content-Type': 'application/json' }
      });
    };

    const response = await POST(new Request('http://localhost/api/chat', {
      method: 'POST',
      body: JSON.stringify({ message: 'How do I add a product?' }),
    }));

    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data).toHaveProperty('reply');
    expect(data.reply).toContain('Based on our help center');
    expect(data).toHaveProperty('link');
    expect(data.link.url).toBe('/help/my-store');
  });

  it('handles backend failure gracefully', async () => {
    global.fetch = async (url: RequestInfo | URL, init?: RequestInit) => {
      return new Response(JSON.stringify({ error: 'Internal Server Error' }), {
        status: 500,
        headers: { 'Content-Type': 'application/json' }
      });
    };

    const response = await POST(new Request('http://localhost/api/chat', {
      method: 'POST',
      body: JSON.stringify({ message: 'Is the backend up?' }),
    }));

    expect(response.status).toBe(500);
    const data = await response.json();
    expect(data).toHaveProperty('error');
  });
});
