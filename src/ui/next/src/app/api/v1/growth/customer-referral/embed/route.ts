import { NextRequest, NextResponse } from 'next/server';

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const tenant = searchParams.get('tenant') || 'unknown';
  const give = searchParams.get('give') || '10';
  const get = searchParams.get('get') || '10';

  const html = `
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Give $${give}, Get $${get}</title>
  <style>
    body {
      margin: 0;
      padding: 0;
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
      background-color: transparent;
    }
    .widget-container {
      background: rgba(255, 255, 255, 0.9);
      backdrop-filter: blur(10px);
      border: 1px solid rgba(0, 0, 0, 0.1);
      border-radius: 16px;
      padding: 20px;
      text-align: center;
      box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
      max-width: 100%;
      box-sizing: border-box;
    }
    .icon {
      font-size: 32px;
      margin-bottom: 12px;
      display: inline-block;
      background: #ecfdf5;
      padding: 12px;
      border-radius: 50%;
    }
    h2 {
      margin: 0 0 8px 0;
      font-size: 20px;
      color: #111827;
    }
    p {
      margin: 0 0 16px 0;
      font-size: 14px;
      color: #4b5563;
      line-height: 1.4;
    }
    .input-group {
      display: flex;
      gap: 8px;
    }
    input {
      flex: 1;
      padding: 10px 14px;
      border: 1px solid #d1d5db;
      border-radius: 8px;
      font-size: 14px;
      outline: none;
    }
    input:focus {
      border-color: #10b981;
      box-shadow: 0 0 0 2px rgba(16, 185, 129, 0.2);
    }
    button {
      background-color: #10b981;
      color: white;
      border: none;
      border-radius: 8px;
      padding: 10px 20px;
      font-size: 14px;
      font-weight: 600;
      cursor: pointer;
      transition: background-color 0.2s;
    }
    button:hover {
      background-color: #059669;
    }
  </style>
</head>
<body>
  <div class="widget-container">
    <div class="icon">🎁</div>
    <h2>Give $${give}, Get $${get}</h2>
    <p>Enter your email to get your unique referral link. Give a friend $${give} off their first order, and you'll get $${get} when they buy!</p>
    <form onsubmit="event.preventDefault(); alert('Referral link generated! Check your email.');">
      <div class="input-group">
        <input type="email" placeholder="Your email address" required>
        <button type="submit">Get Link</button>
      </div>
    </form>
  </div>
</body>
</html>
`;

  return new NextResponse(html, {
    headers: {
      'Content-Type': 'text/html',
      'Cache-Control': 'public, max-age=3600',
    },
  });
}
