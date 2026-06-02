import { NextResponse } from 'next/server';

function safeTenant(value: string | null): string {
  const normalized = (value || 'my-store').trim().slice(0, 80);
  return encodeURIComponent(normalized || 'my-store');
}

function safeHost(value: string | null): string {
  const host = (value || 'ohc.app').trim().toLowerCase();
  return /^[a-z0-9.-]+(?::\d{1,5})?$/.test(host) ? host : 'ohc.app';
}

function safeProtocol(value: string | null): 'http' | 'https' {
  return value === 'http' ? 'http' : 'https';
}

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenant = safeTenant(searchParams.get('tenant'));
  const theme = searchParams.get('theme') || 'light';

  const host = safeHost(request.headers.get('host'));
  const protocol = safeProtocol(request.headers.get('x-forwarded-proto'));

  const isDark = theme === 'dark';

  const html = `
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Wall of Love</title>
      <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700&display=swap" rel="stylesheet">
      <style>
        body { font-family: 'Inter', sans-serif; margin: 0; padding: 16px; background: transparent; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .widget-container {
            background-color: ${isDark ? '#111827' : '#ffffff'};
            border: 1px solid ${isDark ? '#374151' : '#e5e7eb'};
            border-radius: 16px;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
            padding: 24px;
            max-width: 32rem;
            margin: 0 auto;
        }
        .header {
            text-align: center;
            margin-bottom: 24px;
        }
        .title {
            color: ${isDark ? '#ffffff' : '#111827'};
            font-size: 1.5rem;
            font-weight: 700;
            margin: 0 0 8px 0;
        }
        .subtitle {
            color: ${isDark ? '#9ca3af' : '#6b7280'};
            font-size: 0.875rem;
            margin: 0;
        }
        .stars {
            color: #facc15;
            letter-spacing: 2px;
            font-size: 1.25rem;
            margin-bottom: 4px;
        }
        .review-card {
            background-color: ${isDark ? '#1f2937' : '#f9fafb'};
            border: 1px solid ${isDark ? '#374151' : '#f3f4f6'};
            border-radius: 12px;
            padding: 16px;
            margin-bottom: 16px;
        }
        .review-text {
            color: ${isDark ? '#e5e7eb' : '#374151'};
            font-size: 0.875rem;
            line-height: 1.5;
            font-style: italic;
            margin: 0 0 12px 0;
        }
        .reviewer {
            display: flex;
            align-items: center;
            gap: 8px;
        }
        .avatar {
            width: 24px;
            height: 24px;
            border-radius: 50%;
            background-color: #e5e7eb;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 0.75rem;
            font-weight: 600;
            color: #4b5563;
        }
        .reviewer-name {
            color: ${isDark ? '#d1d5db' : '#4b5563'};
            font-size: 0.75rem;
            font-weight: 600;
        }
        .footer {
            margin-top: 24px;
            padding-top: 16px;
            border-top: 1px solid ${isDark ? '#374151' : '#f3f4f6'};
            text-align: center;
            font-size: 0.75rem;
            color: ${isDark ? '#9ca3af' : '#6b7280'};
        }
        .footer a {
            color: #3b82f6;
            text-decoration: none;
            font-weight: 600;
        }
        .footer a:hover {
            text-decoration: underline;
        }
      </style>
    </head>
    <body>
      <div class="widget-container">
        <div class="header">
            <h2 class="title font-outfit">Wall of Love</h2>
            <p class="subtitle">See what our customers are saying</p>
        </div>

        <div class="review-card">
            <div class="stars">★★★★★</div>
            <p class="review-text">"Absolutely amazing product! Changed my life completely. I can't recommend it enough to everyone I know."</p>
            <div class="reviewer">
                <div class="avatar">S</div>
                <span class="reviewer-name">Sarah M.</span>
            </div>
        </div>

        <div class="review-card">
            <div class="stars">★★★★★</div>
            <p class="review-text">"Best customer service and top quality. The attention to detail is just phenomenal."</p>
            <div class="reviewer">
                <div class="avatar">A</div>
                <span class="reviewer-name">Alex J.</span>
            </div>
        </div>

        <div class="footer">
            ⚡ Powered by <a href="https://ohc.store/join?ref=${tenant}" target="_blank" rel="noopener noreferrer">OHC</a>
        </div>
      </div>
    </body>
    </html>
  `;

  return new NextResponse(html, {
    headers: {
      'Content-Type': 'text/html',
      'Cache-Control': 'public, max-age=60, s-maxage=60',
      'Content-Security-Policy': "default-src 'none'; style-src 'unsafe-inline' https://fonts.googleapis.com; font-src https://fonts.gstatic.com; img-src https: data:; connect-src 'none'; frame-ancestors *; base-uri 'none'; form-action 'none'",
      'Referrer-Policy': 'strict-origin-when-cross-origin',
      'X-Content-Type-Options': 'nosniff'
    }
  });
}
