import { EventEmitter } from 'node:events';
import { describe, expect, it, vi } from 'vitest';
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

describe('agent workflow API', () => {
  it('creates a workflow and dispatches a RunWorkflow task to the agent CLI', async () => {
    const response = await POST(new Request('http://localhost/api/v1/agents/workflows', {
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
  });
});
