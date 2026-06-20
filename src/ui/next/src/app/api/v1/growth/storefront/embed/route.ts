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
    const tenant = searchParams.get('tenant');
    const theme = searchParams.get('theme') || 'light';

    if (!tenant) {
        return new NextResponse('Missing tenant', { status: 400 });
    }

    const safeTenant = escapeHtml(encodeURIComponent(tenant));
    const isDark = theme === 'dark';

    const html = `<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Storefront</title>
    <style>
        body { font-family: sans-serif; margin: 0; padding: 16px; background: ${isDark ? '#1a1a1a' : '#fff'}; color: ${isDark ? '#fff' : '#000'}; }
        .buy-button { background: #007bff; color: white; padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer; }
        .footer { margin-top: 24px; font-size: 12px; color: #666; text-align: center; }
        .footer a { color: #007bff; text-decoration: none; }
    </style>
</head>
<body>
    <h2>Our Store</h2>
    <p>Welcome to our online store.</p>
    <button class="buy-button">Buy Now</button>

    <div class="footer">
        Powered by <a href="https://ohc.app?ref=${safeTenant}" target="_blank">OHC</a>
    </div>
</body>
</html>`;

    return new NextResponse(html, {
        headers: {
            'Content-Type': 'text/html',
        },
    });
}
