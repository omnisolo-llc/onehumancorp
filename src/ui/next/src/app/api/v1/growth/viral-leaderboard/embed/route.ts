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
    const title = searchParams.get('title') || 'Top Referrers';
    const metric = searchParams.get('metric') || 'referrers';
    const theme = searchParams.get('theme') || 'light';
    const rawBranding = searchParams.get('branding') !== 'false';

    const escapedTitle = escapeHtml(title);
    const encodedTenant = encodeURIComponent(tenant);
    const isDark = theme === 'dark';

    const bg = isDark ? '#1D1D1F' : '#ffffff';
    const text = isDark ? '#ffffff' : '#111827';
    const textSecondary = isDark ? '#a1a1aa' : '#6b7280';
    const border = isDark ? '#333333' : '#e5e7eb';
    const rowBg = isDark ? '#27272a' : '#f9fafb';

    const backendUrl = process.env.OHC_CORE_URL || 'http://127.0.0.1:18789';
    let leaderboardData: any[] = [];
    try {
        const res = await fetch(`${backendUrl}/api/v1/growth/viral-leaderboard/data?tenant=${encodedTenant}&metric=${encodeURIComponent(metric)}`);
        if (!res.ok) {
            return new NextResponse("Backend service unavailable", { status: 502 });
        }
        leaderboardData = await res.json();
    } catch (e) {
        return new NextResponse("Backend service unavailable", { status: 502 });
    }

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
            margin-bottom: 4px;
        }
        .subtitle {
            font-size: 13px;
            color: ${textSecondary};
        }
        .leaderboard {
            display: flex;
            flex-direction: column;
            gap: 12px;
        }
        .row {
            display: flex;
            align-items: center;
            padding: 12px 16px;
            background: ${rowBg};
            border-radius: 12px;
            border: 1px solid ${border};
        }
        .rank {
            font-size: 20px;
            margin-right: 12px;
            min-width: 24px;
            text-align: center;
        }
        .info {
            flex: 1;
        }
        .name {
            font-weight: 600;
            font-size: 14px;
            margin-bottom: 2px;
        }
        .score {
            font-size: 12px;
            color: ${textSecondary};
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
    </style>
</head>
<body>
    <div class="widget">
        <div class="header">
            <div class="title">${escapedTitle}</div>
            <div class="subtitle">Monthly Top Performers</div>
        </div>
        <div class="leaderboard">
            ${Array.isArray(leaderboardData) ? leaderboardData.map(row => `
                <div class="row">
                    <div class="rank">${escapeHtml(row.emoji || '⭐')}</div>
                    <div class="info">
                        <div class="name">${escapeHtml(row.name)}</div>
                        <div class="score">${escapeHtml(row.score)}</div>
                    </div>
                </div>
            `).join('') : ''}
        </div>
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
