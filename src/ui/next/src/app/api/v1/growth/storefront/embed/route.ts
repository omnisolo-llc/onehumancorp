import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenantRaw = searchParams.get('tenant') || 'my-store';
  const tenant = encodeURIComponent(tenantRaw);
  const theme = searchParams.get('theme') || 'light';

  const isDark = theme === 'dark';
  const bgClass = isDark ? 'bg-gray-900' : 'bg-white';
  const textClass = isDark ? 'text-white' : 'text-gray-900';
  const descClass = isDark ? 'text-gray-300' : 'text-gray-600';
  const borderClass = isDark ? 'border-gray-700' : 'border-gray-200';
  const footerClass = isDark ? 'border-gray-700 text-gray-400' : 'border-gray-100 text-gray-500';

  const html = `
    <!DOCTYPE html>
    <html lang="en">
    <head>
      <meta charset="UTF-8">
      <meta name="viewport" content="width=device-width, initial-scale=1.0">
      <title>Storefront Embed</title>
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
            height: 100%;
            max-width: 24rem;
            margin: 0 auto;
            transition: all 0.3s ease;
        }
        .card:hover { box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04); }
        .image-container {
            width: 100%;
            height: 12rem;
            background: linear-gradient(to bottom right, #6366f1, #9333ea);
            position: relative;
        }
        .image-icon {
            position: absolute;
            inset: 0;
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
            font-size: 3rem;
        }
        .badge {
            position: absolute;
            top: 12px;
            right: 12px;
            background-color: rgba(255, 255, 255, 0.2);
            backdrop-filter: blur(12px);
            border-radius: 9999px;
            padding: 4px 12px;
            font-size: 0.75rem;
            font-weight: 700;
            color: white;
            border: 1px solid rgba(255, 255, 255, 0.3);
        }
        .content { padding: 20px; flex: 1; display: flex; flex-direction: column; }
        .title {
            color: ${isDark ? '#ffffff' : '#111827'};
            font-size: 1.25rem;
            font-weight: 700;
            margin-bottom: 8px;
            margin-top: 0;
            letter-spacing: -0.025em;
        }
        .desc {
            color: ${isDark ? '#d1d5db' : '#4b5563'};
            font-size: 0.875rem;
            margin-bottom: 20px;
            margin-top: 0;
            line-height: 1.625;
            flex: 1;
        }
        .price-row { display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px; }
        .price {
            color: ${isDark ? '#ffffff' : '#111827'};
            font-size: 1.5rem;
            font-weight: 700;
        }
        .stock-badge {
            font-size: 0.75rem;
            color: #22c55e;
            font-weight: 600;
            padding: 4px 8px;
            background-color: ${isDark ? 'rgba(21, 128, 61, 0.3)' : '#f0fdf4'};
            border-radius: 6px;
        }
        .btn {
            width: 100%;
            background-color: #2563eb;
            color: white;
            font-weight: 600;
            padding: 12px 16px;
            border-radius: 12px;
            text-align: center;
            text-decoration: none;
            transition: background-color 0.15s ease;
            box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 8px;
            margin-bottom: 16px;
            box-sizing: border-box;
        }
        .btn:hover { background-color: #1d4ed8; }
        .footer {
            padding-top: 16px;
            margin-top: auto;
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
        <!-- Product Image -->
        <div class="image-container">
           <div class="image-icon">🛍️</div>
           <div class="badge">Featured</div>
        </div>

        <!-- Product Info -->
        <div class="content">
            <h2 class="title font-outfit">Premium Product</h2>
            <p class="desc">Discover our exclusive, high-quality products curated just for you. Buy directly from this widget!</p>

            <div class="price-row">
               <span class="price font-outfit">$49.99</span>
               <span class="stock-badge">In Stock</span>
            </div>

            <a href="#" class="btn">
               <svg style="width: 16px; height: 16px;" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z"></path></svg>
               Buy Now
            </a>

            <!-- Viral Growth Loop Footer -->
            <div class="footer">
               <span>⚡ Powered by</span>
               <a href="ohc://join?ref=${tenant}" target="_blank">OHC</a>
            </div>
        </div>
      </div>
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
