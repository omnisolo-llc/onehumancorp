import { POST } from './route';

describe('POST /api/v1/growth/promotions/generate', () => {
    it('returns a generated promotion', async () => {
        const req = new Request('http://localhost/api/v1/growth/promotions/generate', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ tenant: 'SuperStore' })
        });
        const response = await POST(req);
        const json = await response.json();
        expect(response.status).toBe(200);
        expect(json.message).toContain('SuperStore');
        expect(json.message).toContain('SPECIAL15');
    });

    it('returns a fallback message on missing fields', async () => {
        const req = new Request('http://localhost/api/v1/growth/promotions/generate', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({})
        });
        const response = await POST(req);
        const json = await response.json();
        expect(response.status).toBe(200);
        expect(json.message).toContain('our store');
        expect(json.message).toContain('SPECIAL15');
    });
});
