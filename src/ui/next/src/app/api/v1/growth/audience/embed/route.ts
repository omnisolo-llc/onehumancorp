import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenantRaw = searchParams.get('tenant') || 'my-store';
  const tenant = encodeURIComponent(tenantRaw);
  const theme = searchParams.get('theme') || 'light';
  const headline = searchParams.get('headline') || 'Join our Newsletter';
  const discount = searchParams.get('discount') || 'WELCOME10';

  const isDark = theme === 'dark';
  const bgColor = isDark ? '#111827' : '#ffffff';
  const textColor = isDark ? '#ffffff' : '#111827';
  const descColor = isDark ? '#d1d5db' : '#4b5563';
  const borderColor = isDark ? '#374151' : '#e5e7eb';
  const inputBg = isDark ? '#1f2937' : '#f9fafb';

  const html = `
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Audience Builder Modal</title>
      <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700&display=swap" rel="stylesheet">
      <style>
        body { font-family: 'Inter', sans-serif; margin: 0; padding: 16px; background: transparent; display: flex; justify-content: center; align-items: center; min-height: 100vh; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .modal {
            background-color: ${bgColor};
            border: 1px solid ${borderColor};
            border-radius: 16px;
            box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
            width: 100%;
            max-width: 400px;
            position: relative;
            overflow: hidden;
            display: flex;
            flex-direction: column;
        }
        .header-bg {
            background: linear-gradient(135deg, #a855f7 0%, #3b82f6 100%);
            height: 80px;
            width: 100%;
        }
        .content {
            padding: 24px;
            text-align: center;
            flex: 1;
        }
        .icon-circle {
            width: 56px;
            height: 56px;
            background: white;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 28px;
            margin: -52px auto 16px auto;
            box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
            border: 4px solid ${bgColor};
        }
        h2 {
            margin: 0 0 8px 0;
            color: ${textColor};
            font-size: 24px;
            font-weight: 700;
        }
        p {
            margin: 0 0 20px 0;
            color: ${descColor};
            font-size: 14px;
            line-height: 1.5;
        }
        .form-group {
            margin-bottom: 16px;
            text-align: left;
        }
        input[type="email"] {
            width: 100%;
            padding: 12px;
            border-radius: 8px;
            border: 1px solid ${borderColor};
            background-color: ${inputBg};
            color: ${textColor};
            font-family: inherit;
            box-sizing: border-box;
            outline: none;
        }
        input[type="email"]:focus {
            border-color: #3b82f6;
            box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
        }
        .submit-btn {
            width: 100%;
            padding: 12px;
            border-radius: 8px;
            background-color: #3b82f6;
            color: white;
            border: none;
            font-weight: 600;
            cursor: pointer;
            transition: background-color 0.2s;
            font-family: inherit;
        }
        .submit-btn:hover {
            background-color: #2563eb;
        }
        .discount-badge {
            display: inline-block;
            background-color: #fef3c7;
            color: #d97706;
            padding: 4px 12px;
            border-radius: 9999px;
            font-size: 12px;
            font-weight: 700;
            margin-bottom: 12px;
            letter-spacing: 0.05em;
        }
        .footer {
            padding: 12px;
            text-align: center;
            border-top: 1px solid ${borderColor};
            font-size: 12px;
            color: ${descColor};
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 4px;
        }
        .footer a {
            color: #3b82f6;
            text-decoration: none;
            font-weight: 600;
        }
        .footer a:hover {
            text-decoration: underline;
        }

        .success-state {
            display: none;
            padding: 40px 20px;
        }
        .success-state h3 {
            color: #22c55e;
            margin: 0 0 12px 0;
            font-size: 20px;
        }
        .code-box {
            background: ${inputBg};
            border: 2px dashed ${borderColor};
            padding: 12px;
            border-radius: 8px;
            font-family: monospace;
            font-size: 18px;
            font-weight: 700;
            color: ${textColor};
            margin-top: 16px;
        }
      </style>
    </head>
    <body>
      <div class="modal" id="modal">
        <div class="header-bg"></div>
        <div class="content" id="form-content">
            <div class="icon-circle">💌</div>
            <div class="discount-badge">UNLOCK DISCOUNT</div>
            <h2 class="font-outfit">${escapeHtml(headline)}</h2>
            <p>Sign up to our list and get an exclusive discount code sent directly to your inbox!</p>

            <form id="capture-form" onsubmit="event.preventDefault(); submitForm();">
                <div class="form-group">
                    <input type="email" id="email-input" placeholder="Your email address" required />
                </div>
                <button type="submit" class="submit-btn">Get My Discount</button>
            </form>
        </div>

        <div class="content success-state" id="success-content">
             <div class="icon-circle">🎉</div>
             <h3 class="font-outfit">You're In!</h3>
             <p>Thanks for subscribing. Use the code below at checkout:</p>
             <div class="code-box">${escapeHtml(discount)}</div>
        </div>

        <div class="footer">
            ⚡ Powered by <a href="ohc://join?ref=${tenant}" target="_blank">OHC</a>
        </div>
      </div>

      <script>
        function submitForm() {
            const email = document.getElementById('email-input').value;
            if(email) {
                // In a real app, this would POST to a backend to save the email
                document.getElementById('form-content').style.display = 'none';
                document.getElementById('success-content').style.display = 'block';
            }
        }
      </script>
    </body>
    </html>
  `;

  return new NextResponse(html, {
    headers: {
      'Content-Type': 'text/html',
      'Cache-Control': 'public, max-age=60, s-maxage=60'
    }
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