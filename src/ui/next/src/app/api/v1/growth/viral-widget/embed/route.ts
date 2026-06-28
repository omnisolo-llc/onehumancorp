import { NextResponse } from 'next/server';

function escapeHtml(unsafe: string) {
    if (!unsafe) return unsafe;
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
    const title = searchParams.get('title') || 'Widget';
    const theme = searchParams.get('theme') || 'light';
    const rawBranding = searchParams.get('branding') !== 'false';

    const escapedTenant = escapeHtml(tenant);
    const encodedTenant = encodeURIComponent(tenant);
    const isDark = theme === 'dark';

    const bg = isDark ? '#1D1D1F' : '#ffffff';
    const text = isDark ? '#ffffff' : '#111827';
    const border = isDark ? '#333333' : '#e5e7eb';

    const html = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>${escapeHtml(title)}</title>
    <style>
        body { margin: 0; padding: 20px; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; display: flex; flex-direction: column; height: 100vh; background: transparent; }
        .widget {
            display: flex; flex-direction: column; padding: 24px;
            background: ${bg}; color: ${text};
            border: 1px solid ${border}; border-radius: 12px;
            box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
            max-width: 400px; width: 100%; box-sizing: border-box; margin: auto; text-align: center;
        }
        .title { font-size: 20px; font-weight: 700; margin-bottom: 16px; }
        .content { flex: 1; font-size: 14px; opacity: 0.8; line-height: 1.5; }
        .footer { text-align: center; font-size: 12px; margin-top: auto; padding-top: 16px; }
        .footer a { color: #6b7280; text-decoration: none; font-weight: 600; }
        .footer a:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <div class="widget">
        <div class="title">${escapeHtml(title)}</div>
        <div class="content">This is a dynamic widget content.</div>
        ${rawBranding ? `
        <div class="footer">
            <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodedTenant}" target="_blank">⚡ Powered by OHC</a>
        </div>
        ` : ''}
    </div>
</body>
</html>
    `;

    return new NextResponse(html, {
        headers: {
            'Content-Type': 'text/html; charset=utf-8',
            'Cache-Control': 'public, max-age=300, s-maxage=300'
        },
    });
}
