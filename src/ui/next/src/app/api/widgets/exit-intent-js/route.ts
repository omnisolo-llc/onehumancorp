import { NextResponse } from 'next/server';

export async function GET(req: Request) {
    const script = `
(function() {
  function initExitIntent() {
    var container = document.getElementById('ohc-exit-intent');
    if (!container) return;

    var tenant = container.getAttribute('data-tenant');
    var discount = container.getAttribute('data-discount');
    var headline = container.getAttribute('data-headline');
    var subheading = container.getAttribute('data-subheading');
    var btnText = container.getAttribute('data-btn');
    var theme = container.getAttribute('data-theme') || 'light';
    var showBranding = container.getAttribute('data-branding') === 'true';

    var modal = document.createElement('div');
    modal.style.position = 'fixed';
    modal.style.top = '0';
    modal.style.left = '0';
    modal.style.width = '100vw';
    modal.style.height = '100vh';
    modal.style.backgroundColor = 'rgba(0,0,0,0.6)';
    modal.style.zIndex = '999999';
    modal.style.display = 'none';
    modal.style.justifyContent = 'center';
    modal.style.alignItems = 'center';

    var card = document.createElement('div');
    card.style.background = theme === 'dark' ? '#1D1D1F' : '#ffffff';
    card.style.color = theme === 'dark' ? '#ffffff' : '#1D1D1F';
    card.style.padding = '32px';
    card.style.borderRadius = '16px';
    card.style.boxShadow = '0 25px 50px -12px rgba(0, 0, 0, 0.25)';
    card.style.maxWidth = '400px';
    card.style.width = '90%';
    card.style.textAlign = 'center';
    card.style.fontFamily = 'sans-serif';

    var title = document.createElement('h3');
    title.innerText = headline;
    title.style.margin = '0 0 12px 0';
    title.style.fontSize = '24px';

    var desc = document.createElement('p');
    desc.innerText = subheading;
    desc.style.margin = '0 0 24px 0';
    desc.style.color = theme === 'dark' ? '#9CA3AF' : '#4B5563';

    var btn = document.createElement('button');
    btn.innerText = btnText;
    btn.style.width = '100%';
    btn.style.padding = '12px';
    btn.style.background = '#4F46E5';
    btn.style.color = 'white';
    btn.style.border = 'none';
    btn.style.borderRadius = '8px';
    btn.style.fontSize = '16px';
    btn.style.fontWeight = 'bold';
    btn.style.cursor = 'pointer';

    btn.onclick = function() {
      alert('Discount code applied!');
      modal.style.display = 'none';
    };

    var closeBtn = document.createElement('button');
    closeBtn.innerText = 'No thanks';
    closeBtn.style.marginTop = '16px';
    closeBtn.style.background = 'transparent';
    closeBtn.style.border = 'none';
    closeBtn.style.color = theme === 'dark' ? '#9CA3AF' : '#6B7280';
    closeBtn.style.cursor = 'pointer';
    closeBtn.style.textDecoration = 'underline';

    closeBtn.onclick = function() {
        modal.style.display = 'none';
    };

    card.appendChild(title);
    card.appendChild(desc);
    card.appendChild(btn);
    card.appendChild(closeBtn);

    if (showBranding) {
      var branding = document.createElement('div');
      branding.style.marginTop = '24px';
      branding.style.paddingTop = '16px';
      branding.style.borderTop = '1px solid ' + (theme === 'dark' ? '#374151' : '#E5E7EB');
      var backendUrl = typeof window !== 'undefined' ? (window.location.origin.includes('localhost') ? window.location.origin : 'https://ohc.app') : 'https://ohc.app';
      branding.innerHTML = '<a href="' + backendUrl + '/api/v1/growth/referrals/click?target=/onboarding&ref=' + encodeURIComponent(tenant) + '" target="_blank" style="color: #6B7280; text-decoration: none; font-size: 12px; font-weight: 600;">⚡ Powered by OHC</a>';
      card.appendChild(branding);
    }

    modal.appendChild(card);
    document.body.appendChild(modal);

    var hasTriggered = false;
    document.addEventListener('mouseout', function(e) {
      if (e.clientY < 0 && !hasTriggered) {
        hasTriggered = true;
        modal.style.display = 'flex';
      }
    });
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initExitIntent);
  } else {
    initExitIntent();
  }
})();
    `;

    return new NextResponse(script, {
        headers: {
            'Content-Type': 'application/javascript',
        },
    });
}
