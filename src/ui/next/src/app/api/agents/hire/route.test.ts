import { describe, expect, it, vi, beforeEach, afterEach } from 'vitest';

describe('/api/agents/hire', () => {
  const originalBackend = process.env.BACKEND_URL;
  const originalOhcApi = process.env.OHC_API_URL;

  beforeEach(() => {
    vi.resetModules();
    delete process.env.BACKEND_URL;
    delete process.env.OHC_API_URL;
  });

  afterEach(() => {
    if (originalBackend === undefined) delete process.env.BACKEND_URL;
    else process.env.BACKEND_URL = originalBackend;
    if (originalOhcApi === undefined) delete process.env.OHC_API_URL;
    else process.env.OHC_API_URL = originalOhcApi;
  });

  it('creates a local running hire response when no backend is configured', async () => {
    const { POST } = await import('./route');
    const response = await POST(
      new Request('http://localhost/api/agents/hire', {
        method: 'POST',
        body: JSON.stringify({
          name: 'Growth Strategist',
          role: 'Business growth operator',
          model: 'MiniMax-M3',
          mode: 'Plan',
          workspace: 'Marketing sprint',
          task: 'Plan a launch',
          skills: ['Web Research'],
          connectors: ['Tencent Docs'],
          contextReferences: '@orders @inventory',
          attachments: 'launch.png, revenue.csv',
          customProvider: 'https://llm.example.com/v1',
          workDirectory: '/workspace/launch',
          outputFormat: 'Spreadsheet',
          taskConstraints: 'Budget under $500',
        }),
      }) as any,
    );

    expect(response.status).toBe(201);
    const body = await response.json();
    expect(body.status).toBe('running');
    expect(body.agent_id).toMatch(/^agent-/);
    expect(body.workflow_id).toMatch(/[0-9a-f-]{36}/);
    expect(body.expert).toMatchObject({
      name: 'Growth Strategist',
      role: 'Business growth operator',
      model: 'MiniMax-M3',
      mode: 'Plan',
      workspace: 'Marketing sprint',
      task: 'Plan a launch',
      skills: ['Web Research'],
      connectors: ['Tencent Docs'],
      contextReferences: '@orders @inventory',
      attachments: 'launch.png, revenue.csv',
      customProvider: 'https://llm.example.com/v1',
      workDirectory: '/workspace/launch',
      outputFormat: 'Spreadsheet',
      taskConstraints: 'Budget under $500',
    });
  });

  it('rejects invalid hire requests', async () => {
    const { POST } = await import('./route');
    const response = await POST(
      new Request('http://localhost/api/agents/hire', {
        method: 'POST',
        body: JSON.stringify({ name: '', role: '' }),
      }) as any,
    );

    expect(response.status).toBe(400);
    await expect(response.json()).resolves.toMatchObject({
      status: 'error',
      message: 'Expert name and role are required',
    });
  });
});
