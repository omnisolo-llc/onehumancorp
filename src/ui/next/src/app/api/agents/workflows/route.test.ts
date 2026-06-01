import { EventEmitter } from 'node:events';
import { describe, expect, it, vi, beforeEach } from 'vitest';
import { GET, POST } from './route';

const { spawnMock } = vi.hoisted(() => ({
  spawnMock: vi.fn(() => {
    const child = new EventEmitter() as EventEmitter & {
      stdout: EventEmitter;
      stderr: EventEmitter;
    };
    child.stdout = new EventEmitter();
    child.stderr = new EventEmitter();
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
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('GET returns workflows', async () => {
    const response = await GET();
    expect(response.status).toBe(200);
    const body = await response.json();
    expect(Array.isArray(body.workflows)).toBe(true);
  });

  it('POST returns 400 if name or task missing', async () => {
    const res1 = await POST(new Request('http://localhost', { method: 'POST', body: JSON.stringify({}) }) as any);
    expect(res1.status).toBe(400);

    const res2 = await POST(new Request('http://localhost', { method: 'POST', body: 'invalid json' }) as any);
    expect(res2.status).toBe(400);
  });

  it('creates a workflow and dispatches a RunWorkflow task to the agent CLI', async () => {
    const originalEnv = process.env.OHC_STANDALONE_MODE;
    process.env.OHC_STANDALONE_MODE = 'false'; // test isCluster path

    let mockChild: any;
    spawnMock.mockImplementationOnce(() => {
      mockChild = new EventEmitter() as EventEmitter & { stdout: EventEmitter; stderr: EventEmitter; };
      mockChild.stdout = new EventEmitter();
      mockChild.stderr = new EventEmitter();
      return mockChild;
    });

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
    expect(body.workflow.command).toContain('ohc-builtin-agent --task');

    // Test child output processing
    mockChild.stdout.emit('data', 'test output');
    mockChild.stderr.emit('data', 'test error');
    mockChild.emit('close', 1);

    const res2 = await GET();
    const data = await res2.json();
    const w = data.workflows.find((w: any) => w.id === body.workflow.id);
    expect(w.status).toBe('failed');
    expect(w.output).toBe('test output');
    expect(w.error).toBe('test error');

    process.env.OHC_STANDALONE_MODE = originalEnv;
  });

  it('handles child process error', async () => {
    let mockChild: any;
    spawnMock.mockImplementationOnce(() => {
      mockChild = new EventEmitter() as EventEmitter & { stdout: EventEmitter; stderr: EventEmitter; };
      mockChild.stdout = new EventEmitter();
      mockChild.stderr = new EventEmitter();
      return mockChild;
    });

    const response = await POST(new Request('http://localhost/api/agents/workflows', {
      method: 'POST',
      body: JSON.stringify({ name: 'Error test', task: 'Review' }),
    }) as any);

    const body = await response.json();
    mockChild.emit('error', new Error('Spawn failed'));
    mockChild.emit('close', 1); // should ignore close if already failed

    const res = await GET();
    const data = await res.json();
    const w = data.workflows.find((w: any) => w.id === body.workflow.id);
    expect(w.status).toBe('failed');
    expect(w.error).toContain('Spawn failed');
  });

  it('handles agentBinary override via OHC_BUILTIN_AGENT_BINARY', async () => {
    const originalEnv = process.env.OHC_BUILTIN_AGENT_BINARY;
    process.env.OHC_BUILTIN_AGENT_BINARY = 'custom-binary';

    const response = await POST(new Request('http://localhost/api/agents/workflows', {
      method: 'POST',
      body: JSON.stringify({ name: 'Test', task: 'Task' }),
    }) as any);

    const body = await response.json();
    expect(body.workflow.command).toContain('custom-binary --task');

    process.env.OHC_BUILTIN_AGENT_BINARY = originalEnv;
  });
});
