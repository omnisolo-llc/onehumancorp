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
  const rawTitle = searchParams.get('title') || 'My Awesome Tool';
  const title = escapeHtml(rawTitle);
  const theme = searchParams.get('theme') || 'light';
  const branding = searchParams.get('branding') !== 'false';

  const refParam = searchParams.get('ref');
  const referralLink = refParam
    ? `https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${encodeURIComponent(refParam)}`
    : 'https://ohc.app';

  const bgColor = theme === 'dark' ? '#1f2937' : '#ffffff';
  const textColor = theme === 'dark' ? '#f3f4f6' : '#111827';
  const borderColor = theme === 'dark' ? '#374151' : '#e5e7eb';

  const html = `
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Viral Widget Preview</title>
      <style>
        body {
          margin: 0;
          font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
          background-color: ${bgColor};
          color: ${textColor};
          display: flex;
          flex-direction: column;
          align-items: center;
          justify-content: center;
          height: 100vh;
          text-align: center;
          padding: 24px;
          box-sizing: border-box;
        }
        .container {
          border: 1px solid ${borderColor};
          border-radius: 12px;
          padding: 32px;
          max-width: 400px;
          width: 100%;
          box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
        }
        h2 {
          margin-top: 0;
          font-size: 24px;
        }
        .action-button {
          background-color: #4f46e5;
          color: white;
          border: none;
          padding: 12px 24px;
          border-radius: 8px;
          font-size: 16px;
          font-weight: 600;
          cursor: pointer;
          margin-top: 16px;
          width: 100%;
        }
        .action-button:hover {
          background-color: #4338ca;
        }
        .branding {
          position: absolute;
          bottom: 16px;
          right: 16px;
          display: inline-flex;
          align-items: center;
          gap: 6px;
          padding: 6px 12px;
          background: rgba(0,0,0,0.8);
          color: #fff;
          border-radius: 100px;
          font-size: 13px;
          font-weight: 500;
          text-decoration: none;
          box-shadow: 0 4px 12px rgba(0,0,0,0.1);
        }
        .branding svg {
          width: 14px;
          height: 14px;
          fill: currentColor;
        }
      </style>
    </head>
    <body>
      <div class="container">
        <h2>${title}</h2>
        <p style="opacity: 0.8; margin-bottom: 24px;">Join thousands of others getting amazing results.</p>
        <button class="action-button">Get Started Now</button>
      </div>

      ${branding ? `
      <a href="${escapeHtml(referralLink)}" target="_blank" class="branding">
        <svg viewBox="0 0 24 24"><path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/></svg>
        Powered by OHC
      </a>
      ` : ''}
    </body>
    </html>
  `;

  return new NextResponse(html, {
    status: 200,
    headers: { 'Content-Type': 'text/html' },
  });
}
