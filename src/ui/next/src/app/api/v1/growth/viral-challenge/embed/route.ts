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
    const title = searchParams.get('title') || '30-Day Fitness Challenge';
    const duration = searchParams.get('duration') || '30';
    const reward = searchParams.get('reward') || 'Free T-Shirt';
    const theme = searchParams.get('theme') || 'light';
    const rawBranding = searchParams.get('branding') !== 'false';

    const escapedTitle = escapeHtml(title);
    const escapedReward = escapeHtml(reward);
    const escapedDuration = escapeHtml(duration);
    const encodedTenant = encodeURIComponent(tenant);
    const isDark = theme === 'dark';

    const bg = isDark ? '#1D1D1F' : '#ffffff';
    const text = isDark ? '#ffffff' : '#111827';
    const textSecondary = isDark ? '#a1a1aa' : '#6b7280';
    const border = isDark ? '#333333' : '#e5e7eb';
    const highlight = isDark ? '#3b82f6' : '#2563eb';
    const rowBg = isDark ? '#27272a' : '#f9fafb';

    const html = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>${escapedTitle}</title>
    <style>
        body {
            margin: 0;
            padding: 20px;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            display: flex;
            flex-direction: column;
            height: 100vh;
            background: transparent;
        }
        .widget {
            display: flex;
            flex-direction: column;
            padding: 24px;
            background: ${bg};
            color: ${text};
            border: 1px solid ${border};
            border-radius: 16px;
            box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
            max-width: 400px;
            width: 100%;
            box-sizing: border-box;
            margin: auto;
        }
        .header {
            text-align: center;
            margin-bottom: 20px;
        }
        .title {
            font-size: 20px;
            font-weight: 700;
            margin-bottom: 8px;
        }
        .subtitle {
            font-size: 14px;
            color: ${textSecondary};
            line-height: 1.4;
        }
        .reward-box {
            background: ${rowBg};
            border: 1px solid ${border};
            border-radius: 12px;
            padding: 16px;
            text-align: center;
            margin-bottom: 24px;
        }
        .reward-label {
            font-size: 12px;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            color: ${textSecondary};
            margin-bottom: 4px;
            font-weight: 600;
        }
        .reward-value {
            font-size: 16px;
            font-weight: 700;
            color: ${highlight};
        }
        .button {
            display: block;
            width: 100%;
            padding: 14px;
            background: ${highlight};
            color: white;
            border: none;
            border-radius: 8px;
            font-size: 16px;
            font-weight: 600;
            cursor: pointer;
            text-align: center;
            text-decoration: none;
            box-sizing: border-box;
            transition: opacity 0.2s;
        }
        .button:hover {
            opacity: 0.9;
        }
        .footer {
            text-align: center;
            font-size: 12px;
            margin-top: 24px;
            padding-top: 16px;
            border-top: 1px solid ${border};
        }
        .footer a {
            color: ${textSecondary};
            text-decoration: none;
            font-weight: 600;
            transition: color 0.2s;
        }
        .footer a:hover {
            text-decoration: underline;
            color: ${text};
        }
        .duration-badge {
            display: inline-block;
            background: rgba(59, 130, 246, 0.1);
            color: ${highlight};
            padding: 4px 12px;
            border-radius: 99px;
            font-size: 13px;
            font-weight: 600;
            margin-bottom: 12px;
        }
    </style>
</head>
<body>
    <div class="widget">
        <div class="header">
            <div class="duration-badge">🗓️ ${escapedDuration} Days</div>
            <div class="title">${escapedTitle}</div>
            <div class="subtitle">Join the challenge, build a streak, and earn rewards along the way!</div>
        </div>

        <div class="reward-box">
            <div class="reward-label">Complete to Win</div>
            <div class="reward-value">🎁 ${escapedReward}</div>
        </div>

        <a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${encodedTenant}&source=viral_challenge" target="_blank" class="button">
            Join Challenge
        </a>

        ${rawBranding ? `
        <div class="footer">
            <a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${encodedTenant}&source=viral_challenge" target="_blank" id="preview-branding">⚡ Powered by OHC</a>
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
