import { NextResponse } from 'next/server';

function escapeHtml(unsafe: string) {
    if (!unsafe) return unsafe;
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
    const reward = searchParams.get('reward') || '$10';
    const themeColor = searchParams.get('themeColor') || '#2563eb';
    const removeBranding = searchParams.get('removeBranding') === 'true';

    const escapedTenant = escapeHtml(tenant);
    const encodedTenant = encodeURIComponent(tenant);
    const escapedReward = escapeHtml(reward);
    const escapedThemeColor = escapeHtml(themeColor);

    const jsCode = `
(function() {
    // Prevent multiple initializations
    if (document.getElementById('ohc-referral-fab')) return;

    // Create wrapper
    const wrapper = document.createElement('div');
    wrapper.id = 'ohc-referral-fab';
    wrapper.style.position = 'fixed';
    wrapper.style.bottom = '24px';
    wrapper.style.right = '24px';
    wrapper.style.zIndex = '999999';
    wrapper.style.fontFamily = '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif';

    // Create FAB
    const fab = document.createElement('div');
    fab.style.width = '56px';
    fab.style.height = '56px';
    fab.style.borderRadius = '50%';
    fab.style.backgroundColor = '${escapedThemeColor}';
    fab.style.boxShadow = '0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)';
    fab.style.display = 'flex';
    fab.style.alignItems = 'center';
    fab.style.justifyContent = 'center';
    fab.style.cursor = 'pointer';
    fab.style.transition = 'transform 0.2s ease';
    fab.style.color = 'white';

    // Gift icon SVG
    fab.innerHTML = '<svg style="width: 24px; height: 24px;" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v13m0-13V6a2 2 0 112 2h-2zm0 0V5.5A2.5 2.5 0 109.5 8H12zm-7 4h14M5 12a2 2 0 110-4h14a2 2 0 110 4M5 12v7a2 2 0 002 2h10a2 2 0 002-2v-7" /></svg>';

    // Add hover effect
    fab.onmouseover = () => { fab.style.transform = 'scale(1.05)'; };
    fab.onmouseout = () => { fab.style.transform = 'scale(1)'; };

    // Create Popover
    const popover = document.createElement('div');
    popover.style.position = 'absolute';
    popover.style.bottom = '70px';
    popover.style.right = '0';
    popover.style.width = '300px';
    popover.style.backgroundColor = 'white';
    popover.style.borderRadius = '16px';
    popover.style.boxShadow = '0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)';
    popover.style.padding = '20px';
    popover.style.display = 'none';
    popover.style.transformOrigin = 'bottom right';
    popover.style.transition = 'all 0.2s ease';
    popover.style.opacity = '0';
    popover.style.transform = 'scale(0.95)';
    popover.style.border = '1px solid #f3f4f6';

    const brandingHtml = ${removeBranding} ? '' : \`
        <div style="margin-top: 12px; text-align: center; font-size: 11px; font-weight: 500;">
            <a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${encodedTenant}" target="_blank" style="color: #9ca3af; text-decoration: none;">⚡ Powered by OHC</a>
        </div>
    \`;

    popover.innerHTML = \`
        <h3 style="margin: 0 0 8px 0; color: #111827; font-size: 16px; font-weight: 700;">Get ${escapedReward}</h3>
        <p style="margin: 0 0 16px 0; color: #4b5563; font-size: 14px; line-height: 1.4;">Give a friend ${escapedReward} off their first order, and get ${escapedReward} when they buy!</p>

        <form id="ohc-referral-form" style="display: flex; flex-direction: column; gap: 12px; margin: 0;">
            <input type="email" id="ohc-referral-email" placeholder="Enter your email" required style="width: 100%; padding: 10px 12px; border: 1px solid #e5e7eb; border-radius: 8px; font-size: 14px; box-sizing: border-box; background: #f9fafb;" />
            <button type="submit" style="width: 100%; padding: 10px; background-color: ${escapedThemeColor}; color: white; border: none; border-radius: 8px; font-weight: 600; cursor: pointer; font-size: 14px; transition: opacity 0.2s;">
                Get Share Link
            </button>
        </form>

        <div id="ohc-referral-success" style="display: none; flex-direction: column; gap: 12px;">
            <div style="color: #10b981; font-weight: 500; font-size: 14px;">Here is your link!</div>
            <input type="text" id="ohc-referral-link" readonly value="https://ohc.app/share?ref=${encodedTenant}_xyz" style="width: 100%; padding: 10px 12px; border: 1px solid #e5e7eb; border-radius: 8px; font-size: 12px; box-sizing: border-box; background: #f3f4f6; color: #374151;" />
            <button id="ohc-copy-btn" style="width: 100%; padding: 10px; background-color: #f3f4f6; color: #374151; border: none; border-radius: 8px; font-weight: 600; cursor: pointer; font-size: 14px; transition: background-color 0.2s;">
                Copy Link
            </button>
        </div>
        \${brandingHtml}
    \`;

    wrapper.appendChild(popover);
    wrapper.appendChild(fab);
    document.body.appendChild(wrapper);

    // Toggle popover
    let isOpen = false;
    fab.addEventListener('click', () => {
        isOpen = !isOpen;
        if (isOpen) {
            popover.style.display = 'block';
            // Slight delay to allow display: block to apply before transition
            setTimeout(() => {
                popover.style.opacity = '1';
                popover.style.transform = 'scale(1)';
            }, 10);
            fab.innerHTML = '<svg style="width: 24px; height: 24px;" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" /></svg>';
        } else {
            popover.style.opacity = '0';
            popover.style.transform = 'scale(0.95)';
            setTimeout(() => {
                popover.style.display = 'none';
            }, 200);
            fab.innerHTML = '<svg style="width: 24px; height: 24px;" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v13m0-13V6a2 2 0 112 2h-2zm0 0V5.5A2.5 2.5 0 109.5 8H12zm-7 4h14M5 12a2 2 0 110-4h14a2 2 0 110 4M5 12v7a2 2 0 002 2h10a2 2 0 002-2v-7" /></svg>';
        }
    });

    // Form logic
    const form = document.getElementById('ohc-referral-form');
    const successDiv = document.getElementById('ohc-referral-success');
    const copyBtn = document.getElementById('ohc-copy-btn');
    const linkInput = document.getElementById('ohc-referral-link');

    form.addEventListener('submit', (e) => {
        e.preventDefault();
        const email = document.getElementById('ohc-referral-email').value;
        if (email) {
            fetch('/api/v1/growth/referrals/generate', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ tenantId: '${escapedTenant}', email: email })
            }).then(res => {
                if (res.ok) return res.json();
                throw new Error('Backend failed');
            }).then(data => {
                form.style.display = 'none';
                successDiv.style.display = 'flex';
                if (data && data.link) {
                    linkInput.value = data.link;
                }
            }).catch(err => {
                console.error('Failed to generate referral', err);
                // Fail gracefully, could show error message
            });
        }
    });

    copyBtn.addEventListener('click', () => {
        linkInput.select();
        document.execCommand('copy');
        const originalText = copyBtn.innerText;
        copyBtn.innerText = 'Copied!';
        setTimeout(() => {
            copyBtn.innerText = originalText;
        }, 2000);
    });

})();
    `;

    return new NextResponse(jsCode, {
        headers: {
            'Content-Type': 'application/javascript; charset=utf-8',
            'Cache-Control': 'public, max-age=300, s-maxage=300'
        },
    });
}