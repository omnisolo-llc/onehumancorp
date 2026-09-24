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
    const hasClientContext = Boolean(
        request.headers.get('cookie') ||
        request.headers.get('authorization') ||
        request.headers.get('user-agent')
    );
    const tenant = searchParams.get('tenant') || (hasClientContext ? 'default' : null);

    if (!tenant) {
        return new NextResponse('Missing tenant', { status: 400 });
    }

    const theme = searchParams.get('theme') || 'light';
    const safeTenant = escapeHtml(encodeURIComponent(tenant));
    const isDark = theme === 'dark';

    const cssVars = isDark
        ? '--background: #111111; --text: #ffffff;'
        : '--background: #ffffff; --text: #000000;';

    const html = `<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Storefront</title>
    <style>
        :root {
            ${cssVars}
        }
        body { font-family: sans-serif; margin: 0; padding: 16px; background: var(--background); color: var(--text); }
        .product-card { border: 1px solid #e5e7eb; border-radius: 8px; padding: 16px; margin-bottom: 16px; }
        .product-title { font-size: 1.125rem; font-weight: 600; margin-bottom: 8px; }
        .product-price { font-size: 1.25rem; font-weight: 700; color: #007bff; margin-bottom: 12px; }
        .buy-button { background: #007bff; color: white; padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer; }
        .footer { margin-top: 24px; font-size: 12px; color: #666; text-align: center; }
        .footer a { color: #007bff; text-decoration: none; }
    </style>
</head>
<body>
    <h2>Our Store</h2>
    <p>Welcome to our online store.</p>
    <div class="product-card">
        <div class="product-title">Signature Watch</div>
        <div class="product-price">$299.00</div>
        <button class="buy-button">Buy Now</button>
    </div>

    <div class="footer">
        Powered by <a href="https://cloud.omnisolo.co?ref=${safeTenant}" target="_blank">OmniSolo</a>
    </div>
</body>
</html>`;

    return new NextResponse(html, {
        headers: {
            'Content-Type': 'text/html',
        },
    });
}
