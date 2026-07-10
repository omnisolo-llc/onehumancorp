import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  try {
    const { searchParams } = new URL(request.url);
    const tenant = searchParams.get('tenant') || 'default-team';
    const serviceName = searchParams.get('serviceName') || 'Our Service';
    const currency = searchParams.get('currency') || '$';
    const theme = searchParams.get('theme') || 'light';
    const branding = searchParams.get('branding') !== 'false';

    const escapeHtml = (unsafe: string) => {
      if (!unsafe) return unsafe;
      return unsafe
           .replace(/&/g, "&amp;")
           .replace(/</g, "&lt;")
           .replace(/>/g, "&gt;")
           .replace(/"/g, "&quot;")
           .replace(/'/g, "&#039;");
    };

    const encodedTenant = escapeHtml(tenant);
    const encodedServiceName = escapeHtml(serviceName);
    const encodedCurrency = escapeHtml(currency);

    const isDark = theme === 'dark';
    const bgColor = isDark ? '#111827' : '#ffffff';
    const textColor = isDark ? '#f9fafb' : '#111827';
    const inputBg = isDark ? '#374151' : '#f9fafb';
    const inputBorder = isDark ? '#4b5563' : '#d1d5db';
    const buttonBg = '#0066FF';
    const buttonHover = '#0052cc';
    const labelColor = isDark ? '#d1d5db' : '#4b5563';
    const resultBg = isDark ? '#1f2937' : '#f0f9ff';
    const resultBorder = isDark ? '#374151' : '#bae6fd';

    const html = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>ROI Calculator Embed</title>
  <style>
    @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');

    body {
      margin: 0;
      padding: 0;
      font-family: 'Inter', sans-serif;
      background-color: ${bgColor};
      color: ${textColor};
      -webkit-font-smoothing: antialiased;
      display: flex;
      justify-content: center;
      align-items: center;
      min-height: 100vh;
      overflow: hidden;
    }

    .calculator-container {
      width: 100%;
      max-width: 450px;
      padding: 24px;
      box-sizing: border-box;
      display: flex;
      flex-direction: column;
      gap: 16px;
    }

    h3 {
      font-family: 'Outfit', sans-serif;
      margin: 0 0 16px 0;
      font-size: 20px;
      font-weight: 700;
      text-align: center;
    }

    .form-group {
      margin-bottom: 16px;
    }

    label {
      display: block;
      font-size: 13px;
      font-weight: 600;
      margin-bottom: 6px;
      color: ${labelColor};
    }

    input[type="number"] {
      width: 100%;
      padding: 10px 12px;
      border-radius: 8px;
      border: 1px solid ${inputBorder};
      background-color: ${inputBg};
      color: ${textColor};
      font-size: 14px;
      font-family: 'Inter', sans-serif;
      box-sizing: border-box;
      outline: none;
      transition: all 0.2s;
    }

    input[type="number"]:focus {
      border-color: ${buttonBg};
      box-shadow: 0 0 0 2px rgba(0, 102, 255, 0.2);
    }

    button {
      width: 100%;
      padding: 12px;
      border-radius: 8px;
      background-color: ${buttonBg};
      color: #ffffff;
      border: none;
      font-size: 15px;
      font-weight: 600;
      cursor: pointer;
      transition: background-color 0.2s;
      margin-top: 8px;
    }

    button:hover {
      background-color: ${buttonHover};
    }

    .result-container {
      margin-top: 20px;
      padding: 16px;
      border-radius: 12px;
      background-color: ${resultBg};
      border: 1px solid ${resultBorder};
      text-align: center;
      display: none;
      animation: fadeIn 0.3s ease-out;
    }

    .result-label {
      font-size: 13px;
      color: ${labelColor};
      margin-bottom: 4px;
    }

    .result-value {
      font-family: 'Outfit', sans-serif;
      font-size: 28px;
      font-weight: 800;
      color: ${buttonBg};
      margin: 0;
    }

    .branding {
      text-align: center;
      margin-top: 16px;
      font-size: 11px;
    }

    .branding a {
      color: #9ca3af;
      text-decoration: none;
      font-weight: 600;
      text-transform: uppercase;
      letter-spacing: 0.5px;
      transition: color 0.2s;
    }

    .branding a:hover {
      color: #6b7280;
    }

    @keyframes fadeIn {
      from { opacity: 0; transform: translateY(5px); }
      to { opacity: 1; transform: translateY(0); }
    }
  </style>
</head>
<body>
  <div class="calculator-container">
    <h3>${encodedServiceName} ROI</h3>

    <div class="form-group">
      <label for="investment">Estimated Investment</label>
      <input type="number" id="investment" placeholder="${encodedCurrency}1000" min="0" />
    </div>

    <div class="form-group">
      <label for="return">Estimated Revenue/Value Generated</label>
      <input type="number" id="return" placeholder="${encodedCurrency}5000" min="0" />
    </div>

    <button onclick="calculateROI()">Calculate ROI</button>

    <div class="result-container" id="result-box">
      <div class="result-label">Your Estimated ROI</div>
      <p class="result-value" id="roi-value">0%</p>
    </div>

    ${branding ? `
    <div class="branding">
      <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${encodedTenant}&source=viral_roi_calculator" target="_blank">⚡ Powered by OHC</a>
    </div>
    ` : ''}
  </div>

  <script>
    function calculateROI() {
      const investment = parseFloat(document.getElementById('investment').value);
      const returns = parseFloat(document.getElementById('return').value);

      const resultBox = document.getElementById('result-box');
      const roiValue = document.getElementById('roi-value');

      if (isNaN(investment) || isNaN(returns) || investment <= 0) {
        roiValue.innerText = '0%';
        resultBox.style.display = 'block';
        return;
      }

      const roi = ((returns - investment) / investment) * 100;

      let prefix = '';
      if (roi > 0) prefix = '+';

      roiValue.innerText = prefix + roi.toFixed(1) + '%';

      if (roi > 0) {
        roiValue.style.color = '#10b981'; // Green
      } else if (roi < 0) {
        roiValue.style.color = '#ef4444'; // Red
      } else {
        roiValue.style.color = '${buttonBg}';
      }

      resultBox.style.display = 'block';
    }
  </script>
</body>
</html>`;

    return new NextResponse(html, {
      headers: {
        'Content-Type': 'text/html; charset=utf-8',
        'Cache-Control': 'public, max-age=3600'
      }
    });

  } catch (error) {
    console.error('Error generating viral ROI calculator embed:', error);
    return new NextResponse('Error generating embed', { status: 500 });
  }
}