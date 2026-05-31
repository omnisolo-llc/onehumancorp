import { NextRequest } from 'next/server';
import { POST } from './route';

describe('POST /api/mesh/v2/broadcast', () => {
  it('should return 422 if body is invalid', async () => {
    const req = new NextRequest('http://localhost', {
      method: 'POST',
      body: JSON.stringify({}),
    });
    const res = await POST(req);
    expect(res.status).toBe(422);
  });

  it('should return 200 on success', async () => {
    const req = new NextRequest('http://localhost', {
      method: 'POST',
      body: JSON.stringify({
        data: {
          message: {
            agent_id: 'agent1',
            action: 'action1',
            status: 'status1',
            channel: 'channel1',
            payload: 'payload1',
            msg_id: 'msg1',
          },
        },
      }),
    });
    const res = await POST(req);
    expect(res.status).toBe(200);
  });
});
