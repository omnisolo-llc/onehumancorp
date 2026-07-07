import { describe, it, expect } from 'vitest';
import { GET } from './route';

describe('GET /api/v1/growth/event-rsvp/embed', () => {
  it('returns an HTML response with the viral loop link', async () => {
    const req = new Request('http://localhost/api/v1/growth/event-rsvp/embed?tenant=test-tenant&title=My%20Event&date=Tomorrow&location=Zoom');
    const res = await GET(req);

    expect(res.status).toBe(200);
    expect(res.headers.get('content-type')).toBe('text/html; charset=utf-8');

    const html = await res.text();

    // Check for provided parameters
    expect(html).toContain('My Event');
    expect(html).toContain('Tomorrow');
    expect(html).toContain('Zoom');

    // Check for viral loop link
    expect(html).toContain('⚡ Powered by OHC');
    expect(html).toContain('Want to host your own events?');
    expect(html).toContain('href="/api/v1/growth/referrals/click?target=/onboarding&ref=test-tenant&source=viral_event_rsvp_widget"');
  });

  it('removes viral branding when branding=false', async () => {
    const req = new Request('http://localhost/api/v1/growth/event-rsvp/embed?tenant=test-tenant&branding=false');
    const res = await GET(req);

    const html = await res.text();

    // Check that viral loop link is absent
    expect(html).not.toContain('⚡ Powered by OHC');
    expect(html).not.toContain('Want to host your own events?');
  });

  it('escapes html to prevent XSS', async () => {
    const maliciousTitle = '<script>alert("xss")</script>';
    const req = new Request(`http://localhost/api/v1/growth/event-rsvp/embed?tenant=test-tenant&title=${encodeURIComponent(maliciousTitle)}`);
    const res = await GET(req);

    const html = await res.text();

    expect(html).not.toContain('<script>alert("xss")</script>');
    expect(html).toContain('&lt;script&gt;alert(&quot;xss&quot;)&lt;/script&gt;');
  });

  it('renders dark theme styles', async () => {
    const req = new Request('http://localhost/api/v1/growth/event-rsvp/embed?tenant=test-tenant&theme=dark');
    const res = await GET(req);

    const html = await res.text();
    // bg is #1D1D1F for dark theme
    expect(html).toContain('background: #1D1D1F');
  });
});
