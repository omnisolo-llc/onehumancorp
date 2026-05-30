import { describe, it, expect, vi } from 'vitest';
import { POST } from './route';

describe('POST /api/v1/growth/loyalty-program/generate', () => {
  it('should generate loyalty program details with valid input', async () => {
    const req = new Request('http://localhost/api/v1/growth/loyalty-program/generate', {
      method: 'POST',
      body: JSON.stringify({ rewardGoal: 5, rewardItem: 'coffee' }),
    });

    const res = await POST(req);
    const json = await res.json();

    expect(res.status).toBe(200);
    expect(json.result).toBe('Join our loyalty program! complete 5 and get a free coffee.');
  });

  it('should prevent verb duplication if action starts with Buy', async () => {
    const req = new Request('http://localhost/api/v1/growth/loyalty-program/generate', {
      method: 'POST',
      body: JSON.stringify({ rewardGoal: 'Buy 5 items', rewardItem: 'coffee' }),
    });

    const res = await POST(req);
    const json = await res.json();

    expect(res.status).toBe(200);
    expect(json.result).toBe('Join our loyalty program! Buy 5 items and get a free coffee.');
  });

  it('should return 400 if missing parameters', async () => {
    const req = new Request('http://localhost/api/v1/growth/loyalty-program/generate', {
      method: 'POST',
      body: JSON.stringify({ rewardGoal: 5 }),
    });

    const res = await POST(req);
    const json = await res.json();

    expect(res.status).toBe(400);
    expect(json.error).toBe('Missing parameters');
  });

  it('should return 500 on json parse error', async () => {
    const req = {
        json: vi.fn().mockRejectedValue(new Error('Parse error'))
    } as unknown as Request;

    const res = await POST(req);
    const json = await res.json();

    expect(res.status).toBe(500);
    expect(json.error).toBe('Failed to generate loyalty program');
  });
});
