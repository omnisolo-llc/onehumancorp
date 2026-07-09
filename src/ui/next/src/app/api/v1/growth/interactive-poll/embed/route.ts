import { NextResponse } from 'next/server';

const escapeHtml = (unsafe: string) => {
  if (!unsafe) return unsafe;
  return unsafe
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;")
      .replace(/'/g, "&#039;");
};

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const tenant = searchParams.get('tenant') || 'my-store';
  const q = escapeHtml(searchParams.get('q') || 'What flavor should we make next?');
  const opts = searchParams.get('opts') || 'Chocolate,Vanilla,Strawberry';
  const theme = searchParams.get('theme') || 'light';
  const email = searchParams.get('email') === 'true';
  const hideBranding = searchParams.get('hideBranding') === 'true';

  const options = opts.split(',').filter(o => o.trim() !== '').map(escapeHtml);

  const isDark = theme === 'dark';
  const bgColor = isDark ? '#111111' : '#ffffff';
  const textColor = isDark ? '#ffffff' : '#1D1D1F';
  const borderColor = isDark ? '#333333' : '#e5e7eb';
  const optBg = isDark ? 'transparent' : 'transparent';
  const optHoverBg = isDark ? '#1f2937' : '#f9fafb';
  const btnBg = '#0071E3';
  const btnHoverBg = '#0077ED';

  const html = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Interactive Poll</title>
  <style>
    body {
      margin: 0;
      padding: 24px;
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
      background-color: ${bgColor};
      color: ${textColor};
      display: flex;
      flex-direction: column;
      height: 100vh;
      box-sizing: border-box;
      border: 1px solid ${borderColor};
      border-radius: 16px;
    }
    h3 {
      margin-top: 0;
      margin-bottom: 20px;
      font-size: 20px;
      font-weight: 700;
      text-align: center;
    }
    .options {
      display: flex;
      flex-direction: column;
      gap: 12px;
      margin-bottom: 24px;
      flex: 1;
    }
    .option-btn {
      width: 100%;
      text-align: left;
      padding: 12px 16px;
      border-radius: 12px;
      border: 1px solid ${borderColor};
      background-color: ${optBg};
      color: ${textColor};
      cursor: pointer;
      display: flex;
      align-items: center;
      justify-content: space-between;
      transition: all 0.2s;
      font-size: 15px;
      font-weight: 500;
      font-family: inherit;
    }
    .option-btn:hover {
      background-color: ${optHoverBg};
    }
    .radio-circle {
      width: 16px;
      height: 16px;
      border-radius: 50%;
      border: 2px solid ${isDark ? '#4b5563' : '#d1d5db'};
      display: flex;
      align-items: center;
      justify-content: center;
      transition: all 0.2s;
    }
    .option-btn:hover .radio-circle {
      border-color: ${btnBg};
    }
    .option-btn.selected {
      border-color: ${btnBg};
      background-color: ${isDark ? 'rgba(0, 113, 227, 0.1)' : 'rgba(0, 113, 227, 0.05)'};
    }
    .option-btn.selected .radio-circle {
      border-color: ${btnBg};
    }
    .option-btn.selected .radio-circle::after {
      content: '';
      width: 8px;
      height: 8px;
      border-radius: 50%;
      background-color: ${btnBg};
    }
    .email-input {
      width: 100%;
      padding: 10px 16px;
      border-radius: 12px;
      border: 1px solid ${borderColor};
      background-color: ${isDark ? '#1f2937' : '#f9fafb'};
      color: ${textColor};
      box-sizing: border-box;
      margin-bottom: 16px;
      outline: none;
      font-family: inherit;
      font-size: 14px;
    }
    .email-input:focus {
      border-color: ${btnBg};
      box-shadow: 0 0 0 2px rgba(0, 113, 227, 0.2);
    }
    .submit-btn {
      width: 100%;
      padding: 10px 16px;
      border-radius: 12px;
      border: none;
      background-color: ${btnBg};
      color: white;
      font-weight: 500;
      font-size: 14px;
      cursor: pointer;
      font-family: inherit;
      transition: background-color 0.2s;
    }
    .submit-btn:hover {
      background-color: ${btnHoverBg};
    }
    .submit-btn:disabled {
      background-color: ${isDark ? '#374151' : '#e5e7eb'};
      color: ${isDark ? '#9ca3af' : '#9ca3af'};
      cursor: not-allowed;
    }
    .success-state {
      display: none;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      height: 100%;
      text-align: center;
    }
    .icon {
      font-size: 32px;
      margin-bottom: 16px;
    }
  </style>
</head>
<body>
  <div id="poll-container">
    <h3>${q}</h3>
    <div class="options">
      ${options.map((opt, i) => `
        <button class="option-btn" onclick="selectOption(${i})" id="opt-${i}">
          <span>${opt}</span>
          <div class="radio-circle"></div>
        </button>
      `).join('')}
    </div>

    ${email ? `
      <input type="email" id="email" class="email-input" placeholder="Enter your email to vote" />
    ` : ''}

    <button id="submitBtn" class="submit-btn" onclick="submitVote()" disabled>
      Vote Now
    </button>
  </div>

  <div id="success-container" class="success-state">
    <div class="icon">✅</div>
    <h3>Thanks for voting!</h3>
    <p style="font-size: 14px; color: ${isDark ? '#9ca3af' : '#6b7280'};">Your vote has been recorded.</p>
  </div>

  <script>
    let selectedIdx = -1;

    function selectOption(idx) {
      selectedIdx = idx;

      // Reset all options
      document.querySelectorAll('.option-btn').forEach((btn, i) => {
        if (i === idx) {
          btn.classList.add('selected');
        } else {
          btn.classList.remove('selected');
        }
      });

      checkValidity();
    }

    function checkValidity() {
      const btn = document.getElementById('submitBtn');
      const emailInput = document.getElementById('email');

      let isValid = selectedIdx !== -1;

      if (emailInput) {
        isValid = isValid && emailInput.value.includes('@') && emailInput.value.includes('.');
      }

      btn.disabled = !isValid;
    }

    ${email ? `
    document.getElementById('email').addEventListener('input', checkValidity);
    ` : ''}

    function submitVote() {
      if (document.getElementById('submitBtn').disabled) return;

      // In a real app, this would send data to the server
      document.getElementById('poll-container').style.display = 'none';
      document.getElementById('success-container').style.display = 'flex';
    }
  </script>
</body>
</html>`;

  return new NextResponse(html, {
    headers: {
      'Content-Type': 'text/html; charset=utf-8',
      'Cache-Control': 'public, max-age=3600',
    },
  });
}
