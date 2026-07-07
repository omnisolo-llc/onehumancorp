import { describe, it, expect, vi, beforeEach } from 'vitest';
import { PUT } from './route';

const mockFetch = vi.fn();
global.fetch = mockFetch;

describe('Staff Tasks ID API Route', () => {
  beforeEach(() => {
    mockFetch.mockClear();
    process.env.BACKEND_URL = 'http://127.0.0.1:18789';
  });

  it('PUT should update a task', async () => {
    const mockUpdate = { status: 'COMPLETED' };
    const mockResponse = { id: 'task_1', status: 'COMPLETED' };
    mockFetch.mockResolvedValueOnce({
      status: 200,
      json: async () => mockResponse
    });

    const req = new Request('http://localhost:3000/api/staff/tasks/task_1', {
      method: 'PUT',
      headers: new Headers({
        'x-tenant-id': 'tenant_1',
      }),
      body: JSON.stringify(mockUpdate)
    });

    const res = await PUT(req, { params: { id: 'task_1' } });
    const data = await res.json();

    expect(mockFetch).toHaveBeenCalledWith('http://127.0.0.1:18789/api/staff/tasks/task_1', expect.any(Object));
    expect(res.status).toBe(200);
    expect(data).toEqual(mockResponse);
  });
});
