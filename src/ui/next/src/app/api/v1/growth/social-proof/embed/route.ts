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
    const product = searchParams.get('product') || 'A product';
    const location = searchParams.get('location') || 'Someone';
    const time = searchParams.get('time') || 'just now';
    const theme = searchParams.get('theme') || 'light';
    const rawBranding = searchParams.get('branding') !== 'false';

    const escapedTenant = escapeHtml(tenant);
    const encodedTenant = encodeURIComponent(tenant);
    const isDark = theme === 'dark';

    const bg = isDark ? '#1D1D1F' : '#ffffff';
    const text = isDark ? '#ffffff' : '#111827';
    const border = isDark ? '#333333' : '#e5e7eb';
    const iconBg = isDark ? '#3730a3' : '#e0e7ff';

    const html = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Social Proof Nudge</title>
    <style>
        body { margin: 0; padding: 0; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; display: flex; flex-direction: column; height: 100vh; background: transparent; }
        .widget {
            display: flex; align-items: center; gap: 16px; padding: 16px;
            background: ${bg}; color: ${text};
            border: 1px solid ${border}; border-radius: 12px;
            box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
            max-width: 400px; width: 100%; box-sizing: border-box; margin: auto;
        }
        .icon {
            width: 48px; height: 48px; border-radius: 8px; background: ${iconBg};
            display: flex; align-items: center; justify-content: center; font-size: 20px;
            flex-shrink: 0;
        }
        .content { flex: 1; min-width: 0; }
        .title { font-size: 14px; font-weight: 600; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
        .title span { font-weight: 400; opacity: 0.8; }
        .product { font-size: 14px; font-weight: 700; color: #4f46e5; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
        .footer-row { display: flex; justify-content: space-between; align-items: center; margin-top: 4px; }
        .time { font-size: 12px; opacity: 0.6; font-weight: 500; }
        .verified { display: flex; align-items: center; gap: 4px; opacity: 0.7; }
        .verified span { font-size: 10px; text-transform: uppercase; font-weight: 700; letter-spacing: 0.1em; color: #34C759; }
        .verified svg { width: 12px; height: 12px; color: #34C759; }
        .footer { text-align: center; font-size: 12px; margin-top: 8px; padding-bottom: 8px; }
        .footer a { color: #6b7280; text-decoration: none; font-weight: 600; }
        .footer a:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <div class="widget">
        <div class="icon">🛍️</div>
        <div class="content">
            <div class="title">${escapeHtml(location)} <span>purchased</span></div>
            <div class="product">${escapeHtml(product)}</div>
            <div class="footer-row">
                <div class="time">${escapeHtml(time)}</div>
                <div class="verified">
                    <span>Verified</span>
                    <svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path></svg>
                </div>
            </div>
        </div>
    </div>
    ${rawBranding ? `
    <div class="footer">
        <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodedTenant}" target="_blank">⚡ Powered by OHC</a>
    </div>
    ` : ''}
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
