import { describe, it, expect, vi, afterEach } from 'vitest';
import { POST } from './route';
import { NextResponse } from 'next/server';

const fetchMock = vi.fn();
global.fetch = fetchMock as any;

describe('POST /api/agents/chat', () => {
  afterEach(() => {
    vi.clearAllMocks();
  });

  it('should proxy the request to the backend', async () => {
    fetchMock.mockResolvedValueOnce({
      ok: true,
      json: async () => ({ department_assigned: 'sales', agent: 'The Salesperson', success: true }),
    } as any);

    const request = new Request('http://localhost:3000/api/agents/chat', {
      method: 'POST',
      headers: {
        'x-tenant-id': 'test-tenant',
        'x-user-id': 'test-user',
        'content-type': 'application/json',
      },
      body: JSON.stringify({ message: 'Can you generate a quote?' }),
    });

    const response = await POST(request);
    const data = await response.json();

    expect(data.department_assigned).toBe('sales');
  });
});
