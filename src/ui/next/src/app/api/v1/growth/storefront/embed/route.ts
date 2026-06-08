import { NextResponse } from 'next/server';

function escapeHtml(unsafe: string) {
    if (!unsafe) return unsafe;
    return unsafe
         .replace(/&/g, "&amp;")
         .replace(/</g, "&lt;")
         .replace(/>/g, "&gt;")
         .replace(/"/g, "&quot;")
         .replace(/'/g, "&#039;");
}

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const rawTenant = searchParams.get('tenant') || 'demo';

  // Safe interpolation for arbitrary tenant names
  const tenant = escapeHtml(rawTenant);
  const encodedTenant = encodeURIComponent(rawTenant);

  // To simulate the full storefront embed securely.
  const baseUrl = process.env.NEXT_PUBLIC_BASE_URL || 'https://ohc.app';

  // Generate HTML for the iframe embed, including the viral "Powered by OHC" footer.
  // In a real production system, this would fetch actual store configuration from PostgreSQL via the internal Rust API.
  const html = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Storefront | ${tenant}</title>

    <!-- Open Graph Metadata for Rich Link Previews -->
    <meta property="og:title" content="Shop the Premium Collection at ${tenant}" />
    <meta property="og:description" content="Discover our latest products and exclusive offers." />
    <meta property="og:type" content="website" />
    <meta property="og:image" content="${baseUrl}/api/v1/growth/storefront/og-card?tenant=${encodedTenant}&amp;product_name=Premium%20Collection" />
    <meta property="og:image:width" content="1200" />
    <meta property="og:image:height" content="630" />
    <meta name="twitter:card" content="summary_large_image" />

    <style>
        :root {
            --primary: #000000;
            --background: #ffffff;
            --text: #1a1a1a;
            --border: #eaeaea;
            --muted: #666666;
            --radius: 12px;
        }

        body {
            margin: 0;
            padding: 0;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
            background-color: var(--background);
            color: var(--text);
            -webkit-font-smoothing: antialiased;
        }

        .store-header {
            padding: 24px;
            text-align: center;
            border-bottom: 1px solid var(--border);
        }

        .store-title {
            font-size: 24px;
            font-weight: 700;
            margin: 0 0 8px 0;
            letter-spacing: -0.5px;
        }

        .store-subtitle {
            color: var(--muted);
            font-size: 14px;
            margin: 0;
        }

        .product-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
            gap: 24px;
            padding: 24px;
            max-width: 1200px;
            margin: 0 auto;
        }

        .product-card {
            border: 1px solid var(--border);
            border-radius: var(--radius);
            overflow: hidden;
            transition: transform 0.2s ease, box-shadow 0.2s ease;
            background: white;
        }

        .product-card:hover {
            transform: translateY(-4px);
            box-shadow: 0 12px 24px rgba(0,0,0,0.08);
        }

        .product-image {
            width: 100%;
            height: 280px;
            background-color: #f5f5f7;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 48px;
        }

        .product-info {
            padding: 20px;
        }

        .product-name {
            font-weight: 600;
            font-size: 16px;
            margin: 0 0 8px 0;
        }

        .product-price {
            font-weight: 500;
            color: var(--muted);
            margin: 0 0 16px 0;
        }

        .buy-button {
            width: 100%;
            padding: 12px;
            background-color: var(--primary);
            color: white;
            border: none;
            border-radius: 8px;
            font-weight: 600;
            font-size: 14px;
            cursor: pointer;
            transition: opacity 0.2s ease;
        }

        .buy-button:hover {
            opacity: 0.9;
        }

        .viral-footer {
            text-align: center;
            padding: 32px 24px;
            margin-top: 48px;
            background-color: #fcfcfc;
            border-top: 1px solid var(--border);
            font-size: 14px;
            color: var(--muted);
        }

        .viral-footer a {
            color: var(--primary);
            text-decoration: none;
            font-weight: 600;
            display: inline-flex;
            align-items: center;
            gap: 4px;
        }

        .viral-footer a:hover {
            text-decoration: underline;
        }
    </style>
</head>
<body>
    <header class="store-header">
        <h1 class="store-title">${tenant}</h1>
        <p class="store-subtitle">Premium Goods &amp; Services</p>
    </header>

    <main class="product-grid">
        <!-- Mock Product 1 -->
        <article class="product-card">
            <div class="product-image" style="background: linear-gradient(135deg, #fdfbfb 0%, #ebedee 100%);">
                ⌚
            </div>
            <div class="product-info">
                <h2 class="product-name">Signature Watch</h2>
                <p class="product-price">$299.00</p>
                <button class="buy-button">Add to Cart</button>
            </div>
        </article>

        <!-- Mock Product 2 -->
        <article class="product-card">
            <div class="product-image" style="background: linear-gradient(135deg, #e0c3fc 0%, #8ec5fc 100%);">
                🎒
            </div>
            <div class="product-info">
                <h2 class="product-name">Everyday Backpack</h2>
                <p class="product-price">$129.00</p>
                <button class="buy-button">Add to Cart</button>
            </div>
        </article>

        <!-- Mock Product 3 -->
        <article class="product-card">
            <div class="product-image" style="background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);">
                🎧
            </div>
            <div class="product-info">
                <h2 class="product-name">Wireless Headphones</h2>
                <p class="product-price">$199.00</p>
                <button class="buy-button">Add to Cart</button>
            </div>
        </article>
    </main>

    <footer class="viral-footer">
        <p>Built for modern operators.</p>
        <p>⚡ Powered by <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodedTenant}">OHC</a></p>
    </footer>

    <script>
        // Simple interactivity for the mock buttons
        document.querySelectorAll('.buy-button').forEach(btn => {
            btn.addEventListener('click', function() {
                const originalText = this.innerText;
                this.innerText = 'Added ✓';
                this.style.backgroundColor = '#10b981';

                setTimeout(() => {
                    this.innerText = originalText;
                    this.style.backgroundColor = 'var(--primary)';
                }, 2000);
            });
        });
    </script>
</body>
</html>`;

  return new NextResponse(html, {
    headers: {
      'Content-Type': 'text/html; charset=utf-8',
      'Cache-Control': 'public, max-age=60, s-maxage=60',
    },
  });
}
