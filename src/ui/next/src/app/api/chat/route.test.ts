import { POST } from './route';

describe('Chat Route', () => {
  it('should return default response for unrelated queries', async () => {
    const req = new Request('http://localhost/api/chat', {
      method: 'POST',
      body: JSON.stringify({ message: 'What is the weather today?' })
    });

    const res = await POST(req);
    const json = await res.json();

    expect(json.reply).toContain("I am your AI Help Agent!");
    expect(json.link.url).toBe("/help");
  });

  it('should match keywords and return specific article', async () => {
    const req = new Request('http://localhost/api/chat', {
      method: 'POST',
      body: JSON.stringify({ message: 'How do I add products to my store?' })
    });

    const res = await POST(req);
    const json = await res.json();

    expect(json.reply).toContain("My Store");
    expect(json.link.url).toBe("/help/my-store");
  });

  it('should handle errors for invalid payload', async () => {
    const req = new Request('http://localhost/api/chat', {
      method: 'POST',
      body: JSON.stringify({})
    });

    const res = await POST(req);
    expect(res.status).toBe(400);
  });
});
