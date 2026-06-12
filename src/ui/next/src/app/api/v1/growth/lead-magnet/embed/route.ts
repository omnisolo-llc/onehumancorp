import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenantId = searchParams.get('tenant') || 'default-team';
  const theme = searchParams.get('theme') || 'light';

  const bgColor = theme === 'dark' ? '#1f2937' : '#ffffff';
  const textColor = theme === 'dark' ? '#f3f4f6' : '#111827';
  const borderColor = theme === 'dark' ? '#374151' : '#e5e7eb';
  const buttonColor = '#9333ea'; // Purple-600
  const buttonHoverColor = '#7e22ce'; // Purple-700
  const inputBgColor = theme === 'dark' ? '#374151' : '#f9fafb';

  const html = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Free Lead Magnet</title>
  <style>
    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
      margin: 0;
      padding: 0;
      background-color: ${bgColor};
      color: ${textColor};
      display: flex;
      justify-content: center;
      align-items: center;
      min-height: 100vh;
    }
    .container {
      width: 100%;
      max-width: 400px;
      padding: 24px;
      box-sizing: border-box;
      border: 1px solid ${borderColor};
      border-radius: 12px;
      background-color: ${bgColor};
      box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
    }
    .header {
      text-align: center;
      margin-bottom: 20px;
    }
    .title {
      font-size: 20px;
      font-weight: 700;
      margin: 0 0 8px 0;
    }
    .subtitle {
      font-size: 14px;
      color: #6b7280;
      margin: 0;
    }
    .form-group {
      margin-bottom: 16px;
    }
    .label {
      display: block;
      font-size: 14px;
      font-weight: 500;
      margin-bottom: 6px;
    }
    .input {
      width: 100%;
      padding: 10px 12px;
      font-size: 14px;
      border: 1px solid ${borderColor};
      border-radius: 8px;
      box-sizing: border-box;
      background-color: ${inputBgColor};
      color: ${textColor};
    }
    .input:focus {
      outline: none;
      border-color: ${buttonColor};
      box-shadow: 0 0 0 2px rgba(147, 51, 234, 0.2);
    }
    .button {
      width: 100%;
      padding: 12px;
      font-size: 14px;
      font-weight: 600;
      color: white;
      background-color: ${buttonColor};
      border: none;
      border-radius: 8px;
      cursor: pointer;
      transition: background-color 0.2s;
    }
    .button:hover {
      background-color: ${buttonHoverColor};
    }
    .success-message {
      display: none;
      text-align: center;
      padding: 20px 0;
      color: #059669;
      font-weight: 600;
    }
  </style>
</head>
<body>
  <div class="container" id="magnet-container">
    <div class="header">
      <h2 class="title">Get Our Free Guide</h2>
      <p class="subtitle">Enter your email below to instantly receive our exclusive resources.</p>
    </div>
    <form id="lead-form" onsubmit="submitForm(event)">
      <div class="form-group">
        <label for="email" class="label">Email Address</label>
        <input type="email" id="email" class="input" placeholder="you@example.com" required>
      </div>
      <button type="submit" class="button">Send Me the Guide</button>
    </form>
    <div id="success" class="success-message">
      <svg style="width:48px;height:48px;margin:0 auto 12px auto;display:block;" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
      Check your inbox! Your guide is on its way.
    </div>
  </div>

  <script>
    function submitForm(e) {
      e.preventDefault();
      const email = document.getElementById('email').value;
      if (!email) return;

      const btn = document.querySelector('.button');
      btn.innerText = 'Sending...';
      btn.disabled = true;

      // Call backend
      fetch('/api/v1/growth/lead-magnet/submit', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email: email, tenant: '${tenantId}' })
      })
      .then(res => res.json())
      .then(data => {
        document.getElementById('lead-form').style.display = 'none';
        document.getElementById('success').style.display = 'block';

        // Notify parent window to resize if needed
        window.parent.postMessage({ type: 'ohc-lead-magnet-success', height: document.body.scrollHeight }, '*');
      })
      .catch(err => {
        btn.innerText = 'Error! Try again.';
        btn.disabled = false;
      });
    }
  </script>
</body>
</html>`;

  return new NextResponse(html, {
    headers: {
      'Content-Type': 'text/html',
      'Cache-Control': 'public, max-age=3600',
    },
  });
}
