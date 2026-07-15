import { NextRequest, NextResponse } from 'next/server';

export async function GET(request: NextRequest) {
  const searchParams = request.nextUrl?.searchParams || new URL(request.url).searchParams;
  const tenant = searchParams.get('tenant') || 'default';
  const style = searchParams.get('style') || 'pill';
  const text = searchParams.get('text') || 'Powered by OHC';
  const theme = searchParams.get('theme') || 'light';

  // For testing purposes, we can hardcode the API URL to http://localhost:8080 or use an environment variable.
  const apiBase = process.env.API_BASE_URL || 'http://localhost:8080';

  // Track impression or referral intent via the main server
  try {
     await fetch(`${apiBase}/api/v1/growth/referrals/click`, {
         method: 'POST',
         headers: {
             'Content-Type': 'application/json'
         },
         body: JSON.stringify({
            tenant_id: tenant,
            source: 'footer_branding'
         })
     });
  } catch (error) {
     console.error('Failed to log impression', error);
  }

  const jsCode = `
    (function() {
      const tenant = '${tenant}';
      const style = '${style}';
      const text = '${text}';
      const theme = '${theme}';

      const badgeContainer = document.createElement('div');
      badgeContainer.style.position = 'fixed';

      if (style === 'pill') {
        badgeContainer.style.bottom = '20px';
        badgeContainer.style.right = '20px';
        badgeContainer.style.zIndex = '9999';

        const link = document.createElement('a');
        link.href = 'https://app.onehumancorp.com/invite/' + tenant;
        link.target = '_blank';
        link.style.display = 'flex';
        link.style.alignItems = 'center';
        link.style.gap = '8px';
        link.style.padding = '8px 12px';
        link.style.borderRadius = '20px';
        link.style.backgroundColor = theme === 'dark' ? '#1D1D1F' : '#FFFFFF';
        link.style.color = theme === 'dark' ? '#F5F5F7' : '#1D1D1F';
        link.style.boxShadow = '0 4px 12px rgba(0, 0, 0, 0.15)';
        link.style.textDecoration = 'none';
        link.style.fontFamily = '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif';
        link.style.fontSize = '12px';
        link.style.fontWeight = '500';
        link.style.transition = 'transform 0.2s ease, box-shadow 0.2s ease';

        link.onmouseover = () => {
          link.style.transform = 'translateY(-2px)';
          link.style.boxShadow = '0 6px 16px rgba(0, 0, 0, 0.2)';
        };

        link.onmouseout = () => {
          link.style.transform = 'translateY(0)';
          link.style.boxShadow = '0 4px 12px rgba(0, 0, 0, 0.15)';
        };

        const icon = document.createElement('span');
        icon.innerHTML = '⚡';

        const textSpan = document.createElement('span');
        textSpan.innerText = text;

        link.appendChild(icon);
        link.appendChild(textSpan);
        badgeContainer.appendChild(link);
      } else {
        badgeContainer.style.position = 'static';
        badgeContainer.style.marginTop = '40px';
        badgeContainer.style.padding = '20px';
        badgeContainer.style.textAlign = 'center';
        badgeContainer.style.borderTop = '1px solid ' + (theme === 'dark' ? '#333333' : '#EAEAEA');

        const link = document.createElement('a');
        link.href = 'https://app.onehumancorp.com/invite/' + tenant;
        link.target = '_blank';
        link.style.display = 'inline-flex';
        link.style.alignItems = 'center';
        link.style.gap = '8px';
        link.style.color = '#86868B';
        link.style.textDecoration = 'none';
        link.style.fontFamily = '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif';
        link.style.fontSize = '14px';

        link.onmouseover = () => {
          link.style.color = theme === 'dark' ? '#FFFFFF' : '#1D1D1F';
        };

        link.onmouseout = () => {
          link.style.color = '#86868B';
        };

        const icon = document.createElement('span');
        icon.innerHTML = '⚡';

        const textSpan = document.createElement('span');
        textSpan.innerText = text;

        link.appendChild(icon);
        link.appendChild(textSpan);
        badgeContainer.appendChild(link);
      }

      document.body.appendChild(badgeContainer);
    })();
  `;

  return new NextResponse(jsCode, {
    status: 200,
    headers: {
      'Content-Type': 'application/javascript',
      'Cache-Control': 'public, max-age=3600',
    },
  });
}
