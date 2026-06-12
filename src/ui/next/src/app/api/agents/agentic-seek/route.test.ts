import { POST } from './route';
import { NextRequest } from 'next/server';

describe('AgenticSeek API Route', () => {
  let fetchMock: any;

  beforeEach(() => {
    fetchMock = vi.spyOn(global, 'fetch').mockImplementation(() =>
      Promise.resolve({
        ok: true,
        json: () => Promise.resolve({ choices: [{ message: { content: 'Local Backend Result' } }] }),
      } as Response)
    );
  });

  afterEach(() => {
    fetchMock.mockRestore();
  });

  it('returns 400 if task is missing', async () => {
    const req = new NextRequest('http://localhost/api/agents/agentic-seek', {
      method: 'POST',
      body: JSON.stringify({}),
    });

    const res = await POST(req);
    expect(res.status).toBe(400);
    const data = await res.json();
    expect(data.error).toBe('Task is required');
  });

  it('calls the backend and returns the result', async () => {
    const req = new NextRequest('http://localhost/api/agents/agentic-seek', {
      method: 'POST',
      body: JSON.stringify({ task: 'Do something locally' }),
    });

    const res = await POST(req);
    expect(res.status).toBe(200);
    const data = await res.json();

    expect(fetchMock).toHaveBeenCalledWith(expect.any(String), expect.objectContaining({
      method: 'POST',
      headers: expect.objectContaining({
        'x-ohc-provider': 'agenticseek'
      }),
      body: expect.stringContaining('Do something locally')
    }));

    expect(data.result).toBe('Local Backend Result');
  });

  it('handles backend errors', async () => {
    fetchMock.mockImplementationOnce(() =>
      Promise.resolve({
        ok: false,
        status: 500,
        text: () => Promise.resolve('Backend crashed'),
      } as Response)
    );

    const req = new NextRequest('http://localhost/api/agents/agentic-seek', {
      method: 'POST',
      body: JSON.stringify({ task: 'Cause an error' }),
    });

    const res = await POST(req);
    expect(res.status).toBe(500);
    const data = await res.json();
    expect(data.error).toBe('Backend error: Backend crashed');
  });
});
