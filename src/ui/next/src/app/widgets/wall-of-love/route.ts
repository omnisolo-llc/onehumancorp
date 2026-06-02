import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  // To handle CORS for scripts loading content:
  const headers = {
      'Content-Type': 'application/javascript',
      'Cache-Control': 'public, max-age=3600, s-maxage=3600',
      'Access-Control-Allow-Origin': '*'
  };

  const jsContent = `
(function() {
  const container = document.getElementById('ohc-wall-of-love');
  if (!container) return;

  const storeName = container.getAttribute('data-store') || 'My Store';

  // Set basic container styles
  container.style.fontFamily = 'system-ui, -apple-system, sans-serif';
  container.style.padding = '24px';
  container.style.border = '1px solid #e5e7eb';
  container.style.borderRadius = '16px';
  container.style.maxWidth = '100%';
  container.style.background = '#ffffff';
  container.style.boxShadow = '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)';
  container.style.boxSizing = 'border-box';

  // Create header
  const header = document.createElement('h3');
  header.style.margin = '0 0 20px 0';
  header.style.fontSize = '1.25rem';
  header.style.fontWeight = '700';
  header.style.color = '#111827';
  header.style.textAlign = 'center';
  header.textContent = \`What people say about \${storeName}\`;
  container.appendChild(header);

  // Create loading state
  const loading = document.createElement('div');
  loading.style.textAlign = 'center';
  loading.style.color = '#6b7280';
  loading.style.fontSize = '0.875rem';
  loading.style.padding = '20px 0';
  loading.textContent = 'Loading reviews...';
  container.appendChild(loading);

  // Fetch data
  const scriptTag = document.currentScript || document.querySelector('script[src*="wall-of-love"]');
  let baseUrl = 'https://ohc.app';
  if (scriptTag && scriptTag.src) {
    try {
      const url = new URL(scriptTag.src);
      baseUrl = url.origin;
    } catch (e) {
        console.warn('Could not parse script URL');
    }
  }

  // To properly support e2e tests
  if (window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1' || window.location.protocol === 'about:') {
      baseUrl = window.location.origin === 'null' ? 'http://localhost:3000' : window.location.origin;
  }

  const fetchUrl = \`\${baseUrl}/api/v1/growth/wall_of_love/data?store=\${encodeURIComponent(storeName)}\`;

  fetch(fetchUrl)
    .then(res => res.json())
    .then(data => {
      container.removeChild(loading);

      if (!data.reviews || data.reviews.length === 0) {
        const empty = document.createElement('div');
        empty.textContent = 'No reviews yet.';
        empty.style.color = '#6b7280';
        container.appendChild(empty);
        return;
      }

      const grid = document.createElement('div');
      grid.style.display = 'grid';
      grid.style.gridTemplateColumns = 'repeat(auto-fill, minmax(280px, 1fr))';
      grid.style.gap = '16px';

      data.reviews.forEach(review => {
        const card = document.createElement('div');
        card.style.padding = '16px';
        card.style.background = '#f9fafb';
        card.style.borderRadius = '12px';
        card.style.border = '1px solid #f3f4f6';

        const stars = document.createElement('div');
        stars.style.color = '#fbbf24';
        stars.style.marginBottom = '8px';
        stars.style.fontSize = '1.125rem';
        stars.textContent = '★'.repeat(review.rating) + '☆'.repeat(5 - review.rating);

        const content = document.createElement('p');
        content.style.margin = '0 0 12px 0';
        content.style.fontSize = '0.9375rem';
        content.style.lineHeight = '1.5';
        content.style.color = '#374151';
        content.textContent = \`"\${review.content}"\`;

        const footer = document.createElement('div');
        footer.style.display = 'flex';
        footer.style.justifyContent = 'space-between';
        footer.style.alignItems = 'center';

        const author = document.createElement('span');
        author.style.fontWeight = '600';
        author.style.fontSize = '0.875rem';
        author.style.color = '#111827';
        author.textContent = review.author;

        const date = document.createElement('span');
        date.style.fontSize = '0.75rem';
        date.style.color = '#9ca3af';
        date.textContent = review.date;

        footer.appendChild(author);
        footer.appendChild(date);

        card.appendChild(stars);
        card.appendChild(content);
        card.appendChild(footer);

        grid.appendChild(card);
      });

      container.appendChild(grid);

      // Add viral footer
      const viralFooter = document.createElement('div');
      viralFooter.style.marginTop = '24px';
      viralFooter.style.paddingTop = '16px';
      viralFooter.style.borderTop = '1px solid #f3f4f6';
      viralFooter.style.textAlign = 'center';
      viralFooter.style.fontSize = '0.75rem';
      viralFooter.style.color = '#6b7280';

      const link = document.createElement('a');
      link.href = 'https://ohc.app?ref=wall-of-love-widget';
      link.target = '_blank';
      link.rel = 'noopener noreferrer';
      link.style.color = '#8b5cf6';
      link.style.fontWeight = '600';
      link.style.textDecoration = 'none';
      link.textContent = 'OHC';

      link.onmouseover = function() { this.style.textDecoration = 'underline'; }
      link.onmouseout = function() { this.style.textDecoration = 'none'; }

      viralFooter.innerHTML = '⚡ Powered by ';
      viralFooter.appendChild(link);

      container.appendChild(viralFooter);
    })
    .catch(err => {
      container.removeChild(loading);
      const error = document.createElement('div');
      error.style.color = '#ef4444';
      error.style.fontSize = '0.875rem';
      error.textContent = 'Failed to load reviews.';
      container.appendChild(error);
      console.error('Wall of Love Widget Error:', err);
    });
})();
  `;

  return new NextResponse(jsContent, {
    headers
  });
}
