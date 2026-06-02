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
  const theme = searchParams.get('theme') || 'gradient';

  const host = safeHost(request.headers.get('host'));
  const protocol = safeProtocol(request.headers.get('x-forwarded-proto'));
  const baseUrl = `${protocol}://${host}`;

  let background = 'linear-gradient(135deg, #a18cd1 0%, #fbc2eb 100%)';
  let cardBg = 'rgba(255, 255, 255, 0.65)';
  let textColor = '#1D1D1F';

  if (theme === 'dark') {
      background = '#111827';
      cardBg = 'rgba(31, 41, 55, 0.8)';
      textColor = '#f9fafb';
  } else if (theme === 'light') {
      background = '#f3f4f6';
      cardBg = '#ffffff';
      textColor = '#111827';
  }

  const html = `
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Link in Bio - ${tenant}</title>
      <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700&display=swap" rel="stylesheet">
      <style>
        body {
            font-family: 'Inter', sans-serif;
            margin: 0;
            padding: 24px;
            background: ${background};
            color: ${textColor};
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            align-items: center;
        }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .container {
            width: 100%;
            max-width: 400px;
            margin: 0 auto;
            text-align: center;
            display: flex;
            flex-direction: column;
            flex: 1;
        }
        .profile {
            margin-bottom: 32px;
            margin-top: 24px;
        }
        .avatar {
            width: 96px;
            height: 96px;
            border-radius: 50%;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            margin: 0 auto 16px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 40px;
            color: white;
            box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.2);
            border: 4px solid rgba(255, 255, 255, 0.2);
        }
        .title {
            font-size: 1.5rem;
            font-weight: 700;
            margin: 0 0 8px 0;
            letter-spacing: -0.025em;
        }
        .bio {
            font-size: 0.95rem;
            opacity: 0.9;
            margin: 0;
        }
        .links {
            display: flex;
            flex-direction: column;
            gap: 16px;
            width: 100%;
        }
        .link-btn {
            background-color: ${cardBg};
            backdrop-filter: blur(20px) saturate(200%);
            -webkit-backdrop-filter: blur(20px) saturate(200%);
            border: 1px solid rgba(255, 255, 255, 0.2);
            padding: 16px 24px;
            border-radius: 16px;
            color: ${textColor};
            text-decoration: none;
            font-weight: 600;
            font-size: 1.05rem;
            transition: all 0.2s ease;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.05);
            display: flex;
            align-items: center;
            justify-content: center;
        }
        .link-btn:hover {
            transform: translateY(-2px);
            box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1);
            background-color: rgba(255, 255, 255, 0.8);
            color: #000;
        }
        .footer {
            margin-top: auto;
            padding-top: 40px;
            padding-bottom: 20px;
            font-size: 0.85rem;
            opacity: 0.8;
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 6px;
        }
        .footer a {
            font-weight: 700;
            color: ${textColor};
            text-decoration: none;
            transition: opacity 0.2s ease;
        }
        .footer a:hover { opacity: 1; text-decoration: underline; }
      </style>
      <meta property="og:title" content="${tenant} - Links" />
      <meta property="og:description" content="Check out my links, store, and bookings." />
    </head>
    <body>
      <div class="container">
        <div class="profile">
            <div class="avatar">✨</div>
            <h1 class="title font-outfit">${decodeURIComponent(tenant).replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;').replace(/'/g, '&#039;')}</h1>
            <p class="bio">Welcome to my official page. Find all my latest products, services, and content right here!</p>
        </div>

        <div class="links">
            <a href="${baseUrl}/storefront/${tenant}" class="link-btn" target="_blank" rel="noopener noreferrer">🛍️ Shop My Store</a>
            <a href="${baseUrl}/booking/${tenant}" class="link-btn" target="_blank" rel="noopener noreferrer">📅 Book a Session</a>
            <a href="${baseUrl}/portfolio/${tenant}" class="link-btn" target="_blank" rel="noopener noreferrer">🎨 View Portfolio</a>
        </div>

        <!-- Viral Growth Loop Footer -->
        <div class="footer">
            <span>⚡ Powered by</span>
            <a href="${baseUrl}/join?ref=${tenant}" target="_blank" rel="noopener noreferrer">OHC</a>
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