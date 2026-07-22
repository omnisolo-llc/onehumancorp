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

function inlineScriptValue(value: string): string {
  return JSON.stringify(value).replace(/[<>&\u2028\u2029]/g, (character) => ({
    '<': '\\u003c',
    '>': '\\u003e',
    '&': '\\u0026',
    '\u2028': '\\u2028',
    '\u2029': '\\u2029',
  })[character] ?? character);
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

    const goal = parseInt(goalStr, 10) || 7;

    const encodedTenant = encodeURIComponent(tenant);
    const titleHtml = escapeHtml(title);
    const rewardHtml = escapeHtml(reward);
    const rewardScriptVal = inlineScriptValue(reward);
    const tenantScriptVal = inlineScriptValue(tenant);
    const nonce = randomBytes(16).toString('base64url');

    const isDark = theme === 'dark';
    const bgColor = isDark ? '#111827' : '#ffffff';
    const textColor = isDark ? '#f9fafb' : '#111827';
    const secondaryColor = isDark ? '#9ca3af' : '#6b7280';
    const borderColor = isDark ? '#374151' : '#e5e7eb';
    const accentColor = isDark ? '#3b82f6' : '#2563eb';
    const blockBg = isDark ? '#1f2937' : '#f3f4f6';

    const html = `
      <!DOCTYPE html>
      <html>
      <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <style>
          @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');
          body {
            margin: 0;
            padding: 16px;
            font-family: 'Inter', system-ui, -apple-system, sans-serif;
            background-color: ${bgColor};
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
            border-radius: 12px;
            padding: 20px;
            background-color: ${bgColor};
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
            display: flex;
            flex-direction: column;
            align-items: center;
          }
          .streak-title {
            font-size: 18px;
            font-weight: 600;
            margin-bottom: 8px;
            text-align: center;
          }
          .streak-subtitle {
            font-size: 14px;
            color: ${secondaryColor};
            margin-bottom: 24px;
            text-align: center;
          }
          .streak-days {
            display: flex;
            gap: 8px;
            margin-bottom: 24px;
            flex-wrap: wrap;
            justify-content: center;
          }
          .streak-day {
            width: 40px;
            height: 40px;
            border-radius: 50%;
            background-color: ${blockBg};
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 600;
            font-size: 14px;
            color: ${secondaryColor};
            border: 2px solid transparent;
            transition: all 0.2s ease;
          }
          .streak-day.active {
            background-color: ${accentColor};
            color: white;
            box-shadow: 0 0 10px rgba(59, 130, 246, 0.5);
          }
          .streak-day.completed {
            background-color: #10b981;
            color: white;
          }
          .reward-box {
            background-color: ${blockBg};
            border: 1px dashed ${borderColor};
            border-radius: 8px;
            padding: 16px;
            text-align: center;
            width: 100%;
            box-sizing: border-box;
            margin-bottom: ${showBranding ? '20px' : '0'};
          }
          .reward-title {
            font-size: 12px;
            text-transform: uppercase;
            color: ${secondaryColor};
            font-weight: 600;
            letter-spacing: 0.05em;
            margin-bottom: 4px;
          }
          .reward-value {
            font-size: 16px;
            font-weight: 700;
            color: ${accentColor};
          }
          .check-in-btn {
            background-color: ${accentColor};
            color: white;
            border: none;
            border-radius: 8px;
            padding: 12px 24px;
            font-size: 16px;
            font-weight: 600;
            cursor: pointer;
            transition: opacity 0.2s;
            width: 100%;
            margin-bottom: 16px;
          }
          .check-in-btn:hover {
            opacity: 0.9;
          }
          .check-in-btn:disabled {
            background-color: ${secondaryColor};
            cursor: not-allowed;
            opacity: 0.7;
          }
          .branding {
            font-size: 12px;
            color: ${secondaryColor};
            text-decoration: none;
            display: flex;
            align-items: center;
            gap: 4px;
            transition: color 0.2s;
            padding-top: 12px;
            border-top: 1px solid ${borderColor};
            width: 100%;
            justify-content: center;
          }
          .branding:hover {
            color: ${textColor};
          }
        </style>
      </head>
      <body>
        <div class="widget-container">
          <div class="streak-title">${titleHtml}</div>
          <div class="streak-subtitle">Check in for ${goal} days to unlock your reward!</div>

          <div class="streak-days" id="streak-days">
            ${Array.from({ length: goal }, (_, i) => `
              <div class="streak-day" id="day-${i + 1}">${i + 1}</div>
            `).join('')}
          </div>

          <button class="check-in-btn" id="check-in-btn">Check In Today</button>

          <div class="reward-box">
            <div class="reward-title">Goal Reward</div>
            <div class="reward-value">🎁 ${rewardHtml}</div>
          </div>

          ${showBranding ? `
            <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodedTenant}&source=viral_streak_widget" target="_blank" class="branding">
              ⚡ Powered by OHC
            </a>
          ` : ''}
        </div>

        <script nonce="${nonce}">
          const goal = ${goal};
          const tenantId = ${tenantScriptVal};
          const rewardVal = ${rewardScriptVal};

          let currentStreak = parseInt(localStorage.getItem('streak_' + encodeURIComponent(tenantId)) || '0', 10);
          let lastCheckIn = parseInt(localStorage.getItem('last_check_in_' + encodeURIComponent(tenantId)) || '0', 10);

          function updateUI() {
            const btn = document.getElementById('check-in-btn');
            const now = new Date().getTime();
            const oneDay = 24 * 60 * 60 * 1000;

            // Reset if more than 2 days have passed (streak lost)
            if (lastCheckIn > 0 && now - lastCheckIn > oneDay * 2) {
              currentStreak = 0;
            }

            // Already checked in today
            if (lastCheckIn > 0 && now - lastCheckIn < oneDay && new Date(lastCheckIn).getDate() === new Date().getDate()) {
               btn.disabled = true;
               btn.innerText = 'Checked In!';
            } else if (currentStreak >= goal) {
               btn.disabled = true;
               btn.innerText = 'Goal Reached!';
            } else {
               btn.disabled = false;
               btn.innerText = 'Check In Today';
            }

            for (let i = 1; i <= goal; i++) {
              const dayEl = document.getElementById('day-' + i);
              dayEl.className = 'streak-day';
              if (i < currentStreak + 1) {
                dayEl.classList.add('completed');
              } else if (i === currentStreak + 1 && !btn.disabled) {
                dayEl.classList.add('active');
              }
            }
          }

          document.getElementById('check-in-btn').addEventListener('click', () => {
            currentStreak++;
            lastCheckIn = new Date().getTime();
            localStorage.setItem('streak_' + encodeURIComponent(tenantId), currentStreak.toString());
            localStorage.setItem('last_check_in_' + encodeURIComponent(tenantId), lastCheckIn.toString());
            updateUI();

            if (currentStreak >= goal) {
              alert('Congratulations! You reached your streak goal of ' + goal + ' days! Reward unlocked: ' + rewardVal);
            }
          });

          updateUI();
        </script>
      </body>
      </html>
    `;

    return new NextResponse(html, {
      headers: {
        'Content-Type': 'text/html',
        'Cache-Control': 'public, max-age=60', // Cache for 1 minute
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
