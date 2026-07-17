import { NextResponse } from 'next/server';
import { proxyBackendRequest } from '@/lib/auth/backendTransport';

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
    const target = searchParams.get('target') || '500';
    const reward = searchParams.get('reward') || '50% off for everyone!';

    let current = 0;
    try {
        const res = await proxyBackendRequest(request, '/api/v1/growth/referrals/stats', {
            forwardQuery: false,
            suppressRequestBody: true,
        });
        if (res.ok) {
           const data = await res.json();
           if (data && data.metrics && data.metrics.total_referrals !== undefined) {
               current = data.metrics.total_referrals;
           } else if (data && data.invites_sent !== undefined) {
               current = data.invites_sent;
           }
        }
    } catch {
        return new NextResponse('Backend service unavailable', { status: 502 });
    }

    const percentage = Math.min(100, Math.round((current / parseInt(target, 10)) * 100));

    const trackingUrl = '/api/v1/growth/referrals/click?target=/onboarding';

    const html = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Community Goal</title>
    <style>
        body { margin: 0; padding: 12px; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; display: flex; flex-direction: column; background: transparent; }
        .widget {
            display: flex; flex-direction: column; padding: 20px;
            background: #ffffff; color: #111827;
            border: 1px solid #e5e7eb; border-radius: 12px;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
            max-width: 400px; width: 100%; box-sizing: border-box; margin: auto; text-align: center;
        }
        .title { font-size: 18px; font-weight: 700; margin-bottom: 12px; }
        .progress-bar-container {
            width: 100%; height: 12px; background: #e5e5ea; border-radius: 6px; overflow: hidden; margin-bottom: 8px;
        }
        .progress-bar-fill {
            height: 100%; background: #0066FF; width: ${percentage}%; border-radius: 6px; transition: width 0.5s ease-out;
        }
        .content { font-size: 13px; color: #6b7280; margin: 0; line-height: 1.5; }
        .reward { color: #111827; font-weight: 700; }
        .footer { text-align: center; margin-top: 16px; }
        .footer a {
            display: inline-block; padding: 8px 16px; background: #0066FF; color: white; border-radius: 100px; text-decoration: none; font-size: 14px; font-weight: 500;
            transition: transform 0.1s;
        }
        .footer a:active { transform: scale(0.98); }
    </style>
</head>
<body>
    <div class="widget">
        <div class="title">Help Us Reach Our Goal!</div>
        <div class="progress-bar-container">
            <div class="progress-bar-fill"></div>
        </div>
        <div class="content">
            <span id="current-count">${escapeHtml(current.toString())}</span> / <span id="target-count">${escapeHtml(target)}</span> supporters<br>
            <strong class="reward">Reward: ${escapeHtml(reward)}</strong>
        </div>
        <div class="footer">
            <a href="${trackingUrl}" target="_blank">Share to support!</a>
        </div>
    </div>
</body>
</html>
    `;

    return new NextResponse(html, {
        headers: {
            'Content-Type': 'text/html; charset=utf-8',
            'Cache-Control': 'private, no-store'
        },
    });
}
