import { describe, it, expect, vi, beforeEach } from 'vitest';
import { GET, POST } from './route';

const mockFetch = vi.fn();
global.fetch = mockFetch;

describe('Staff Tasks API Route', () => {
  beforeEach(() => {
    mockFetch.mockClear();
    process.env.BACKEND_URL = 'http://127.0.0.1:18789';
  });

  it('GET should fetch tasks from backend', async () => {
    const mockTasks = [{ id: 'task_1', description: 'Test Task' }];
    mockFetch.mockResolvedValueOnce({
      status: 200,
      json: async () => mockTasks
    });

    const req = new Request('http://localhost:3000/api/staff/tasks', {
      headers: new Headers({
        'x-tenant-id': 'tenant_1',
      })
    });

    const res = await GET(req);
    const data = await res.json();

    expect(mockFetch).toHaveBeenCalledWith('http://127.0.0.1:18789/api/staff/tasks', expect.any(Object));
    expect(res.status).toBe(200);
    expect(data).toEqual(mockTasks);
  });

  it('POST should create a task', async () => {
    const mockTask = { description: 'New Task' };
    const mockResponse = { id: 'task_2', ...mockTask };
    mockFetch.mockResolvedValueOnce({
      status: 201,
      json: async () => mockResponse
    });

    const req = new Request('http://localhost:3000/api/staff/tasks', {
      method: 'POST',
      headers: new Headers({
        'x-tenant-id': 'tenant_1',
      }),
      body: JSON.stringify(mockTask)
    });

    const res = await POST(req);
    const data = await res.json();

    expect(mockFetch).toHaveBeenCalledWith('http://127.0.0.1:18789/api/staff/tasks', expect.any(Object));
    expect(res.status).toBe(201);
    expect(data).toEqual(mockResponse);
  });
});
