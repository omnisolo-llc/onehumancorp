import { NextRequest, NextResponse } from 'next/server';

function escapeHtml(unsafe: string) {
  return unsafe
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

export async function GET(request: NextRequest) {
  const searchParams = request.nextUrl.searchParams;

  const tenant = escapeHtml(searchParams.get('tenant') || 'embed');
  const title = escapeHtml(searchParams.get('title') || 'Mystery Discount Box');
  const desc = escapeHtml(searchParams.get('desc') || 'Unlock a surprise discount up to 50% off! Enter your email to reveal.');
  const codesStr = escapeHtml(searchParams.get('codes') || 'MYSTERY10,MYSTERY20,MYSTERY50');
  const theme = searchParams.get('theme') || 'light';
  const showBranding = searchParams.get('branding') !== 'false';

  const isDark = theme === 'dark';
  const bgColor = isDark ? '#111827' : '#ffffff';
  const textColor = isDark ? '#f9fafb' : '#111827';
  const secondaryTextColor = isDark ? '#9ca3af' : '#4b5563';
  const inputBg = isDark ? '#374151' : '#f9fafb';
  const inputBorder = isDark ? '#4b5563' : '#e5e7eb';
  const boxBg = isDark ? 'linear-gradient(135deg, #4f46e5 0%, #7c3aed 100%)' : 'linear-gradient(135deg, #6366f1 0%, #a855f7 100%)';
  const btnHover = isDark ? '#4338ca' : '#4f46e5';

  const html = `<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <style>
    body {
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
      margin: 0; padding: 0;
      background-color: transparent;
      color: ${textColor};
    }
    .widget-container {
      width: 100%; height: 100vh;
      display: flex; flex-direction: column; align-items: center; justify-content: center;
      background: ${bgColor};
      border-radius: 16px;
      padding: 24px; box-sizing: border-box;
      text-align: center;
      position: relative;
    }
    .title { font-size: 24px; font-weight: 800; margin-bottom: 8px; }
    .desc { font-size: 14px; color: ${secondaryTextColor}; margin-bottom: 24px; line-height: 1.5; }

    .mystery-box {
      width: 120px; height: 120px;
      background: ${boxBg};
      border-radius: 24px;
      display: flex; align-items: center; justify-content: center;
      font-size: 48px;
      box-shadow: 0 10px 25px -5px rgba(99, 102, 241, 0.4);
      margin-bottom: 24px;
      transition: transform 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
      cursor: pointer;
    }
    .mystery-box:hover { transform: scale(1.05) rotate(-5deg); }
    .mystery-box.opened {
      transform: scale(1.1);
      background: ${isDark ? '#1f2937' : '#f3f4f6'};
      box-shadow: none;
      border: 2px dashed ${isDark ? '#4b5563' : '#d1d5db'};
    }

    .form-group { width: 100%; max-width: 280px; display: flex; flex-direction: column; gap: 12px; }
    input[type="email"] {
      width: 100%; padding: 12px 16px; border-radius: 12px;
      border: 1px solid ${inputBorder}; background: ${inputBg}; color: ${textColor};
      font-size: 14px; box-sizing: border-box; outline: none;
    }
    input[type="email"]:focus { border-color: #6366f1; }

    button {
      width: 100%; padding: 12px 16px; border-radius: 12px;
      background: ${isDark ? '#4f46e5' : '#6366f1'}; color: white;
      border: none; font-size: 14px; font-weight: 600; cursor: pointer;
      transition: background 0.2s; box-sizing: border-box;
    }
    button:hover { background: ${btnHover}; }

    .result-container { display: none; width: 100%; max-width: 280px; flex-direction: column; align-items: center; gap: 16px; }
    .code-display {
      font-family: monospace; font-size: 24px; font-weight: 800;
      letter-spacing: 2px; color: ${isDark ? '#a855f7' : '#9333ea'};
      padding: 12px 24px; background: ${isDark ? '#3b0764' : '#f3e8ff'};
      border-radius: 12px; border: 2px dashed ${isDark ? '#9333ea' : '#d8b4fe'};
      user-select: all;
    }

    .viral-actions {
      display: flex; gap: 8px; width: 100%;
    }
    .btn-secondary {
      background: ${isDark ? '#374151' : '#f3f4f6'}; color: ${textColor};
      flex: 1; padding: 10px; font-size: 13px;
    }
    .btn-secondary:hover { background: ${isDark ? '#4b5563' : '#e5e7eb'}; }
    .btn-x { background: #000; color: #fff; }
    .btn-x:hover { background: #333; }

    .branding { margin-top: auto; padding-top: 16px; }
    .branding a {
      font-size: 12px; font-weight: 600; text-decoration: none;
      color: ${secondaryTextColor}; display: inline-flex; align-items: center; gap: 4px;
    }
    .branding a:hover { color: ${textColor}; }

    #double-discount { display: none; color: #10b981; font-weight: bold; font-size: 14px; margin-top: 8px; }
  </style>
</head>
<body>
  <div class="widget-container">
    <h2 class="title" id="main-title">${title}</h2>
    <p class="desc" id="main-desc">${desc}</p>

    <div class="mystery-box" id="box">🎁</div>

    <div class="form-group" id="form-group">
      <input type="email" id="email" placeholder="Enter your email" required />
      <button id="unlock-btn">Unlock My Discount</button>
    </div>

    <div class="result-container" id="result-container">
      <div class="code-display" id="code-display"></div>
      <p id="double-discount">🎉 Discount Doubled!</p>
      <p style="font-size: 13px; margin: 0; color: ${secondaryTextColor};">Share to double your discount chance next time!</p>
      <div class="viral-actions">
        <button class="btn-secondary" id="copy-btn">Copy Link</button>
        <button class="btn-secondary btn-x" id="share-x-btn">Share on X</button>
      </div>
    </div>

    ${showBranding ? `
    <div class="branding">
      <a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}&source=mystery_discount_widget" target="_blank">
        ⚡ Powered by OHC
      </a>
    </div>
    ` : ''}
  </div>

  <script>
    const codes = "${codesStr}".split(',').map(c => c.trim()).filter(Boolean);
    const box = document.getElementById('box');
    const formGroup = document.getElementById('form-group');
    const resultContainer = document.getElementById('result-container');
    const unlockBtn = document.getElementById('unlock-btn');
    const emailInput = document.getElementById('email');
    const codeDisplay = document.getElementById('code-display');
    const mainTitle = document.getElementById('main-title');
    const mainDesc = document.getElementById('main-desc');
    const doubleDiscountMsg = document.getElementById('double-discount');

    const referralLink = "https://${tenant}.ohc.app?ref=mystery";
    const shareText = "I just unlocked a mystery discount at ${tenant}! 🎁 Get yours here: " + referralLink;

    unlockBtn.addEventListener('click', () => {
      if (!emailInput.value || !emailInput.value.includes('@')) {
        alert('Please enter a valid email address.');
        return;
      }

      // Simulate API call and pick random code
      unlockBtn.innerText = 'Unlocking...';
      unlockBtn.disabled = true;

      setTimeout(() => {
        const randomCode = codes[Math.floor(Math.random() * codes.length)] || 'MYSTERY';

        formGroup.style.display = 'none';
        box.classList.add('opened');
        box.innerText = '✨';

        mainTitle.innerText = "You've Unlocked:";
        mainDesc.style.display = 'none';

        codeDisplay.innerText = randomCode;
        resultContainer.style.display = 'flex';
      }, 800);
    });

    document.getElementById('copy-btn').addEventListener('click', (e) => {
      navigator.clipboard.writeText(shareText);
      e.target.innerText = 'Copied!';
      doubleDiscountMsg.style.display = 'block';
      setTimeout(() => e.target.innerText = 'Copy Link', 2000);
    });

    document.getElementById('share-x-btn').addEventListener('click', () => {
      window.open('https://twitter.com/intent/tweet?text=' + encodeURIComponent(shareText), '_blank');
      doubleDiscountMsg.style.display = 'block';
    });
  </script>
</body>
</html>`;

  return new NextResponse(html, {
    headers: {
      'Content-Type': 'text/html; charset=utf-8',
      'Cache-Control': 'public, max-age=60',
    },
  });
}
