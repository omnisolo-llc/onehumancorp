import { NextResponse } from 'next/server';

export async function POST(request: Request) {
  try {
    const formData = await request.formData();
    const name = formData.get('name') as string;
    const email = formData.get('email') as string;
    const details = formData.get('details') as string;

    const { searchParams } = new URL(request.url);
    const tenant = searchParams.get('tenant') || 'my-business';

    const message = `Name: ${name}\nEmail: ${email}\nDetails: ${details}`;

    const backendUrl = process.env.BACKEND_URL || 'http://127.0.0.1:18789';

    try {

      await fetch(`${backendUrl}/api/agents/webhook`, {

        method: 'POST',

        headers: {

          'Content-Type': 'application/json',

        },

        body: JSON.stringify({

          tenant_id: tenant,

          source: 'work-intake',

          message: message,

        }),

      });

    } catch (e) {

      console.error('Failed to notify backend of work intake:', e);

    }

    const html = `

    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Request Submitted</title>
      <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700&display=swap" rel="stylesheet">
      <style>
        body { font-family: 'Inter', sans-serif; margin: 0; padding: 16px; background: transparent; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .card {
            background-color: #ffffff;
            border: 1px solid #e5e7eb;
            border-radius: 16px;
            box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
            overflow: hidden;
            display: flex;
            flex-direction: column;
            max-width: 24rem;
            margin: 0 auto;
        }
        .content { padding: 40px 20px; text-align: center; }
        .icon {
            font-size: 4rem;
            margin-bottom: 16px;
        }
        .title {
            color: #111827;
            font-size: 1.5rem;
            font-weight: 700;
            margin-bottom: 8px;
        }
        .desc {
            color: #4b5563;
            font-size: 1rem;
            margin-bottom: 24px;
            line-height: 1.5;
        }
        .footer {
            padding-top: 16px;
            margin-top: 16px;
            border-top: 1px solid #f3f4f6;
            color: #6b7280;
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
        <div class="content">
            <div class="icon">✅</div>
            <h2 class="title font-outfit">Request Received!</h2>
            <p class="desc">Thanks, ${name}! We've received your request and will be in touch shortly.</p>
        </div>
        <div style="padding: 0 20px 20px;">
             <!-- Viral Growth Loop Footer -->
             <div class="footer">
                <span>⚡ Powered by</span>
                <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodeURIComponent(tenant)}" target="_blank" rel="noopener noreferrer">OHC</a>
             </div>
        </div>
      </div>
    </body>
    </html>
    `;

    return new NextResponse(html, {
      headers: {
        'Content-Type': 'text/html',
        'Cache-Control': 'no-store, max-age=0',
      }
    });
  } catch (error) {
    return new NextResponse('Bad Request', { status: 400 });
  }
}
