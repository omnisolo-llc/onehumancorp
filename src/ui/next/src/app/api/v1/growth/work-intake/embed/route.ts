import { NextResponse } from 'next/server';

function safeTenant(value: string | null): string {
  const normalized = (value || 'my-business').trim().slice(0, 80);
  return encodeURIComponent(normalized || 'my-business');
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
  const title = searchParams.get('title') || 'Work Request';

  const host = safeHost(request.headers.get('host'));
  const protocol = safeProtocol(request.headers.get('x-forwarded-proto'));
  const baseUrl = `${protocol}://${host}`;

  const isDark = theme === 'dark';
  const submitUrl = `/api/v1/work-intake/submit?tenant=${tenant}`;

  const html = `
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Work Intake Embed</title>
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
            max-width: 24rem;
            margin: 0 auto;
            transition: all 0.3s ease;
        }
        .header {
            padding: 20px;
            background: linear-gradient(to right, #3b82f6, #8b5cf6);
            color: white;
            text-align: center;
        }
        .header-icon {
            font-size: 2.5rem;
            margin-bottom: 8px;
        }
        .title {
            font-size: 1.25rem;
            font-weight: 700;
            margin: 0;
        }
        .content { padding: 20px; flex: 1; display: flex; flex-direction: column; }
        .form-group { margin-bottom: 16px; text-align: left; }
        label {
            display: block;
            color: ${isDark ? '#d1d5db' : '#374151'};
            font-size: 0.875rem;
            font-weight: 600;
            margin-bottom: 6px;
        }
        input, textarea {
            width: 100%;
            padding: 10px 12px;
            border: 1px solid ${isDark ? '#4b5563' : '#d1d5db'};
            border-radius: 8px;
            background-color: ${isDark ? '#1f2937' : '#ffffff'};
            color: ${isDark ? '#ffffff' : '#111827'};
            font-family: inherit;
            font-size: 0.875rem;
            box-sizing: border-box;
            transition: border-color 0.15s ease;
        }
        input:focus, textarea:focus {
            outline: none;
            border-color: #3b82f6;
            box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
        }
        .btn {
            width: 100%;
            background-color: #2563eb;
            color: white;
            font-weight: 600;
            padding: 12px 16px;
            border-radius: 12px;
            text-align: center;
            border: none;
            cursor: pointer;
            transition: background-color 0.15s ease;
            box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 8px;
            margin-top: 8px;
            font-size: 1rem;
        }
        .btn:hover { background-color: #1d4ed8; }
        .footer {
            padding-top: 16px;
            margin-top: 16px;
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
        <div class="header">
           <div class="header-icon">✉️</div>
           <h2 class="title font-outfit">${title}</h2>
        </div>

        <div class="content">
            <form action="${submitUrl}" method="POST">
                <div class="form-group">
                    <label for="name">Name</label>
                    <input type="text" id="name" name="name" required placeholder="Jane Doe">
                </div>
                <div class="form-group">
                    <label for="email">Email</label>
                    <input type="email" id="email" name="email" required placeholder="jane@example.com">
                </div>
                <div class="form-group">
                    <label for="details">How can we help?</label>
                    <textarea id="details" name="details" rows="3" required placeholder="Describe your request..."></textarea>
                </div>
                <button type="submit" class="btn">Send Request</button>
            </form>

            <!-- Viral Growth Loop Footer -->
            <div class="footer">
               <span>⚡ Powered by</span>
               <a href="/onboarding?ref=${tenant}" target="_blank" rel="noopener noreferrer">OHC</a>
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
      'X-Content-Type-Options': 'nosniff'
    }
  });
}
