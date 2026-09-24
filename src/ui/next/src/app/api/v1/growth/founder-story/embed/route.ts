import { NextResponse } from 'next/server';

function escapeHtml(unsafe: string) {
  if (!unsafe) return '';
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
  const founderName = searchParams.get('founder_name') || 'Maya';
  const storyText = searchParams.get('story_text') || 'I started this business with a dream to bring the best quality to our community. Share this page and support our journey!';
  const reward = searchParams.get('reward') || '15% Off Your Next Order';
  const theme = searchParams.get('theme') || 'light';
  const hideBranding = searchParams.get('hideBranding') === 'true';

  const isDark = theme === 'dark';
  const bg = isDark ? '#1d1d1f' : '#ffffff';
  const text = isDark ? '#f5f5f7' : '#1d1d1f';
  const textSecondary = isDark ? '#a1a1a6' : '#86868b';
  const cardBg = isDark ? 'rgba(255, 255, 255, 0.05)' : '#f9fafb';
  const border = isDark ? 'rgba(255, 255, 255, 0.1)' : '#e5e7eb';
  const primary = '#0066FF';

  const html = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>${escapeHtml(founderName)}'s Story</title>
  <style>
    * { box-sizing: border-box; margin: 0; padding: 0; }
    body {
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
      background-color: ${bg};
      color: ${text};
      padding: 24px;
      display: flex;
      flex-direction: column;
      justify-content: space-between;
      min-height: 100vh;
      border-radius: 16px;
    }
    .header {
      display: flex;
      align-items: center;
      gap: 12px;
      margin-bottom: 16px;
    }
    .avatar {
      width: 48px;
      height: 48px;
      border-radius: 50%;
      background: linear-gradient(135deg, #0066FF 0%, #a855f7 100%);
      display: flex;
      align-items: center;
      justify-content: center;
      color: white;
      font-weight: 700;
      font-size: 20px;
    }
    .founder-info h3 {
      font-size: 18px;
      font-weight: 700;
      margin-bottom: 2px;
    }
    .founder-info span {
      font-size: 12px;
      color: ${textSecondary};
      text-transform: uppercase;
      letter-spacing: 0.5px;
    }
    .story-body {
      background: ${cardBg};
      border: 1px solid ${border};
      border-radius: 12px;
      padding: 16px;
      margin-bottom: 16px;
      font-size: 14px;
      line-height: 1.6;
      color: ${text};
    }
    .reward-box {
      display: flex;
      align-items: center;
      gap: 10px;
      padding: 12px 16px;
      background: rgba(0, 102, 255, 0.08);
      border: 1px dashed ${primary};
      border-radius: 10px;
      margin-bottom: 16px;
    }
    .reward-icon {
      font-size: 20px;
    }
    .reward-text {
      font-size: 13px;
      font-weight: 600;
      color: ${primary};
    }
    .branding {
      text-align: center;
      font-size: 12px;
      color: ${textSecondary};
      padding-top: 8px;
    }
    .branding a {
      color: ${textSecondary};
      text-decoration: none;
      font-weight: 600;
    }
    .branding a:hover {
      text-decoration: underline;
    }
  </style>
</head>
<body>
  <div>
    <div class="header">
      <div class="avatar">${escapeHtml(founderName.charAt(0).toUpperCase())}</div>
      <div class="founder-info">
        <h3>${escapeHtml(founderName)}</h3>
        <span>Founder</span>
      </div>
    </div>
    <div class="story-body">
      ${escapeHtml(storyText)}
    </div>
    <div class="reward-box">
      <span class="reward-icon">🎁</span>
      <span class="reward-text">Share &amp; Get: ${escapeHtml(reward)}</span>
    </div>
  </div>
  ${!hideBranding ? `
  <div class="branding">
    <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodeURIComponent(tenant)}" target="_blank" rel="noopener noreferrer">
      ⚡ Powered by OmniSolo
    </a>
  </div>
  ` : ''}
</body>
</html>`;

  return new NextResponse(html, {
    headers: {
      'Content-Type': 'text/html; charset=utf-8',
      'Cache-Control': 'public, max-age=300',
    },
  });
}
