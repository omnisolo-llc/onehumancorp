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
  const tenant = searchParams.get('tenant') || 'demo';
  const authorName = searchParams.get('authorName') || 'Happy Customer';
  const reviewText = searchParams.get('reviewText') || 'This is the best service I have ever used. Highly recommended!';
  const rating = searchParams.get('rating') || '5';
  const theme = searchParams.get('theme') || 'light';
  const branding = searchParams.get('branding') !== 'false';

  const numRating = Math.max(1, Math.min(5, parseInt(rating, 10) || 5));
  const stars = '★'.repeat(numRating) + '☆'.repeat(5 - numRating);

  const html = `
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Customer Testimonial</title>
      <style>
        body { margin: 0; padding: 0; font-family: sans-serif; display: flex; justify-content: center; align-items: center; min-height: 100vh; background: transparent; }
        .widget {
          width: 100%; max-width: 400px; padding: 24px; border-radius: 16px; text-align: left;
          ${escapeHtml(theme) === 'dark' ? 'background: #1f2937; color: #f9fafb; border: 1px solid #374151;' : 'background: #ffffff; color: #111827; box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05); border: 1px solid #e5e7eb;'}
        }
        .stars { font-size: 1.5rem; color: #fbbf24; margin-bottom: 12px; letter-spacing: 2px; }
        .review { font-size: 1.1rem; line-height: 1.6; margin-bottom: 20px; font-style: italic; }
        .author { font-weight: bold; font-size: 1rem; display: flex; align-items: center; gap: 8px; }
        .author::before {
            content: '';
            display: inline-block;
            width: 32px;
            height: 32px;
            background: #e5e7eb;
            border-radius: 50%;
            background-image: url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="%239ca3af"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" /></svg>');
            background-size: 20px;
            background-position: center;
            background-repeat: no-repeat;
        }
        .footer { margin-top: 20px; font-size: 12px; text-align: center; border-top: 1px solid ${escapeHtml(theme) === 'dark' ? '#374151' : '#f3f4f6'}; padding-top: 12px; }
        .footer a { color: #6b7280; text-decoration: none; font-weight: bold; }
        .footer a:hover { color: #3b82f6; text-decoration: underline; }
      </style>
    </head>
    <body>
      <div class="widget">
        <div class="stars">${escapeHtml(stars)}</div>
        <div class="review">"${escapeHtml(reviewText)}"</div>
        <div class="author">${escapeHtml(authorName)}</div>
        ${branding ? `
        <div class="footer">
          <a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${encodeURIComponent(tenant)}&source=testimonial_widget" target="_blank">⚡ Powered by OHC</a>
        </div>
        ` : ''}
      </div>
    </body>
    </html>
  `;

  return new NextResponse(html, {
    headers: { 'Content-Type': 'text/html' },
  });
}
