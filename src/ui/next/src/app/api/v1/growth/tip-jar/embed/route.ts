import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const tenant = searchParams.get('tenant') || 'default-tenant';
    const name = searchParams.get('name') || 'Creator Name';
    const message = searchParams.get('message') || 'Buy me a coffee!';
    const amountsStr = searchParams.get('amounts') || '5, 10, 20';
    const theme = searchParams.get('theme') || 'light';
    const branding = searchParams.get('branding') !== 'false';

    const amounts = amountsStr.split(',').map(a => a.trim()).filter(a => a);

    const isDark = theme === 'dark';
    const bgColor = isDark ? '#1f2937' : '#ffffff';
    const textColor = isDark ? '#f9fafb' : '#111827';
    const descColor = isDark ? '#d1d5db' : '#4b5563';
    const borderColor = isDark ? '#374151' : '#e5e7eb';
    const buttonBg = isDark ? '#374151' : '#f9fafb';
    const buttonBorder = isDark ? '#4b5563' : '#e5e7eb';
    const inputBg = isDark ? '#1f2937' : '#ffffff';
    const brandingColor = '#6b7280';
    const brandingBorder = isDark ? '#374151' : '#f3f4f6';

    const amountsHtml = amounts.map(amt => `
      <button type="button" class="amt-btn" onclick="document.getElementById('custom-amount').value = '${amt}'">
        $${amt}
      </button>
    `).join('');

    const brandingHtml = branding ? `
      <div class="branding">
        <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodeURIComponent(tenant)}" target="_blank">
          ⚡ Powered by OHC
        </a>
      </div>
    ` : '';

    const html = `<!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Tip Jar</title>
      <style>
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700&display=swap');

        * { box-sizing: border-box; }
        body {
          margin: 0;
          padding: 0;
          font-family: 'Inter', sans-serif;
          background: transparent;
        }
        .widget-container {
          background-color: ${bgColor};
          border: 1px solid ${borderColor};
          border-radius: 16px;
          padding: 24px;
          display: flex;
          flex-direction: column;
          color: ${textColor};
          box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
        }
        .header {
          display: flex;
          justify-content: center;
          margin-bottom: 16px;
        }
        .avatar {
          width: 64px;
          height: 64px;
          border-radius: 50%;
          background: linear-gradient(to top right, #6366f1, #ec4899);
          display: flex;
          align-items: center;
          justify-content: center;
          font-size: 24px;
          box-shadow: inset 0 2px 4px rgba(0,0,0,0.1);
        }
        .name {
          font-family: 'Outfit', sans-serif;
          font-size: 20px;
          font-weight: 700;
          text-align: center;
          margin: 0 0 8px 0;
        }
        .message {
          font-size: 14px;
          text-align: center;
          margin: 0 0 24px 0;
          color: ${descColor};
          line-height: 1.5;
        }
        .amounts {
          display: flex;
          gap: 8px;
          justify-content: center;
          flex-wrap: wrap;
          margin-bottom: 16px;
        }
        .amt-btn {
          padding: 8px 16px;
          border-radius: 9999px;
          border: 1px solid ${buttonBorder};
          background-color: ${buttonBg};
          color: ${textColor};
          font-size: 14px;
          font-weight: 500;
          cursor: pointer;
          transition: all 0.2s;
        }
        .amt-btn:hover {
          border-color: #6366f1;
        }
        .custom-input-group {
          display: flex;
          margin-bottom: 16px;
        }
        .custom-input {
          flex: 1;
          padding: 8px 16px;
          border: 1px solid ${buttonBorder};
          border-right: none;
          border-top-left-radius: 8px;
          border-bottom-left-radius: 8px;
          background-color: ${inputBg};
          color: ${textColor};
          font-size: 14px;
          outline: none;
        }
        .custom-input:focus {
          border-color: #6366f1;
        }
        .submit-btn {
          padding: 8px 16px;
          background-color: #4f46e5;
          color: white;
          border: none;
          border-top-right-radius: 8px;
          border-bottom-right-radius: 8px;
          font-size: 14px;
          font-weight: 500;
          cursor: pointer;
          transition: background-color 0.2s;
        }
        .submit-btn:hover {
          background-color: #4338ca;
        }
        .branding {
          margin-top: 8px;
          padding-top: 12px;
          border-top: 1px solid ${brandingBorder};
          text-align: center;
          font-size: 12px;
        }
        .branding a {
          color: ${brandingColor};
          text-decoration: none;
          font-weight: 700;
        }
        .branding a:hover {
          text-decoration: underline;
        }
      </style>
    </head>
    <body>
      <div class="widget-container">
        <div class="header">
          <div class="avatar">☕</div>
        </div>
        <h3 class="name">${escapeHtml(name)}</h3>
        <p class="message">${escapeHtml(message)}</p>

        <form action="/checkout" method="GET" target="_blank" style="margin:0;">
          <input type="hidden" name="tenant" value="${escapeHtml(tenant)}" />
          <input type="hidden" name="type" value="tip" />

          <div class="amounts">
            ${amountsHtml}
          </div>

          <div class="custom-input-group">
            <input type="number" id="custom-amount" name="amount" class="custom-input" placeholder="Custom amount" min="1" step="1" required />
            <button type="submit" class="submit-btn">Tip</button>
          </div>
        </form>

        ${brandingHtml}
      </div>
    </body>
    </html>`;

    return new NextResponse(html, {
      headers: {
        'Content-Type': 'text/html',
        'Cache-Control': 'public, max-age=300, s-maxage=300',
      },
    });
  } catch (error) {
    return new NextResponse('Internal Server Error', { status: 500 });
  }
}

function escapeHtml(unsafe: string) {
    return unsafe
         .replace(/&/g, "&amp;")
         .replace(/</g, "&lt;")
         .replace(/>/g, "&gt;")
         .replace(/"/g, "&quot;")
         .replace(/'/g, "&#039;");
}
