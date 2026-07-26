import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  const target = searchParams.get('target') || '1000';
  const reward = searchParams.get('reward') || 'Free shipping for everyone!';
  const tenant = searchParams.get('tenant') || 'default';

  // In a real app we'd fetch the current progress from the DB based on tenant
  // For the embed widget, we'll return a static/simulated interactive widget
  const currentProgress = Math.floor(Math.random() * (parseInt(target, 10) * 0.8)) + 1;
  const percentage = Math.min(100, Math.round((currentProgress / parseInt(target, 10)) * 100));

  const html = `
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Community Goal Tracker</title>
  <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@400;600;700&display=swap" rel="stylesheet">
  <style>
    body {
      font-family: 'Outfit', sans-serif;
      margin: 0;
      padding: 0;
      background: transparent;
      display: flex;
      justify-content: center;
      align-items: center;
      height: 100%;
    }
    .widget-container {
      background: linear-gradient(145deg, #ffffff 0%, #f8f9fa 100%);
      border: 1px solid rgba(0, 102, 255, 0.1);
      border-radius: 16px;
      padding: 24px;
      width: 100%;
      max-width: 380px;
      box-shadow: 0 8px 30px rgba(0, 0, 0, 0.04);
      text-align: center;
      position: relative;
      overflow: hidden;
    }
    .widget-container::before {
      content: '';
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      height: 4px;
      background: linear-gradient(90deg, #0066FF, #00c6ff);
    }
    .header {
      font-weight: 700;
      font-size: 20px;
      color: #1d1d1f;
      margin-bottom: 8px;
    }
    .reward {
      font-size: 15px;
      color: #0066FF;
      font-weight: 600;
      margin-bottom: 20px;
      background: rgba(0, 102, 255, 0.08);
      padding: 6px 12px;
      border-radius: 20px;
      display: inline-block;
    }
    .progress-wrapper {
      background: #e2e8f0;
      border-radius: 12px;
      height: 24px;
      width: 100%;
      margin-bottom: 12px;
      overflow: hidden;
      position: relative;
    }
    .progress-fill {
      background: linear-gradient(90deg, #0066FF, #00c6ff);
      height: 100%;
      width: ${percentage}%;
      border-radius: 12px;
      transition: width 1s ease-in-out;
      display: flex;
      align-items: center;
      justify-content: flex-end;
      padding-right: 8px;
      color: white;
      font-size: 12px;
      font-weight: 700;
    }
    .stats {
      display: flex;
      justify-content: space-between;
      font-size: 14px;
      color: #64748b;
      font-weight: 500;
      margin-bottom: 20px;
    }
    .share-btn {
      background: #1d1d1f;
      color: white;
      border: none;
      border-radius: 10px;
      padding: 12px 20px;
      font-size: 15px;
      font-weight: 600;
      width: 100%;
      cursor: pointer;
      transition: transform 0.1s, background 0.2s;
    }
    .share-btn:hover {
      background: #000;
    }
    .share-btn:active {
      transform: scale(0.98);
    }
    .footer-brand {
      margin-top: 16px;
      font-size: 11px;
      color: #94a3b8;
      text-decoration: none;
      font-weight: 500;
    }
  </style>
</head>
<body>
  <div class="widget-container">
    <div class="header">Community Goal</div>
    <div class="reward">🎁 Unlock: ${reward}</div>

    <div class="progress-wrapper">
      <div class="progress-fill">${percentage}%</div>
    </div>

    <div class="stats">
      <span>${currentProgress} joined</span>
      <span>Goal: ${target}</span>
    </div>

    <button class="share-btn" onclick="alert('Sharing logic triggered!')">Share to help us reach the goal!</button>

    <div style="margin-top:12px;">
      <a href="https://ohc.app/invite?ref=${tenant}" target="_blank" class="footer-brand">⚡ Powered by OHC</a>
    </div>
  </div>
</body>
</html>
  `;

  return new NextResponse(html, {
    headers: { 'Content-Type': 'text/html' },
  });
}
