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
    global.fetch = vi.fn().mockResolvedValue({
      ok: true,
      json: () => Promise.resolve({
        reply: 'Based on our help center: To set up your storefront, go to the \'My Store\' tab and add your products. It\'s easy! Just upload a photo, write a simple description, and set a price.',
        link: { url: '/help/my-store', title: 'Read the full article →' }
      })
    });

    const response = await POST(new Request('http://localhost/api/chat', {
      method: 'POST',
      body: JSON.stringify({ message: 'How do I set up my store?' }),
    }));

    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data).toHaveProperty('reply');
    expect(data.reply).toContain("Based on our help center: To set up your storefront, go to the 'My Store' tab and add your products. It's easy! Just upload a photo, write a simple description, and set a price.");
    expect(data).toHaveProperty('link');
    expect(data.link.url).toBe('/help/my-store');
    expect(data.link.title).toBe('Read the full article →');
  });

  it('handles backend failure gracefully', async () => {
    const consoleSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

    // Mock a failure response
    global.fetch = vi.fn().mockResolvedValue({
      ok: false,
      status: 500,
    });

    const response = await POST(new Request('http://localhost/api/chat', {
      method: 'POST',
      body: JSON.stringify({ message: 'How do I add a product?' }),
    }));

    expect(response.status).toBe(200);
    const data = await response.json();
    expect(data).toHaveProperty('reply');
    expect(data.reply).toContain("I'm having trouble connecting to my brain right now");

    expect(consoleSpy).toHaveBeenCalled();
  });
});
