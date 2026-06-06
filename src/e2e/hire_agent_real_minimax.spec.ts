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

const specialistLabels = [
  'Revenue strategist',
  'Operations analyst',
  'Finance controller',
  'Customer success lead',
  'Risk and compliance reviewer',
] as const;

async function readWorkflow(request: APIRequestContext, workflowId: string): Promise<WorkflowRecord | undefined> {
  const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';
  const response = await request.get(`${apiBase}/api/agents/workflows`);
  expect(response.ok()).toBeTruthy();
  const body = await response.json();
  return (body.workflows as WorkflowRecord[]).find((workflow) => workflow.id === workflowId);
}

async function readAgentCommands(): Promise<string> {
  const { stdout } = await execFileAsync('ps', ['-eo', 'args='], { timeout: 5000 });
  return stdout;
}

function specialistProcessesFor(commands: string, agentName: string): string[] {
  const lines = commands.split('\n');
  return specialistLabels.filter((label) =>
    lines.some((line) =>
      line.includes('ohc-builtin-agent --task') &&
      line.includes(agentName) &&
      line.includes(label),
    ),
  );
}

test.describe('real MiniMax hire-agent flow', () => {
  test('hiring an agent starts a real M3 business swarm with specialist agents', async ({ request }) => {
    test.setTimeout(360_000);

    test.skip(!process.env.MINIMAX_API_KEY, 'requires a real MINIMAX_API_KEY in the environment or .env');
    expect(process.env.MINIMAX_API_KEY, 'requires a real MINIMAX_API_KEY in the environment or .env').toBeTruthy();
    expect(process.env.OHC_LLM_PROVIDER || 'minimax').toBe('minimax');
    expect(process.env.OHC_LLM_MODEL || process.env.MINIMAX_MODEL || 'MiniMax-M3').toBe('MiniMax-M3');

    const agentName = `M3 E2E Business Operator ${Date.now()}`;
    const apiBase = process.env.OHC_API_URL || process.env.BACKEND_URL || process.env.BASE_URL || '';
    const hireResponse = await request.post(`${apiBase}/api/agents/hire`, {
      data: {
        name: agentName,
        role: 'Business growth operator',
        providerType: 'builtin',
        model: 'MiniMax-M3',
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
        if (workflow.status === 'failed') return `failed: ${workflow.error || workflow.output || 'unknown error'}`;
        return workflow.status;
      }, { timeout: 30_000, intervals: [500, 1000, 2000] })
      .toMatch(/running|completed/);

    expect(workflow).toBeDefined();
    expect(workflow!.workflow).toBe('ohc_business_swarm');
    expect(workflow!.task).toContain(agentName);
    expect(workflow!.task).toContain('MiniMax-M3');
    expect(workflow!.command).toContain('ohc_business_swarm');
    expect(workflow!.command).toContain('MiniMax-M3');

    await expect
      .poll(async () => {
        const latest = await readWorkflow(request, hired.workflow_id);
        if (latest?.status === 'failed') {
          return `workflow failed: ${latest.error || latest.output || 'unknown error'}`;
        }
        const commands = await readAgentCommands();
        return specialistProcessesFor(commands, agentName).join('|');
      }, { timeout: 140_000, intervals: [1000, 2000, 5000] })
      .toBe(specialistLabels.join('|'));

    const commands = await readAgentCommands();
    const runningSpecialists = specialistProcessesFor(commands, agentName);
    expect(runningSpecialists).toEqual(expect.arrayContaining([...specialistLabels]));
    expect(commands).toContain('ohc-builtin-agent --task');
    expect(commands).toContain(agentName);
    expect(commands).toContain('MiniMax-M3');
  });
});
