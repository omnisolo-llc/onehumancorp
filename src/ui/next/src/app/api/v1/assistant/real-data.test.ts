import { describe, expect, test, vi, beforeEach } from 'vitest';
import { GET as getTasks } from './tasks/route';
import { PATCH as patchTask } from './tasks/[id]/route';
import { GET as getMemory } from './memory/route';
import { PATCH as patchSkills } from './skills/route';
import { PATCH as patchConnectors } from './connectors/route';

vi.mock('@/lib/auth/backendTransport', () => ({
  proxyBackendRequest: vi.fn(async () => Response.json(
    { error: 'backend unavailable' },
    { status: 502 },
  )),
}));

describe('Assistant API Real Data Proxy Tests', () => {
  beforeEach(() => {
    // Mock global.fetch to simulate backend being unavailable
    vi.stubGlobal('fetch', vi.fn().mockRejectedValue(new Error('Connection refused')));
  });

  test('tasks route does not fall back to demo data when backend is unavailable', async () => {
    const response = await getTasks(new Request('http://localhost/api/v1/assistant/tasks'));
    const data = await response.json();
    expect(response.status).toBe(502);
    expect(data.error).toMatch(/unavailable/i);
    expect(data.tasks).toBeUndefined(); // Proves no demo fallback tasks are returned
  });

  test('task mutation proxy fails honestly when backend is unavailable', async () => {
    const request = new Request('http://localhost/api/v1/assistant/tasks/123', {
      method: 'PATCH',
      body: JSON.stringify({ action: 'archive' }),
    });
    const response = await patchTask(request, { params: Promise.resolve({ id: '123' }) });
    const data = await response.json();
    expect(response.status).toBe(502);
    expect(data.error).toMatch(/unavailable/i);
  });

  test('memory route proxy fails honestly when backend is unavailable', async () => {
    const response = await getMemory(new Request('http://localhost/api/v1/assistant/memory'));
    const data = await response.json();
    expect(response.status).toBe(502);
    expect(data.error).toMatch(/unavailable/i);
  });

  test('skills route proxy fails honestly when backend is unavailable', async () => {
    const request = new Request('http://localhost/api/v1/assistant/skills', {
      method: 'PATCH',
      body: JSON.stringify({ action: 'install', name: 'Test' }),
    });
    const response = await patchSkills(request);
    const data = await response.json();
    expect(response.status).toBe(502);
    expect(data.error).toMatch(/unavailable/i);
  });

  test('connectors route proxy fails honestly when backend is unavailable', async () => {
    const request = new Request('http://localhost/api/v1/assistant/connectors', {
      method: 'PATCH',
      body: JSON.stringify({ action: 'connect', name: 'Test' }),
    });
    const response = await patchConnectors(request);
    const data = await response.json();
    expect(response.status).toBe(502);
    expect(data.error).toMatch(/unavailable/i);
  });
});
