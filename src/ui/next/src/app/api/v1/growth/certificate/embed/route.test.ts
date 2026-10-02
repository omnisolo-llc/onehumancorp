import { describe, expect, it } from 'vitest';
import { NextRequest } from 'next/server';
import { JSDOM } from 'jsdom';
import { GET } from './route';

async function renderCertificate(parameters: Record<string, string>) {
  const query = new URLSearchParams(parameters);
  const response = await GET(new NextRequest(`https://business.example/api/v1/growth/certificate/embed?${query}`));
  expect(response.status).toBe(200);
  expect(response.headers.get('content-type')).toBe('text/html; charset=utf-8');
  return new JSDOM(await response.text(), { url: 'https://business.example/' });
}

describe('certificate embed HTML', () => {
  it('renders malicious title, recipient and course markup as literal text', async () => {
    const title = '</title><script>alert("title")</script> & "Achievement"';
    const recipient = '<img src=x onerror="alert(1)"> O\'Reilly & Co';
    const course = '<svg onload="alert(2)"></svg> "Course" & <b>Test</b>';
    const dom = await renderCertificate({ title, recipient, course });
    try {
      const { document } = dom.window;
      expect(document.querySelector('script, img, svg, b, [onerror], [onload]')).toBeNull();
      expect(document.title).toBe(title);
      expect(document.querySelector('h1')?.textContent).toBe(title);
      expect(document.querySelector('.recipient')?.textContent).toBe(recipient);
      expect(document.querySelector('h2')?.textContent).toBe(course);
    } finally { dom.window.close(); }
  });

  it('escapes the legacy course_name alias without changing its displayed text', async () => {
    const course = '<img src=x onerror="alert(1)"> \'Legacy\' & "Course"';
    const dom = await renderCertificate({ course_name: course });
    try {
      expect(dom.window.document.querySelector('img, [onerror]')).toBeNull();
      expect(dom.window.document.querySelector('h2')?.textContent).toBe(course);
    } finally { dom.window.close(); }
  });

  it('keeps the referral tenant in a single encoded same-origin query parameter', async () => {
    const tenant = 'tenant"&ref=wrong\' onclick="alert(3)';
    const dom = await renderCertificate({ tenant, title: 'Baking & Pastry', recipient: 'Jane Doe', course: 'Level 1' });
    try {
      const link = dom.window.document.querySelector('footer a');
      const url = new URL(link.href);
      expect(url.origin).toBe('https://business.example');
      expect(url.pathname).toBe('/onboarding');
      expect([...url.searchParams]).toEqual([['ref', tenant]]);
      expect(link.hasAttribute('onclick')).toBe(false);
      expect(dom.window.document.querySelector('h1')?.textContent).toBe('Baking & Pastry');
    } finally { dom.window.close(); }
  });
});
