import { NextResponse } from 'next/server';

function safeTenant(value: string | null): string {
  const normalized = (value || 'default-store').trim().slice(0, 80);
  return encodeURIComponent(normalized || 'default-store');
}

function safeHost(value: string | null): string {
  const host = (value || 'ohc.app').trim().toLowerCase();
  return /^[a-z0-9.-]+(?::\d{1,5})?$/.test(host) ? host : 'ohc.app';
}

function safeProtocol(value: string | null): 'http' | 'https' {
  return value === 'http' ? 'http' : 'https';
}

function escapeHtml(unsafe: string): string {
    return unsafe
         .replace(/&/g, "&amp;")
         .replace(/</g, "&lt;")
         .replace(/>/g, "&gt;")
         .replace(/"/g, "&quot;")
         .replace(/'/g, "&#039;");
}

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenant = safeTenant(searchParams.get('tenant'));
  const theme = searchParams.get('theme') || 'light';

  // Use escapeHtml to prevent XSS
  const rawService = searchParams.get('service') || 'Service Consultation';
  const service = escapeHtml(rawService);

  const host = safeHost(request.headers.get('host'));
  const protocol = safeProtocol(request.headers.get('x-forwarded-proto'));
  const baseUrl = `${protocol}://${host}`;

  const isDark = theme === 'dark';
  const bookingUrl = `${baseUrl}/booking?tenant=${tenant}`;

  const html = `
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Booking Embed</title>
      <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700&display=swap" rel="stylesheet">
      <style>
        body { font-family: 'Inter', sans-serif; margin: 0; padding: 16px; background: transparent; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .card {
            background-color: ${isDark ? '#111827' : '#ffffff'};
            border: 1px solid ${isDark ? '#374151' : '#e5e7eb'};
            border-radius: 16px;
            box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
            overflow: hidden;
            display: flex;
            flex-direction: column;
            height: 100%;
            max-width: 24rem;
            margin: 0 auto;
            transition: all 0.3s ease;
        }
        .card:hover { box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04); }
        .image-container {
            width: 100%;
            height: 12rem;
            background: linear-gradient(to bottom right, #3b82f6, #06b6d4);
            position: relative;
        }
        .image-icon {
            position: absolute;
            inset: 0;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
            font-size: 3rem;
        }
        .badge {
            position: absolute;
            top: 12px;
            right: 12px;
            background-color: rgba(255, 255, 255, 0.2);
            backdrop-filter: blur(12px);
            border-radius: 9999px;
            padding: 4px 12px;
            font-size: 0.75rem;
            font-weight: 700;
            color: white;
            border: 1px solid rgba(255, 255, 255, 0.3);
        }
        .content { padding: 20px; flex: 1; display: flex; flex-direction: column; }
        .title {
            color: ${isDark ? '#ffffff' : '#111827'};
            font-size: 1.25rem;
            font-weight: 700;
            margin-bottom: 8px;
            margin-top: 0;
            letter-spacing: -0.025em;
        }
        .desc {
            color: ${isDark ? '#d1d5db' : '#4b5563'};
            font-size: 0.875rem;
            margin-bottom: 20px;
            margin-top: 0;
            line-height: 1.625;
            flex: 1;
        }
        .btn {
            width: 100%;
            background-color: #2563eb;
            color: white;
            font-weight: 600;
            padding: 12px 16px;
            border-radius: 12px;
            text-align: center;
            text-decoration: none;
            transition: background-color 0.15s ease;
            box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 8px;
            margin-bottom: 16px;
            box-sizing: border-box;
        }
        .btn:hover { background-color: #1d4ed8; }
        .footer {
            padding-top: 16px;
            margin-top: auto;
            border-top: 1px solid ${isDark ? '#374151' : '#f3f4f6'};
            color: ${isDark ? '#9ca3af' : '#6b7280'};
            font-size: 0.75rem;
            text-align: center;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 6px;
        }
        .footer a {
            font-weight: 700;
            color: #3b82f6;
            text-decoration: none;
            transition: color 0.15s ease;
        }
        .footer a:hover { color: #2563eb; text-decoration: underline; }
      </style>
    </head>
    <body>
      <div class="card">
        <!-- Image -->
        <div class="image-container">
           <div class="image-icon">📅</div>
           <div class="badge">Book Now</div>
        </div>

        <!-- Info -->
        <div class="content">
            <h2 class="title font-outfit">${service}</h2>
            <p class="desc">Schedule your appointment with us easily. Tell us what you need and we will get right back to you.</p>

            <a href="${bookingUrl}" class="btn" target="_blank" rel="noopener noreferrer">
               <svg style="width: 16px; height: 16px;" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
               Request a Service
            </a>

            <!-- Viral Growth Loop Footer -->
            <div class="footer">
               <span>⚡ Powered by</span>
               <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}">OHC</a>
            </div>
        </div>
      </div>
    </body>
    </html>
  `;

  return new NextResponse(html, {
    headers: {
      'Content-Type': 'text/html',
      'Cache-Control': 'public, max-age=60, s-maxage=60, stale-while-revalidate=300',
      'Content-Security-Policy': "default-src 'none'; style-src 'unsafe-inline' https://fonts.googleapis.com; font-src https://fonts.gstatic.com; img-src https: data:; connect-src 'none'; frame-ancestors *; base-uri 'none'; form-action 'none'",
      'Referrer-Policy': 'strict-origin-when-cross-origin',
      'X-Content-Type-Options': 'nosniff'
    }
  });
}
