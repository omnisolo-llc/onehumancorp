import { NextResponse } from 'next/server';

function safeTenant(value: string | null): string {
  const normalized = (value || 'my-store').trim().slice(0, 80);
  return encodeURIComponent(normalized || 'my-store');
}

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenant = safeTenant(searchParams.get('tenant'));
  const theme = searchParams.get('theme') || 'light';

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
        .wall-container {
            background-color: ${isDark ? 'rgba(17, 24, 39, 0.7)' : 'rgba(255, 255, 255, 0.65)'};
            backdrop-filter: blur(30px) saturate(210%);
            border: 1px solid ${isDark ? 'rgba(55, 65, 81, 0.5)' : 'rgba(229, 231, 235, 0.5)'};
            border-radius: 16px;
            box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);
            padding: 24px;
            max-width: 48rem;
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
            margin-bottom: 8px;
            margin-top: 0;
            letter-spacing: -0.025em;
        }
        .subtitle {
            color: ${isDark ? '#d1d5db' : '#4b5563'};
            font-size: 0.875rem;
            margin: 0;
        }
        .reviews-grid {
            display: grid;
            grid-template-columns: 1fr;
            gap: 16px;
        }
        @media (min-width: 640px) {
            .reviews-grid {
                grid-template-columns: repeat(2, 1fr);
            }
        }
        .review-card {
            background-color: ${isDark ? 'rgba(31, 41, 55, 0.8)' : '#fefce8'};
            border: 1px solid ${isDark ? 'rgba(55, 65, 81, 0.8)' : 'rgba(254, 240, 138, 0.5)'};
            border-radius: 12px;
            padding: 16px;
            display: flex;
            flex-direction: column;
            box-shadow: inset 0 2px 4px 0 rgba(255, 255, 255, 0.3);
        }
        .stars {
            color: #fbbf24;
            font-size: 1rem;
            margin-bottom: 8px;
            letter-spacing: 2px;
        }
        .review-text {
            color: ${isDark ? '#e5e7eb' : '#374151'};
            font-size: 0.875rem;
            font-style: italic;
            margin-bottom: 12px;
            flex: 1;
            line-height: 1.5;
        }
        .author {
            color: ${isDark ? '#9ca3af' : '#6b7280'};
            font-size: 0.75rem;
            font-weight: 600;
        }
        .footer {
            padding-top: 20px;
            margin-top: 24px;
            border-top: 1px solid ${isDark ? 'rgba(55, 65, 81, 0.5)' : 'rgba(229, 231, 235, 0.5)'};
            text-align: center;
        }
        .footer a {
            display: inline-flex;
            align-items: center;
            gap: 6px;
            font-weight: 700;
            color: #8b5cf6;
            text-decoration: none;
            font-size: 0.875rem;
            transition: all 0.2s ease;
            background-color: ${isDark ? 'rgba(139, 92, 246, 0.1)' : 'rgba(139, 92, 246, 0.1)'};
            padding: 6px 16px;
            border-radius: 9999px;
        }
        .footer a:hover {
            background-color: ${isDark ? 'rgba(139, 92, 246, 0.2)' : 'rgba(139, 92, 246, 0.2)'};
            color: #7c3aed;
        }
      </style>
    </head>
    <body>
      <div class="wall-container">
        <div class="header">
            <h2 class="title font-outfit">Loved by Customers</h2>
            <p class="subtitle">Don't just take our word for it.</p>
        </div>

        <div class="reviews-grid">
            <div class="review-card">
                <div class="stars">★★★★★</div>
                <p class="review-text">"Absolutely amazing product! Changed my life."</p>
                <div class="author">— Sarah M.</div>
            </div>
            <div class="review-card">
                <div class="stars">★★★★★</div>
                <p class="review-text">"Best customer service and top quality. Highly recommended!"</p>
                <div class="author">— Alex J.</div>
            </div>
             <div class="review-card">
                <div class="stars">★★★★★</div>
                <p class="review-text">"I buy from them every month. Never disappointed."</p>
                <div class="author">— Jamie L.</div>
            </div>
             <div class="review-card">
                <div class="stars">★★★★★</div>
                <p class="review-text">"Incredible value for the price. Fast shipping too."</p>
                <div class="author">— Chris D.</div>
            </div>
        </div>

        <!-- Viral Growth Loop Footer -->
        <div class="footer">
            <a href="https://ohc.store/join?ref=${tenant}" target="_blank" rel="noopener noreferrer">
                ⚡ Powered by OHC - Create your own Wall of Love
            </a>
        </div>
      </div>
    </body>
    </html>
  `;

  return new NextResponse(html, {
    headers: {
      'Content-Type': 'text/html',
      'Cache-Control': 'public, max-age=60, s-maxage=60',
      'Content-Security-Policy': "default-src 'none'; style-src 'unsafe-inline' https://fonts.googleapis.com; font-src https://fonts.gstatic.com; connect-src 'none'; frame-ancestors *; base-uri 'none'; form-action 'none'",
      'Referrer-Policy': 'strict-origin-when-cross-origin',
      'X-Content-Type-Options': 'nosniff'
    }
  });
}
