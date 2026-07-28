import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenant = searchParams.get('tenant') || 'default-team';
  const theme = searchParams.get('theme') || 'light';
  const title = searchParams.get('title') || 'Build your daily streak!';
  const goal = parseInt(searchParams.get('goal') || '7', 10);
  const reward = searchParams.get('reward') || 'a mystery discount';
  const showBranding = searchParams.get('branding') !== 'false';

  const isDark = theme === 'dark';
  const encodedTenant = encodeURIComponent(tenant);
  const bgColor = isDark ? '#111827' : '#ffffff';
  const textColor = isDark ? '#ffffff' : '#1f2937';
  const subTextColor = isDark ? '#9ca3af' : '#6b7280';
  const borderColor = isDark ? '#374151' : '#e5e7eb';
  const circleBg = isDark ? '#1f2937' : '#f3f4f6';

  let daysHtml = '';
  for (let i = 0; i < goal; i++) {
    const isCompleted = i < 3;
    const isToday = i === 3;
    let circleClasses = `width: 32px; height: 32px; border-radius: 50%; display: flex; align-items: center; justify-content: center; font-size: 12px; font-weight: bold; transition: all 0.2s;`;

    if (isCompleted) {
        circleClasses += ` background-color: #f97316; color: #ffffff; box-shadow: 0 4px 6px -1px rgba(249, 115, 22, 0.3);`;
    } else if (isToday) {
        circleClasses += ` border: 2px solid #f97316; color: #f97316;`;
    } else {
        circleClasses += ` background-color: ${circleBg}; color: ${subTextColor};`;
    }

    daysHtml += `
        <div style="display: flex; flex-direction: column; align-items: center; gap: 4px;">
            <div style="${circleClasses}">
                ${isCompleted ? '✓' : (i + 1)}
            </div>
            ${(i + 1) === goal ? `<div style="font-size: 10px; text-transform: uppercase; font-weight: bold; color: #f97316; margin-top: 4px;">Reward</div>` : ''}
        </div>
    `;
  }

  const html = `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@600;700;800&family=Inter:wght@400;500;600&display=swap" rel="stylesheet">
    <style>
        body {
            margin: 0;
            padding: 0;
            font-family: 'Inter', sans-serif;
            background: transparent;
            display: flex;
            align-items: center;
            justify-content: center;
            min-height: 100vh;
        }
        .widget-container {
            width: 100%;
            max-width: 384px;
            background-color: ${bgColor};
            color: ${textColor};
            border: 1px solid ${borderColor};
            border-radius: 16px;
            padding: 24px;
            box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
            box-sizing: border-box;
        }
        .header-icon {
            display: inline-block;
            padding: 16px;
            border-radius: 50%;
            background: linear-gradient(to top right, #fbbf24, #f97316);
            color: white;
            margin-bottom: 16px;
            box-shadow: inset 0 2px 4px 0 rgba(0,0,0,0.06);
        }
        .header-icon svg {
            width: 32px;
            height: 32px;
        }
        .title {
            font-family: 'Outfit', sans-serif;
            font-size: 24px;
            font-weight: 700;
            margin: 0 0 4px 0;
        }
        .subtitle {
            font-size: 14px;
            color: ${subTextColor};
            margin: 0 0 24px 0;
        }
        .days-container {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 24px;
        }
        .claim-btn {
            width: 100%;
            padding: 12px;
            background-color: #f97316;
            color: white;
            border: none;
            border-radius: 12px;
            font-size: 16px;
            font-weight: 700;
            cursor: pointer;
            transition: background-color 0.2s, transform 0.1s;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
        }
        .claim-btn:hover {
            background-color: #ea580c;
        }
        .claim-btn:active {
            transform: scale(0.98);
        }
        .branding {
            display: block;
            text-align: center;
            margin-top: 24px;
            font-size: 11px;
            font-weight: 600;
            color: ${subTextColor};
            text-decoration: none;
            letter-spacing: 0.05em;
            transition: color 0.2s;
        }
        .branding:hover {
            color: #f97316;
        }
    </style>
</head>
<body>
    <div class="widget-container">
        <div style="text-align: center;">
            <div class="header-icon">
                <svg fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M12.395 2.553a1 1 0 00-1.45-.385c-.345.23-.614.558-.822.88-.214.33-.403.713-.57 1.116-.334.804-.614 1.768-.84 2.734a31.365 31.365 0 00-.613 3.58 2.64 2.64 0 01-.945-1.067c-.328-.68-.398-1.534-.398-2.654A1 1 0 005.05 6.05 6.981 6.981 0 003 11a7 7 0 1011.95-4.95c-.592-.591-.98-.985-1.348-1.467-.363-.476-.724-1.063-1.207-2.03zM12.12 15.12A3 3 0 017 13s.879.5 2.5.5c0-1 .5-4 1.25-4.5.5 1 .786 1.293 1.371 1.879A2.99 2.99 0 0113 13a2.99 2.99 0 01-.879 2.121z" clip-rule="evenodd"></path></svg>
            </div>
            <h3 class="title">${title}</h3>
            <p class="subtitle">Hit ${goal} days to unlock ${reward}</p>
        </div>

        <div class="days-container">
            ${daysHtml}
        </div>

        <button class="claim-btn">
            Claim Today's Streak
        </button>

        ${showBranding ? `
            <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodedTenant}&source=viral_streak_widget" target="_blank" class="branding">
                ⚡ Powered by OHC
            </a>
        ` : ''}
    </div>
</body>
</html>`;

  return new NextResponse(html, {
    headers: {
      'Content-Type': 'text/html; charset=utf-8',
      'Cache-Control': 'public, max-age=3600',
    },
  });
}
