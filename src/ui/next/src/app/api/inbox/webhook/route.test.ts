import { POST } from './route';
import { NextRequest } from 'next/server';

describe('POST /api/inbox/webhook', () => {
    it('returns Ambassador reply for open hours', async () => {
        const req = new NextRequest('http://localhost/api/inbox/webhook', {
            method: 'POST',
            body: JSON.stringify({ message: 'are you open', tenantId: 'test' })
        });
        const res = await POST(req);
        const data = await res.json();

        expect(res.status).toBe(200);
        expect(data.agent).toBe('The Ambassador');
        expect(data.reply).toContain('we are open until 6 PM');
    });

    it('returns Ambassador reply for pricing', async () => {
        const req = new NextRequest('http://localhost/api/inbox/webhook', {
            method: 'POST',
            body: JSON.stringify({ message: 'what is the price of the cake', tenantId: 'test' })
        });
        const res = await POST(req);
        const data = await res.json();

        expect(res.status).toBe(200);
        expect(data.agent).toBe('The Ambassador');
        expect(data.reply).toContain('$4.99');
    });
});
