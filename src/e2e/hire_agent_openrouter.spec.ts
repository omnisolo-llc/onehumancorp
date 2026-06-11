import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { test, expect, type APIRequestContext } from '@playwright/test';

const execFileAsync = promisify(execFile);

type WorkflowRecord = {
  id: string;
  name: string;
  workflow: string;
  task: string;
  status: string;
  command: string;
  output?: string | null;
  error?: string | null;
};

async function readWorkflow(request: APIRequestContext, workflowId: string): Promise<WorkflowRecord | undefined> {
  const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';
  const response = await request.get(`${apiBase}/api/agents/workflows`);
  expect(response.ok()).toBeTruthy();
  const body = await response.json();
  return (body.workflows as WorkflowRecord[]).find((workflow) => workflow.id === workflowId);
}

test.describe('openrouter hire-agent flow', () => {
  test('hiring an agent triggers the correct workflow using openrouter', async ({ request }) => {
    test.setTimeout(120_000);

    // We do not skip if there is no OPENROUTER_API_KEY because we can still test the initial route parsing and startup.
    // The agent might fail to make the network request, but it will be instantiated correctly.

    // For local tests where we want to ensure the process runs
    process.env.OHC_LLM_PROVIDER = 'openrouter';
    process.env.OPENROUTER_MODEL = 'openai/gpt-4o-mini';
    process.env.OPENROUTER_API_KEY = process.env.OPENROUTER_API_KEY || 'fake-key-for-test';

    const agentName = `OpenRouter E2E Business Operator ${Date.now()}`;
    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';
    const hireResponse = await request.post(`${apiBase}/api/agents/hire`, {
      data: {
        name: agentName,
        role: 'Business growth operator',
        providerType: 'builtin',
        model: 'openai/gpt-4o-mini',
      },
    });

    expect(hireResponse.status()).toBe(201);
    const hired = await hireResponse.json();
    expect(hired.status).toBe('running');
    expect(hired.agent_id).toMatch(/^agent-/);
    expect(hired.workflow_id).toMatch(/^[0-9a-f-]{36}$/);

    const agentsResponse = await request.get(`${apiBase}/api/agents`);
    expect(agentsResponse.ok()).toBeTruthy();
    const agents = await agentsResponse.json();
    const hiredAgent = agents.find((agent: any) => agent.id === hired.agent_id);
    expect(hiredAgent).toMatchObject({
      name: agentName,
      role: 'Business growth operator',
      status: 'RUNNING',
      provider_type: 'builtin',
    });

    let workflow: WorkflowRecord | undefined;
    await expect
      .poll(async () => {
        workflow = await readWorkflow(request, hired.workflow_id);
        if (!workflow) return 'missing';
        if (workflow.status === 'failed' || workflow.status === 'completed') return workflow.status;
        return workflow.status;
      }, { timeout: 30_000, intervals: [500, 1000, 2000] })
      .toMatch(/running|completed|failed/);

    expect(workflow).toBeDefined();
    expect(workflow!.workflow).toBe('ohc_business_swarm');
    expect(workflow!.task).toContain(agentName);
  });
});
