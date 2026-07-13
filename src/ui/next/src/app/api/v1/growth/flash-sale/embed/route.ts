import { NextResponse } from 'next/server';

function escapeHtml(unsafe: string) {
    return unsafe
         .replace(/&/g, "&amp;")
         .replace(/</g, "&lt;")
         .replace(/>/g, "&gt;")
         .replace(/"/g, "&quot;")
         .replace(/'/g, "&#039;");
}

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenant = searchParams.get('tenant') || 'demo';
  const title = searchParams.get('title') || 'Flash Sale';
  const code = searchParams.get('code') || 'SALE';
  const percent = searchParams.get('percent') || '0';
  const theme = searchParams.get('theme') || 'light';
  const rawBranding = searchParams.get('branding') !== 'false';

  const html = `
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Flash Sale Widget</title>
      <style>
        body { margin: 0; padding: 0; font-family: sans-serif; display: flex; justify-content: center; align-items: center; min-height: 100vh; background: transparent; }
        .widget {
          width: 100%; max-width: 400px; padding: 20px; border-radius: 12px; text-align: center;
          ${escapeHtml(theme) === 'dark' ? 'background: #111827; color: white;' : 'background: white; color: #111827; box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);'}
        }
        .title { font-size: 1.5rem; font-weight: bold; margin-bottom: 8px; }
        .discount { font-size: 1.2rem; color: #ef4444; font-weight: bold; margin-bottom: 16px; }
        .code {
          background: ${escapeHtml(theme) === 'dark' ? '#374151' : '#f3f4f6'};
          padding: 8px 16px; border-radius: 8px; font-family: monospace; font-weight: bold; letter-spacing: 2px;
          display: inline-block;
        }
        .footer { margin-top: 16px; font-size: 12px; }
        .footer a { color: #6b7280; text-decoration: none; font-weight: bold; }
      </style>
    </head>
    <body>
      <div class="widget">
        <div class="title">${escapeHtml(title)}</div>
        <div class="discount">${escapeHtml(percent)}% OFF</div>
        <div class="code">${escapeHtml(code)}</div>
        ${rawBranding ? `
        <div class="footer">
          <a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${encodeURIComponent(tenant)}" target="_blank">⚡ Powered by OHC</a>
        </div>
        ` : ''}
      </div>
    </body>
    </html>
  `;

  return new NextResponse(html, {
    headers: { 'Content-Type': 'text/html' },
  });
}
