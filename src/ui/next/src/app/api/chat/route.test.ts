import { describe, it, expect, vi, beforeEach } from 'vitest';
import { POST } from './route';

describe('chat API', () => {
  beforeEach(() => {
    vi.clearAllMocks();
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

  it('fetches chat reply from the backend and returns it', async () => {
    const mockReply = {
      reply: 'I am your AI Help Agent! I specialize in answering questions about OHC features and helping you grow your small business. Check out our Getting Started guide.',
      link: { url: '/help/getting-started', title: 'Read the full article →' }
    };

    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve(mockReply),
    });

    const response = await POST(new Request('http://localhost/api/chat', {
      method: 'POST',
      body: JSON.stringify({ message: 'How do I add a product?' }),
    }));

    expect(response.status).toBe(200);
    const data = await response.json();

    expect(global.fetch).toHaveBeenCalledWith('http://localhost:8080/api/chat', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ message: 'How do I add a product?' })
    });
    expect(data).toHaveProperty('reply');
    expect(data.reply).toContain('AI Help Agent');
    expect(data).toHaveProperty('link');
    expect(data.link.url).toBe('/help/getting-started');
  });

  it('returns an error on backend error', async () => {
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 404,
    });

    const response = await POST(new Request('http://localhost/api/chat', {
      method: 'POST',
      body: JSON.stringify({ message: 'Test message' }),
    }));

    expect(response.status).toBe(404);
    const data = await response.json();
    expect(data).toEqual({ error: 'Backend error' });
  });

  it('handles fetch exceptions gracefully', async () => {
    global.fetch = vi.fn().mockRejectedValue(new Error('Network error'));

    const response = await POST(new Request('http://localhost/api/chat', {
      method: 'POST',
      body: JSON.stringify({ message: 'Test message' }),
    }));

    expect(response.status).toBe(500);
    const data = await response.json();
    expect(data).toEqual({ error: 'Backend communication failed' });
  });
});
