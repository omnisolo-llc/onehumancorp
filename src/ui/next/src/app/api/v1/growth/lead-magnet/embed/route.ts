import { NextResponse } from 'next/server';
import { randomBytes } from 'node:crypto';

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
  const { searchParams } = new URL(request.url);
  const tenant = searchParams.get('tenant') || 'my-store';
  const theme = searchParams.get('theme') || 'light';
  const title = searchParams.get('title') || 'Unlock the Ultimate Business Checklist';
  const description = searchParams.get('description') || 'Enter your email below to get instant access.';
  const buttonText = searchParams.get('buttonText') || 'Download Now';
  const hideBranding = searchParams.get('hideBranding') === 'true';
  const nonce = randomBytes(16).toString('base64url');
  const tenantScriptValue = inlineScriptValue(tenant);
  const titleScriptValue = inlineScriptValue(title);
  const buttonTextScriptValue = inlineScriptValue(buttonText);

  const isDark = theme === 'dark';

  const bgColor = isDark ? '#111827' : '#ffffff';
  const textColor = isDark ? '#f9fafb' : '#111827';
  const descColor = isDark ? '#9ca3af' : '#4b5563';
  const borderColor = isDark ? '#374151' : '#e5e7eb';
  const inputBg = isDark ? '#1f2937' : '#ffffff';
  const iconBg = isDark ? '#312e81' : '#e0e7ff';
  const iconColor = isDark ? '#a5b4fc' : '#4f46e5';

  const html = `
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Lead Magnet</title>
      <style>
        body {
          margin: 0;
          padding: 16px;
          font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
          background: transparent;
        }
        .widget-container {
          background-color: ${bgColor};
          color: ${textColor};
          border: 1px solid ${borderColor};
          border-radius: 16px;
          padding: 24px;
          box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
          max-width: 400px;
          margin: 0 auto;
          text-align: center;
          transition: all 0.3s ease;
        }
        .icon-container {
          display: inline-flex;
          align-items: center;
          justify-content: center;
          width: 48px;
          height: 48px;
          border-radius: 50%;
          background-color: ${iconBg};
          color: ${iconColor};
          margin-bottom: 16px;
        }
        h3 {
          margin: 0 0 8px 0;
          font-size: 1.25rem;
          font-weight: 700;
          line-height: 1.2;
        }
        p {
          margin: 0 0 20px 0;
          font-size: 0.875rem;
          color: ${descColor};
          line-height: 1.5;
        }
        .form-group {
          display: flex;
          flex-direction: column;
          gap: 12px;
        }
        input[type="email"] {
          width: 100%;
          padding: 12px 16px;
          border: 1px solid ${borderColor};
          border-radius: 8px;
          background-color: ${inputBg};
          color: ${textColor};
          font-size: 0.875rem;
          box-sizing: border-box;
          outline: none;
          transition: border-color 0.2s;
        }
        input[type="email"]:focus {
          border-color: #4f46e5;
        }
        button {
          width: 100%;
          padding: 12px 16px;
          background-color: #4f46e5;
          color: white;
          border: none;
          border-radius: 8px;
          font-weight: 600;
          font-size: 0.875rem;
          cursor: pointer;
          transition: background-color 0.2s;
        }
        button:hover {
          background-color: #4338ca;
        }
        .success-message {
          display: none;
          color: #10b981;
          font-weight: 600;
          margin-top: 16px;
          padding: 12px;
          background: rgba(16, 185, 129, 0.1);
          border-radius: 8px;
        }
        .loading {
          opacity: 0.7;
          pointer-events: none;
        }
        .footer {
          margin-top: 16px;
          padding-top: 16px;
          border-top: 1px solid ${borderColor};
          font-size: 0.75rem;
          color: ${descColor};
          ${hideBranding ? 'display: none;' : ''}
        }
        .footer a {
          color: ${descColor};
          text-decoration: none;
          font-weight: 600;
        }
        .footer a:hover {
          color: ${textColor};
        }
      </style>
    </head>
    <body>
      <div class="widget-container" id="widget">
        <div class="icon-container">
          <svg width="24" height="24" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
          </svg>
        </div>

        <div id="content">
          <h3>${escapeHtml(title)}</h3>
          <p>${escapeHtml(description)}</p>

          <form id="leadForm" class="form-group">
            <input type="email" id="email" placeholder="Enter your email address" required />
            <button type="submit" id="submitBtn">${escapeHtml(buttonText)}</button>
          </form>
        </div>

        <div id="success" class="success-message">
          Success! Check your email for the download link.
        </div>

        <div class="footer">
          <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodeURIComponent(tenant)}" target="_blank" rel="noopener noreferrer">⚡ Powered by OHC</a>
        </div>
      </div>

      <script nonce="${nonce}">
        document.getElementById('leadForm').addEventListener('submit', async function(e) {
          e.preventDefault();
          const email = document.getElementById('email').value;
          const btn = document.getElementById('submitBtn');

          btn.classList.add('loading');
          btn.textContent = 'Sending...';

          try {
            // Record capture via backend API
            const response = await fetch('/api/v1/growth/lead-magnet/capture', {
              method: 'POST',
              headers: { 'Content-Type': 'application/json' },
              body: JSON.stringify({
                tenant_id: ${tenantScriptValue},
                email: email,
                source: 'lead_magnet_embed',
                campaign: ${titleScriptValue}
              })
            });

            if (response.ok) {
              document.getElementById('content').style.display = 'none';
              document.getElementById('success').style.display = 'block';
            } else {
              throw new Error('Capture failed');
            }
          } catch (error) {
            console.error('Error:', error);
            alert('Something went wrong. Please try again.');
            btn.classList.remove('loading');
            btn.textContent = ${buttonTextScriptValue};
          }
        });
      </script>
    </body>
    </html>
  `;

  return new NextResponse(html, {
    headers: {
      'Content-Type': 'text/html',
      'Cache-Control': 'public, max-age=3600',
      'Content-Security-Policy': `default-src 'none'; style-src 'unsafe-inline'; script-src 'nonce-${nonce}'; connect-src 'self'; frame-ancestors *; base-uri 'none'; form-action 'none'`,
      'X-Content-Type-Options': 'nosniff',
      'Referrer-Policy': 'no-referrer',
    },
  });
}

function escapeHtml(unsafe: string) {
    return unsafe
         .replace(/&/g, "&amp;")
         .replace(/</g, "&lt;")
         .replace(/>/g, "&gt;")
         .replace(/"/g, "&quot;")
         .replace(/'/g, "&#039;");
 }
