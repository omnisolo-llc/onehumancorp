import { NextResponse } from 'next/server';

export async function GET() {
  const html = `
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Premium Product Embed</title>
      <style>
        body { font-family: 'Inter', sans-serif; margin: 0; padding: 0; background: transparent; }
        .card { border: 1px solid #eaeaea; padding: 16px; border-radius: 8px; box-shadow: 0 1px 2px rgba(0,0,0,0.05); background: white; }
        .title { font-size: 18px; font-weight: bold; margin-bottom: 8px; }
        .desc { font-size: 14px; color: #4b5563; margin-bottom: 16px; }
        .btn { background-color: #2563eb; color: white; padding: 8px 16px; border-radius: 4px; font-size: 14px; width: 100%; display: block; text-align: center; text-decoration: none; margin-bottom: 16px; }
        .footer { font-size: 12px; text-align: center; color: #6b7280; padding-top: 8px; border-top: 1px solid #eaeaea; }
        .footer a { color: #3b82f6; text-decoration: none; }
        .footer a:hover { text-decoration: underline; }
      </style>
    </head>
    <body>
      <div class="card">
        <h2 class="title">Premium Product</h2>
        <p class="desc">A great product description.</p>
        <a href="#" class="btn">Buy Now</a>
        <div class="footer">
          ⚡ Powered by OHC <a href="ohc://join?ref=embed">Create yours</a>
        </div>
      </div>
    </body>
    </html>
  `;

  return new NextResponse(html, {
    headers: {
      'Content-Type': 'text/html'
    }
  });
}
