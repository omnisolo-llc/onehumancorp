import { NextResponse } from 'next/server';

function escapeHtml(unsafe: string) {
    return unsafe
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;")
        .replace(/'/g, "&#039;");
}

export async function GET(request: Request) {
    const { searchParams } = new URL(request.url);
    const tenant = searchParams.get('tenant') || 'embed';
    const product = searchParams.get('product') || 'New Feature Launch';
    const goal = searchParams.get('goal') || '3';
    const theme = searchParams.get('theme') || 'light';
    const hideBranding = searchParams.get('hideBranding') === 'true';

    const isDark = theme === 'dark';
    const bgColor = isDark ? '#1d1d1f' : '#ffffff';
    const textColor = isDark ? '#f5f5f7' : '#1d1d1f';
    const mutedColor = isDark ? '#a1a1aa' : '#6b7280';
    const inputBg = isDark ? '#2d2d30' : '#f9fafb';
    const borderColor = isDark ? '#3f3f46' : '#e5e7eb';

    const safeTenant = escapeHtml(tenant);
    const safeProduct = escapeHtml(product);
    const safeGoal = escapeHtml(goal);

    const brandingHtml = hideBranding
        ? ''
        : `<div style="margin-top: 16px; font-size: 12px; text-align: center;">
            <a href="https://omnisolo.co/api/v1/growth/referrals/click?target=/onboarding&ref=${safeTenant}&source=waitlist_embed" target="_blank" rel="noopener noreferrer" style="color: ${mutedColor}; text-decoration: none; font-weight: 600;">⚡ OmniSolo</a>
        </div>`;

    const html = `<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>Join Waitlist</title>
  <style>
    body {
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
      background: ${bgColor};
      color: ${textColor};
      margin: 0;
      padding: 24px;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      box-sizing: border-box;
      height: 100%;
    }
    .widget-container {
      max-width: 400px;
      width: 100%;
      text-align: center;
    }
    .icon {
      font-size: 32px;
      margin-bottom: 12px;
    }
    h2 {
      margin: 0 0 8px 0;
      font-size: 20px;
      font-weight: 700;
    }
    p {
      margin: 0 0 20px 0;
      font-size: 14px;
      color: ${mutedColor};
      line-height: 1.5;
    }
    .input-group {
      display: flex;
      gap: 8px;
      margin-bottom: 16px;
    }
    input {
      flex: 1;
      padding: 12px 16px;
      border: 1px solid ${borderColor};
      border-radius: 8px;
      font-size: 14px;
      background: ${inputBg};
      color: ${textColor};
      outline: none;
    }
    input:focus {
      border-color: #0066FF;
    }
    button {
      background: #0066FF;
      color: white;
      border: none;
      padding: 12px 24px;
      border-radius: 8px;
      font-weight: 600;
      cursor: pointer;
      font-size: 14px;
      transition: background 0.2s;
    }
    button:hover {
      background: #0052cc;
    }
  </style>
</head>
<body>
  <div class="widget-container" data-tenant="${safeTenant}">
    <div class="icon">✨</div>
    <h2>Join the ${safeProduct} Waitlist</h2>
    <p>Be the first to access our new launch. Refer ${safeGoal} friends to jump to the front of the line!</p>

    <div class="input-group">
      <input type="email" placeholder="Your email address" id="email-input" />
      <button id="join-btn">Join Waitlist</button>
    </div>

    <div id="success-message" style="display: none; padding: 12px; background: rgba(34, 197, 94, 0.1); color: #16a34a; border-radius: 8px; margin-bottom: 16px; font-size: 14px; font-weight: 500;">
      Thanks for joining! We'll be in touch.
    </div>

    ${brandingHtml}
  </div>

  <script>
    document.getElementById('join-btn').addEventListener('click', function() {
      const email = document.getElementById('email-input').value;
      if (!email) return;

      const btn = this;
      btn.textContent = 'Joining...';
      btn.disabled = true;

      fetch('/api/v1/growth/waitlist', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email: email, tenant: '${safeTenant}' })
      })
      .then(res => res.json())
      .then(() => {
        document.querySelector('.input-group').style.display = 'none';
        document.getElementById('success-message').style.display = 'block';
      })
      .catch(() => {
        btn.textContent = 'Join Waitlist';
        btn.disabled = false;
      });
    });
  </script>
</body>
</html>`;

    return new NextResponse(html, {
        headers: {
            'Content-Type': 'text/html; charset=utf-8',
        },
    });
}
