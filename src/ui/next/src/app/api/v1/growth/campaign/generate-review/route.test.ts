import { POST } from './route';

describe('POST /api/v1/growth/campaign/generate-review', () => {
    it('returns a generated review request', async () => {
        const req = new Request('http://localhost/api/v1/growth/campaign/generate-review', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ customer_name: 'Alice', product_name: 'Super Gadget', order_id: 'ord_123' })
        });
        const response = await POST(req);
        const json = await response.json();
        expect(response.status).toBe(200);
        expect(json.message).toContain('Alice');
        expect(json.message).toContain('Super Gadget');
        expect(json.message).toContain('ord_123');
    });

    it('returns a fallback message on missing fields', async () => {
        const req = new Request('http://localhost/api/v1/growth/campaign/generate-review', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({})
        });
        const response = await POST(req);
        const json = await response.json();
        expect(response.status).toBe(200);
        expect(json.message).toContain('Hi there');
        expect(json.message).toContain('recent purchase');
        expect(json.message).toContain('12345');
    });
});
