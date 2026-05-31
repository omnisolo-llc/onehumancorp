import { EventEmitter } from 'node:events';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { POST } from './route';

const { spawnMock } = vi.hoisted(() => ({
  spawnMock: vi.fn(() => {
    const child = new EventEmitter() as EventEmitter & {
      stdout: EventEmitter;
      stderr: EventEmitter;
    };
    child.stdout = new EventEmitter();
    child.stderr = new EventEmitter();
    queueMicrotask(() => child.emit('close', 0));
    return child;
  }),
}));

vi.mock('node:child_process', () => ({
  default: {
    spawn: spawnMock,
  },
  spawn: spawnMock,
}));

const AGENT_ENV_KEYS = [
  'OHC_AGENT_BINARY',
  'OHC_BUILTIN_AGENT_BINARY',
  'OHC_SOURCE_MODE',
  'OHC_STANDALONE_MODE',
] as const;

const originalEnv = Object.fromEntries(
  AGENT_ENV_KEYS.map((key) => [key, process.env[key]]),
) as Record<(typeof AGENT_ENV_KEYS)[number], string | undefined>;

describe('agent workflow API', () => {
  beforeEach(() => {
    spawnMock.mockClear();
  });

  afterEach(() => {
    for (const key of AGENT_ENV_KEYS) {
      const value = originalEnv[key];
      if (value === undefined) {
        delete process.env[key];
      } else {
        process.env[key] = value;
      }
    }
  });

  it('creates a standalone workflow and dispatches it through the server binary', async () => {
    const response = await POST(new Request('http://localhost/api/agents/workflows', {
      method: 'POST',
      body: JSON.stringify({
        name: 'Branch review',
        task: 'Review the branch',
      }),
    }) as any);

    expect(response.status).toBe(202);
    const body = await response.json();
    expect(body.workflow.name).toBe('Branch review');
    expect(body.workflow.workflow).toBe('ohc_review_branch');
    expect(body.workflow.command).toContain('server --task');
    expect(body.workflow.command).toContain('RunWorkflow');
    expect(spawnMock).toHaveBeenCalledWith(
      'server',
      expect.arrayContaining(['--task', expect.stringContaining('RunWorkflow')]),
      expect.objectContaining({ env: process.env }),
    );
  });

  it('creates a cluster workflow and dispatches it through the separate agent binary', async () => {
    process.env.OHC_STANDALONE_MODE = 'false';

    const response = await POST(new Request('http://localhost/api/agents/workflows', {
      method: 'POST',
      body: JSON.stringify({
        name: 'Branch review',
        task: 'Review the branch',
      }),
    }) as any);

    expect(response.status).toBe(202);
    const body = await response.json();
    expect(body.workflow.name).toBe('Branch review');
    expect(body.workflow.workflow).toBe('ohc_review_branch');
    expect(body.workflow.command).toContain('ohc-builtin-agent --task');
    expect(body.workflow.command).toContain('RunWorkflow');
    expect(spawnMock).toHaveBeenCalledWith(
      'ohc-builtin-agent',
      expect.arrayContaining(['--task', expect.stringContaining('RunWorkflow')]),
      expect.objectContaining({ env: process.env }),
    );
  });
});
