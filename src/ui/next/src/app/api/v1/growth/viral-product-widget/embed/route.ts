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
    const productName = searchParams.get('productName') || 'Product Name';
    const price = searchParams.get('price') || '$0.00';
    const description = searchParams.get('description') || 'Product description';
    const imageUrl = searchParams.get('imageUrl') || 'https://images.unsplash.com/photo-1559525839-b184a4d698c7?ixlib=rb-4.0.3&auto=format&fit=crop&w=800&q=80';
    const theme = searchParams.get('theme') || 'light';
    const rawBranding = searchParams.get('branding') !== 'false';

    const escapedTenant = escapeHtml(tenant);
    const encodedTenant = encodeURIComponent(tenant);
    const isDark = theme === 'dark';

    const bg = isDark ? '#1D1D1F' : '#ffffff';
    const text = isDark ? '#ffffff' : '#111827';
    const textSecondary = isDark ? '#a1a1aa' : '#6b7280';
    const border = isDark ? '#333333' : '#e5e7eb';
    const buttonBg = '#0066FF';
    const buttonHoverBg = '#005CE6';

    const html = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>${escapeHtml(productName)}</title>
    <style>
        body { margin: 0; padding: 16px; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; background: transparent; box-sizing: border-box; }
        .widget-card {
            display: flex; flex-direction: column;
            background: ${bg}; color: ${text};
            border: 1px solid ${border}; border-radius: 16px;
            box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
            max-width: 320px; width: 100%; box-sizing: border-box; overflow: hidden;
            transition: transform 0.2s ease-in-out, box-shadow 0.2s ease-in-out;
        }
        .widget-card:hover {
            transform: translateY(-2px);
            box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
        }
        .image-container {
            width: 100%; height: 200px; overflow: hidden;
        }
        .image-container img {
            width: 100%; height: 100%; object-fit: cover;
        }
        .content-container {
            padding: 20px; display: flex; flex-direction: column; gap: 8px;
        }
        .title { font-size: 18px; font-weight: 700; margin: 0; line-height: 1.2; }
        .price { font-size: 16px; font-weight: 600; color: ${buttonBg}; margin: 0; }
        .description { font-size: 13px; color: ${textSecondary}; line-height: 1.5; margin: 0; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; overflow: hidden; text-overflow: ellipsis; }
        .buy-button {
            display: block; width: 100%; padding: 12px 0; margin-top: 12px;
            background: ${buttonBg}; color: #ffffff; text-align: center; text-decoration: none;
            font-size: 14px; font-weight: 600; border-radius: 8px; border: none; cursor: pointer;
            transition: background 0.2s ease-in-out;
        }
        .buy-button:hover { background: ${buttonHoverBg}; }
        .footer { text-align: center; font-size: 11px; margin-top: 12px; padding-top: 12px; border-top: 1px solid ${border}; }
        .footer a { color: ${textSecondary}; text-decoration: none; font-weight: 600; opacity: 0.8; transition: opacity 0.2s ease-in-out; }
        .footer a:hover { text-decoration: underline; opacity: 1; }
    </style>
</head>
<body>
    <div class="widget-card">
        <div class="image-container">
            <img src="${escapeHtml(imageUrl)}" alt="${escapeHtml(productName)}" onerror="this.src='https://images.unsplash.com/photo-1559525839-b184a4d698c7?ixlib=rb-4.0.3&auto=format&fit=crop&w=800&q=80'" />
        </div>
        <div class="content-container">
            <h3 class="title">${escapeHtml(productName)}</h3>
            <p class="price">${escapeHtml(price)}</p>
            <p class="description">${escapeHtml(description)}</p>
            <a href="/checkout?tenant=${encodedTenant}&product=${encodeURIComponent(productName)}" target="_top" class="buy-button">Buy Now</a>
            ${rawBranding ? `
            <div class="footer">
                <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodedTenant}&source=viral_product_widget" target="_blank">⚡ Powered by OHC</a>
            </div>
            ` : ''}
        </div>
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
