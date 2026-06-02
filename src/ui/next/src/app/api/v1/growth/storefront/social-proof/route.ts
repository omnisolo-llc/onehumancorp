import { NextResponse } from 'next/server';

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url);
  // Sanitize the input to prevent XSS. We only expect alphanumeric characters and dashes.
  const rawTenant = searchParams.get('store') || 'my-store';
  const tenant = rawTenant.replace(/[^a-zA-Z0-9-]/g, '');

  const scriptContent = `
(function() {
  if (document.getElementById('ohc-social-proof-container')) return;

  const container = document.createElement('div');
  container.id = 'ohc-social-proof-container';
  Object.assign(container.style, {
    position: 'fixed',
    bottom: '20px',
    left: '20px',
    zIndex: '9999',
    display: 'flex',
    flexDirection: 'column',
    gap: '10px',
    pointerEvents: 'none'
  });
  document.body.appendChild(container);

  const style = document.createElement('style');
  style.textContent = \`
    @keyframes ohcSlideUpFadeIn {
      0% { opacity: 0; transform: translateY(20px); }
      10% { opacity: 1; transform: translateY(0); }
      90% { opacity: 1; transform: translateY(0); }
      100% { opacity: 0; transform: translateY(-20px); }
    }
    .ohc-sp-popup {
      background: rgba(255, 255, 255, 0.9);
      backdrop-filter: blur(10px);
      border: 1px solid rgba(0,0,0,0.05);
      border-radius: 12px;
      padding: 12px 16px;
      box-shadow: 0 10px 25px rgba(0,0,0,0.1);
      display: flex;
      align-items: center;
      gap: 12px;
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
      animation: ohcSlideUpFadeIn 6s ease-in-out forwards;
      pointer-events: auto;
      max-width: 320px;
    }
    .ohc-sp-icon {
      font-size: 24px;
      line-height: 1;
    }
    .ohc-sp-content {
      display: flex;
      flex-direction: column;
      gap: 2px;
    }
    .ohc-sp-title {
      font-size: 13px;
      color: #111827;
      margin: 0;
      font-weight: 500;
    }
    .ohc-sp-time {
      font-size: 11px;
      color: #6b7280;
      margin: 0;
    }
    .ohc-sp-footer {
      font-size: 10px;
      margin-top: 4px;
    }
    .ohc-sp-footer a {
      color: #9ca3af;
      text-decoration: none;
      font-weight: 600;
      transition: color 0.2s;
    }
    .ohc-sp-footer a:hover {
      color: #4b5563;
    }
  \`;
  document.head.appendChild(style);

  // Fallback items based on generic e-commerce terms
  const events = [
    { name: "Sarah from Seattle", action: "just purchased", item: "an item from this store", time: "2 minutes ago", icon: "🛍️" },
    { name: "Mike from Austin", action: "just ordered", item: "a product", time: "15 minutes ago", icon: "📦" },
    { name: "Emily from NY", action: "just purchased", item: "an item", time: "1 hour ago", icon: "✨" },
    { name: "Alex from Chicago", action: "just bought", item: "a product", time: "3 hours ago", icon: "🛒" }
  ];

  function showNextEvent() {
    const event = events[Math.floor(Math.random() * events.length)];

    const popup = document.createElement('div');
    popup.className = 'ohc-sp-popup';

    popup.innerHTML = \`
      <div class="ohc-sp-icon">\${event.icon}</div>
      <div class="ohc-sp-content">
        <p class="ohc-sp-title"><strong>\${event.name}</strong> \${event.action} <strong>\${event.item}</strong></p>
        <p class="ohc-sp-time">\${event.time}</p>
        <div class="ohc-sp-footer">
          <a href="https://ohc.store/join?ref=${tenant}" target="_blank" rel="noopener noreferrer">⚡ Powered by OHC</a>
        </div>
      </div>
    \`;

    container.appendChild(popup);

    setTimeout(() => {
      if (popup.parentNode === container) {
        container.removeChild(popup);
      }
    }, 6000);
  }

  // Initial delay, then show every 12-25 seconds
  setTimeout(() => {
    showNextEvent();
    setInterval(showNextEvent, Math.floor(Math.random() * 13000) + 12000);
  }, 3000);

})();
  `;

  return new NextResponse(scriptContent, {
    headers: {
      'Content-Type': 'application/javascript',
      'Cache-Control': 'public, max-age=3600',
    },
  });
}
