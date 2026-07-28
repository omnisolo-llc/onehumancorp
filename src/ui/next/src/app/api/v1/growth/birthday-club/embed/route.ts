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
    const discountAmount = searchParams.get('discount') || '15';
    const hideBranding = searchParams.get('hideBranding') === 'true';

    const escapedTenant = escapeHtml(tenant);
    const encodedTenant = encodeURIComponent(tenant);
    const escapedDiscountAmount = escapeHtml(discountAmount);

    const html = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Join Birthday Club</title>
    <style>
        body { margin: 0; padding: 16px; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; background: transparent; box-sizing: border-box; }
        .widget-card {
            display: flex; flex-direction: column; align-items: center; text-align: center;
            background: #ffffff; color: #111827;
            border: 1px solid #e5e7eb; border-radius: 16px;
            box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
            max-width: 320px; width: 100%; box-sizing: border-box; overflow: hidden; padding: 24px;
        }
        .icon-container {
            font-size: 48px; margin-bottom: 16px;
        }
        .title { font-size: 20px; font-weight: 700; margin: 0 0 8px 0; line-height: 1.2; }
        .description { font-size: 14px; color: #6b7280; line-height: 1.5; margin: 0 0 20px 0; }
        .discount-highlight { color: #f59e0b; font-weight: 700; font-size: 16px; }
        input[type="email"], input[type="date"] {
            width: 100%; padding: 12px; margin-bottom: 12px;
            border: 1px solid #d1d5db; border-radius: 8px; font-size: 14px; box-sizing: border-box;
        }
        .join-button {
            display: block; width: 100%; padding: 12px 0;
            background: #0066FF; color: #ffffff; text-align: center; text-decoration: none;
            font-size: 14px; font-weight: 600; border-radius: 8px; border: none; cursor: pointer;
            transition: background 0.2s ease-in-out;
        }
        .join-button:hover { background: #005CE6; }
        .footer { text-align: center; font-size: 11px; margin-top: 16px; padding-top: 16px; border-top: 1px solid #e5e7eb; width: 100%; }
        .footer a { color: #6b7280; text-decoration: none; font-weight: 600; opacity: 0.8; transition: opacity 0.2s ease-in-out; }
        .footer a:hover { text-decoration: underline; opacity: 1; }
    </style>
</head>
<body>
    <div class="widget-card">
        <div class="icon-container">🎂</div>
        <h3 class="title">Join Our Birthday Club</h3>
        <p class="description">Sign up to get a special <span class="discount-highlight">${escapedDiscountAmount}% OFF</span> discount code on your special day!</p>
        <form style="width: 100%;">
            <input type="email" placeholder="Email Address" required />
            <input type="date" required />
            <button type="submit" class="join-button">Join the Club</button>
        </form>
        ${!hideBranding ? `
        <div class="footer">
            <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodedTenant}&source=birthday_club" target="_blank">⚡ Powered by OHC</a>
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
