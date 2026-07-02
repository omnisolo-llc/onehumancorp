import { NextRequest, NextResponse } from 'next/server';

function escapeHtml(unsafe: string) {
  return unsafe
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

export async function GET(request: NextRequest) {
  const searchParams = request.nextUrl.searchParams;
  const tenant = escapeHtml(searchParams.get('tenant') || 'my-business');
  const theme = escapeHtml(searchParams.get('theme') || 'light');
  const label = escapeHtml(searchParams.get('label') || 'Metric');
  const value = escapeHtml(searchParams.get('value') || '0');
  const branding = searchParams.get('branding') !== 'false';
  const origin = request.nextUrl.origin || 'https://ohc.app';

  const bgColor = theme === 'dark' ? '#1f2937' : '#ffffff';
  const textColor = theme === 'dark' ? '#f3f4f6' : '#111827';
  const borderColor = theme === 'dark' ? '#374151' : '#e5e7eb';
  const labelColor = theme === 'dark' ? '#9ca3af' : '#6b7280';
  const poweredByColor = theme === 'dark' ? '#d1d5db' : '#6b7280';

  const html = `
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=Outfit:wght@500;700&display=swap" rel="stylesheet">
        <style>
          body {
            margin: 0;
            padding: 0;
            font-family: 'Inter', sans-serif;
            background-color: transparent;
          }
          .widget-container {
            background-color: ${bgColor};
            color: ${textColor};
            border: 1px solid ${borderColor};
            border-radius: 12px;
            padding: 24px;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
            display: flex;
            flex-direction: column;
            justify-content: center;
            height: 100%;
            box-sizing: border-box;
          }
          .metric-label {
            font-size: 14px;
            font-weight: 500;
            color: ${labelColor};
            text-transform: uppercase;
            letter-spacing: 0.05em;
            margin-bottom: 8px;
          }
          .metric-value {
            font-family: 'Outfit', sans-serif;
            font-size: 36px;
            font-weight: 700;
            margin: 0;
          }
          .powered-by {
            margin-top: 16px;
            padding-top: 16px;
            border-top: 1px solid ${borderColor};
            text-align: center;
          }
          .powered-by a {
            color: ${poweredByColor};
            text-decoration: none;
            font-size: 12px;
            font-weight: 600;
          }
          .powered-by a:hover {
            text-decoration: underline;
          }
        </style>
      </head>
      <body>
        <div class="widget-container">
          <div class="metric-label">${label}</div>
          <div class="metric-value">${value}</div>
          ${branding ? `
          <div class="powered-by">
            <a href="${origin}/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}" target="_blank" rel="noreferrer">
              ⚡ Powered by OHC
            </a>
          </div>
          ` : ''}
        </div>
      </body>
    </html>
  `;

  return new NextResponse(html, {
    headers: {
      'Content-Type': 'text/html; charset=utf-8',
      'Cache-Control': 'public, max-age=3600, s-maxage=3600, stale-while-revalidate=86400',
      'Content-Security-Policy': "default-src 'none'; style-src 'unsafe-inline' https://fonts.googleapis.com; font-src https://fonts.gstatic.com;"
    },
  });
}
