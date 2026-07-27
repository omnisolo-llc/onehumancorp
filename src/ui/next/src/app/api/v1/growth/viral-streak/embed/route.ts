import { NextResponse } from 'next/server';
import { randomBytes } from 'node:crypto';

function escapeHtml(value: string): string {
  return value
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#x27;');
}

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const tenant = searchParams.get('tenant') || 'unknown';
    const theme = searchParams.get('theme') || 'light';
    const title = searchParams.get('title') || 'Daily Login Streak';
    const goalStr = searchParams.get('goal') || '7';
    const reward = searchParams.get('reward') || 'Free Coffee';
    const showBranding = searchParams.get('branding') !== 'false';

    const goal = Math.max(2, Math.min(30, parseInt(goalStr) || 7));

    const encodedTenant = encodeURIComponent(tenant);
    const titleHtml = escapeHtml(title);
    const rewardHtml = escapeHtml(reward);
    const nonce = randomBytes(16).toString('base64url');

    const isDark = theme === 'dark';
    const bgColor = isDark ? '#111827' : '#ffffff';
    const textColor = isDark ? '#f9fafb' : '#111827';
    const secondaryColor = isDark ? '#9ca3af' : '#6b7280';
    const borderColor = isDark ? '#374151' : '#e5e7eb';
    const blockBg = isDark ? '#1f2937' : '#f3f4f6';

    let dotsHtml = '';
    const displayCount = Math.min(goal, 7);
    for (let i = 0; i < displayCount; i++) {
      const dayNum = i + 1;
      let dotClass = 'dot inactive';
      let dotContent = dayNum.toString();

      // Mocking day 1, 2, 3 as active/completed, day 4 as current, rest as inactive
      if (dayNum < 4) {
        dotClass = 'dot active';
        dotContent = '✓';
      } else if (dayNum === 4) {
        dotClass = 'dot current';
      }

      const isLast = dayNum === goal;
      dotsHtml += `
        <div class="dot-col">
          <div class="${dotClass}" id="dot-${dayNum}">${dotContent}</div>
          ${isLast ? '<div class="dot-label">Reward</div>' : ''}
        </div>
      `;
    }

    const html = `
      <!DOCTYPE html>
      <html lang="en">
      <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <title>${titleHtml}</title>
        <style>
          @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');
          body {
            margin: 0;
            padding: 16px;
            font-family: 'Inter', system-ui, -apple-system, sans-serif;
            background-color: transparent;
            color: ${textColor};
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            min-height: 100vh;
            box-sizing: border-box;
          }
          .widget-container {
            width: 100%;
            max-width: 400px;
            border: 1px solid ${borderColor};
            border-radius: 20px;
            padding: 24px;
            background-color: ${bgColor};
            box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 8px 10px -6px rgba(0, 0, 0, 0.1);
            display: flex;
            flex-direction: column;
            align-items: center;
            box-sizing: border-box;
            backdrop-filter: blur(20px);
            -webkit-backdrop-filter: blur(20px);
          }
          .streak-icon-container {
            padding: 16px;
            border-radius: 50%;
            background: linear-gradient(135deg, #f59e0b 0%, #ea580c 100%);
            color: #ffffff;
            box-shadow: inset 0 2px 4px rgba(255, 255, 255, 0.2), 0 4px 10px rgba(234, 88, 12, 0.3);
            margin-bottom: 16px;
            display: flex;
            align-items: center;
            justify-content: center;
          }
          .streak-icon {
            width: 32px;
            height: 32px;
          }
          .streak-title {
            font-size: 20px;
            font-weight: 700;
            margin-bottom: 4px;
            text-align: center;
          }
          .streak-subtitle {
            font-size: 13px;
            color: ${secondaryColor};
            text-align: center;
            margin-bottom: 24px;
          }
          .streak-dots {
            display: flex;
            justify-content: space-between;
            width: 100%;
            gap: 8px;
            margin-bottom: 24px;
            flex-wrap: wrap;
          }
          .dot-col {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 4px;
            flex: 1;
            min-width: 36px;
          }
          .dot {
            width: 32px;
            height: 32px;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 12px;
            font-weight: 700;
            transition: all 0.3s ease;
            box-sizing: border-box;
          }
          .dot.active {
            background-color: #ea580c;
            color: #ffffff;
            box-shadow: 0 4px 6px rgba(234, 88, 12, 0.3);
          }
          .dot.current {
            border: 2px solid #ea580c;
            color: #ea580c;
            background-color: transparent;
          }
          .dot.inactive {
            background-color: ${blockBg};
            color: ${secondaryColor};
            border: 1px solid ${borderColor};
          }
          .dot-label {
            font-size: 10px;
            font-weight: 600;
            text-transform: uppercase;
            color: #ea580c;
          }
          .button {
            display: flex;
            align-items: center;
            justify-content: center;
            width: 100%;
            min-height: 48px;
            background: #ea580c;
            color: white;
            border: none;
            border-radius: 12px;
            font-size: 15px;
            font-weight: 600;
            cursor: pointer;
            text-align: center;
            text-decoration: none;
            box-sizing: border-box;
            transition: background-color 0.2s, transform 0.1s;
            box-shadow: 0 4px 12px rgba(234, 88, 12, 0.3);
          }
          .button:hover {
            background-color: #d97706;
          }
          .button:active {
            transform: scale(0.98);
          }
          .button.claimed {
            background-color: ${blockBg};
            color: ${secondaryColor};
            border: 1px solid ${borderColor};
            box-shadow: none;
            cursor: default;
          }
          .branding {
            font-size: 12px;
            color: ${secondaryColor};
            text-decoration: none;
            display: flex;
            align-items: center;
            gap: 4px;
            transition: color 0.2s;
            padding-top: 16px;
            border-top: 1px solid ${borderColor};
            width: 100%;
            justify-content: center;
            margin-top: 20px;
          }
          .branding:hover {
            color: ${textColor};
          }
        </style>
      </head>
      <body>
        <div class="widget-container">
          <div class="streak-icon-container">
            <svg class="streak-icon" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M12.395 2.553a1 1 0 00-1.45-.385c-.345.23-.614.558-.822.88-.214.33-.403.713-.57 1.116-.334.804-.614 1.768-.84 2.734a31.365 31.365 0 00-.613 3.58 2.64 2.64 0 01-.945-1.067c-.328-.68-.398-1.534-.398-2.654A1 1 0 005.05 6.05 6.981 6.981 0 003 11a7 7 0 1011.95-4.95c-.592-.591-.98-.985-1.348-1.467-.363-.476-.724-1.063-1.207-2.03zM12.12 15.12A3 3 0 017 13s.879.5 2.5.5c0-1 .5-4 1.25-4.5.5 1 .786 1.293 1.371 1.879A2.99 2.99 0 0113 13a2.99 2.99 0 01-.879 2.121z" clip-rule="evenodd"></path>
            </svg>
          </div>

          <div class="streak-title">${titleHtml}</div>
          <div class="streak-subtitle">Hit ${goal} days to unlock ${rewardHtml}</div>

          <div class="streak-dots">
            ${dotsHtml}
          </div>

          <button id="streak-button" class="button">
            Claim Today's Streak
          </button>

          ${showBranding ? `
            <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodedTenant}&source=viral_streak_widget" target="_blank" class="branding">
              ⚡ Powered by OHC
            </a>
          ` : ''}
        </div>

        <script nonce="${nonce}">
          const button = document.getElementById('streak-button');
          button.addEventListener('click', () => {
            if (button.classList.contains('claimed')) return;

            button.classList.add('claimed');
            button.innerText = 'Streak Claimed! ✓';
            button.style.backgroundColor = '${blockBg}';
            button.style.color = '${secondaryColor}';
            button.style.border = '1px solid ${borderColor}';
            button.style.boxShadow = 'none';
            button.style.cursor = 'default';

            // Also mock day 4 dot becoming checked
            const todayDot = document.getElementById('dot-4');
            if (todayDot) {
              todayDot.className = 'dot active';
              todayDot.innerText = '✓';
            }
          });
        </script>
      </body>
      </html>
    `;

    return new NextResponse(html, {
      headers: {
        'Content-Type': 'text/html',
        'Cache-Control': 'public, max-age=60',
        'Content-Security-Policy': `default-src 'none'; style-src 'unsafe-inline' https://fonts.googleapis.com; font-src https://fonts.gstatic.com; script-src 'nonce-${nonce}'; connect-src 'none'; frame-ancestors *; base-uri 'none'; form-action 'none'`,
        'X-Content-Type-Options': 'nosniff',
        'Referrer-Policy': 'no-referrer',
      },
    });
  } catch (error) {
    console.error('Error generating viral streak widget embed:', error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
