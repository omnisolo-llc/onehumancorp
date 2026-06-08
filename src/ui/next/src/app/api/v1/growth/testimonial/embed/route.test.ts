import { describe, it, expect } from 'vitest';
import { GET } from './route';

describe('GET /api/v1/growth/testimonial/embed', () => {
    it('returns a testimonial widget with default parameters', async () => {
        const req = new Request('http://localhost/api/v1/growth/testimonial/embed');
        const res = await GET(req);

        expect(res.status).toBe(200);
        const html = await res.text();

        expect(html).toContain('Customer Testimonial');
        expect(html).toContain('Happy Customer');
        expect(html).toContain('This is the best service');
        expect(html).toContain('★★★★★');
        expect(html).toContain('⚡ Powered by OHC');
        expect(html).toContain('https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=demo');
    });

    it('customizes the widget based on query parameters', async () => {
        const req = new Request('http://localhost/api/v1/growth/testimonial/embed?tenant=acme-corp&authorName=John%20Smith&reviewText=Amazing%20job!&rating=3&theme=dark');
        const res = await GET(req);

        expect(res.status).toBe(200);
        const html = await res.text();

        expect(html).toContain('John Smith');
        expect(html).toContain('Amazing job!');
        expect(html).toContain('★★★☆☆');
        expect(html).toContain('background: #1f2937'); // dark theme
        expect(html).toContain('ref=acme-corp');
    });

    it('escapes XSS payloads in query parameters', async () => {
        const req = new Request('http://localhost/api/v1/growth/testimonial/embed?tenant=my-shop&authorName=%3Cscript%3Ealert(1)%3C/script%3E&reviewText=%3Cimg%20src=x%20onerror=alert(1)%3E&rating=5');
        const res = await GET(req);

        expect(res.status).toBe(200);
        const html = await res.text();

        expect(html).not.toContain('<script>');
        expect(html).toContain('&lt;script&gt;alert(1)&lt;/script&gt;');
        expect(html).not.toContain('<img');
        expect(html).toContain('&lt;img src=x onerror=alert(1)&gt;');
    });

    it('caps the rating between 1 and 5', async () => {
        const reqHigh = new Request('http://localhost/api/v1/growth/testimonial/embed?rating=10');
        const resHigh = await GET(reqHigh);
        expect(await resHigh.text()).toContain('★★★★★');

        const reqLow = new Request('http://localhost/api/v1/growth/testimonial/embed?rating=-2');
        const resLow = await GET(reqLow);
        expect(await resLow.text()).toContain('★☆☆☆☆');

        const reqInvalid = new Request('http://localhost/api/v1/growth/testimonial/embed?rating=abc');
        const resInvalid = await GET(reqInvalid);
        expect(await resInvalid.text()).toContain('★★★★★'); // defaults to 5 if NaN
    });
});
