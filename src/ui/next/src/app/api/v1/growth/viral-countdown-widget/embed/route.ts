import { NextResponse } from 'next/server';
// import { trackGrowthEvent } from '../../../lib/growth'; // REMOVED

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const tenant = searchParams.get('tenant') || 'unknown';
    const theme = searchParams.get('theme') || 'light';
    const eventName = searchParams.get('event') || 'Event';
    const targetDateStr = searchParams.get('target') || new Date(Date.now() + 86400000).toISOString();
    const showBranding = searchParams.get('branding') !== 'false';

    const encodedTenant = encodeURIComponent(tenant);

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
          .event-title {
            font-size: 18px;
            font-weight: 600;
            margin-bottom: 16px;
            text-align: center;
          }
          .countdown-blocks {
            display: flex;
            gap: 12px;
            margin-bottom: ${showBranding ? '20px' : '0'};
          }
          .time-block {
            display: flex;
            flex-direction: column;
            align-items: center;
            background-color: ${blockBg};
            border-radius: 8px;
            padding: 10px 12px;
            min-width: 48px;
          }
          .time-value {
            font-size: 24px;
            font-weight: 700;
            color: ${accentColor};
            line-height: 1.2;
          }
          .time-label {
            font-size: 10px;
            text-transform: uppercase;
            color: ${secondaryColor};
            font-weight: 600;
            letter-spacing: 0.05em;
            margin-top: 4px;
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
          .expired-message {
            font-size: 16px;
            font-weight: 600;
            color: ${secondaryColor};
            padding: 20px 0;
            display: none;
          }
        </style>
      </head>
      <body>
        <div class="widget-container">
          <div class="event-title">${eventName}</div>
          <div class="countdown-blocks" id="countdown">
            <div class="time-block">
              <span class="time-value" id="days">00</span>
              <span class="time-label">Days</span>
            </div>
            <div class="time-block">
              <span class="time-value" id="hours">00</span>
              <span class="time-label">Hrs</span>
            </div>
            <div class="time-block">
              <span class="time-value" id="minutes">00</span>
              <span class="time-label">Min</span>
            </div>
            <div class="time-block">
              <span class="time-value" id="seconds">00</span>
              <span class="time-label">Sec</span>
            </div>
          </div>
          <div class="expired-message" id="expired">Event has started!</div>
          ${showBranding ? `
            <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodedTenant}&source=viral_countdown_widget" target="_blank" class="branding">
              ⚡ Powered by OHC
            </a>
          ` : ''}
        </div>

        <script>
          function updateCountdown() {
            const target = new Date('${targetDateStr}').getTime();
            const now = new Date().getTime();
            const distance = target - now;

            if (distance < 0) {
              document.getElementById('countdown').style.display = 'none';
              document.getElementById('expired').style.display = 'block';
              return;
            }

            const days = Math.floor(distance / (1000 * 60 * 60 * 24));
            const hours = Math.floor((distance % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
            const minutes = Math.floor((distance % (1000 * 60 * 60)) / (1000 * 60));
            const seconds = Math.floor((distance % (1000 * 60)) / 1000);

            document.getElementById('days').innerText = days.toString().padStart(2, '0');
            document.getElementById('hours').innerText = hours.toString().padStart(2, '0');
            document.getElementById('minutes').innerText = minutes.toString().padStart(2, '0');
            document.getElementById('seconds').innerText = seconds.toString().padStart(2, '0');
          }

          setInterval(updateCountdown, 1000);
          updateCountdown(); // initial call
        </script>
      </body>
      </html>
    `;

    return new NextResponse(html, {
      headers: {
        'Content-Type': 'text/html',
        'Cache-Control': 'public, max-age=60', // Cache for 1 minute
      },
    });
  } catch (error) {
    console.error('Error generating viral countdown widget embed:', error);
    return NextResponse.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
