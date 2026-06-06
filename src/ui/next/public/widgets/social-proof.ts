(function() {
    function initSocialProofWidget() {
        const container = document.getElementById('ohc-social-proof');
        if (!container) return;

        const product = container.getAttribute('data-product') || 'A product';
        const location = container.getAttribute('data-location') || 'Someone';
        const time = container.getAttribute('data-time') || 'just now';
        const theme = container.getAttribute('data-theme') || 'light';
        const branding = container.getAttribute('data-branding') === 'true';

        const isDark = theme === 'dark';
        const bgColor = isDark ? '#1D1D1F' : '#ffffff';
        const textColor = isDark ? '#ffffff' : '#111827';
        const borderColor = isDark ? '#333333' : '#e5e7eb';
        const secondaryTextColor = isDark ? 'rgba(255, 255, 255, 0.8)' : 'rgba(17, 24, 39, 0.8)';
        const timeColor = isDark ? 'rgba(255, 255, 255, 0.6)' : 'rgba(17, 24, 39, 0.6)';

        const widgetHtml = `
            <div id="ohc-social-proof-widget" style="
                position: fixed;
                bottom: 24px;
                left: 24px;
                z-index: 999999;
                font-family: -apple-system, BlinkMacSystemFont, 'Inter', 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
                pointer-events: auto;
                opacity: 0;
                transform: translateY(20px);
                transition: opacity 0.5s ease-out, transform 0.5s ease-out;
            ">
                <div style="
                    background: ${bgColor};
                    color: ${textColor};
                    border: 1px solid ${borderColor};
                    border-radius: 12px;
                    padding: 16px;
                    box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
                    display: flex;
                    align-items: center;
                    gap: 16px;
                    max-width: 384px;
                    position: relative;
                ">
                    <button id="ohc-social-proof-close" style="
                        position: absolute;
                        top: 8px;
                        right: 8px;
                        background: none;
                        border: none;
                        color: ${timeColor};
                        cursor: pointer;
                        font-size: 16px;
                        line-height: 1;
                        padding: 4px;
                        border-radius: 50%;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        opacity: 0.5;
                        transition: opacity 0.2s;
                    " onmouseover="this.style.opacity=1" onmouseout="this.style.opacity=0.5">
                        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 6L6 18M6 6l12 12"/></svg>
                    </button>
                    <div style="
                        width: 48px;
                        height: 48px;
                        background: #e0e7ff;
                        border-radius: 8px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        font-size: 20px;
                        flex-shrink: 0;
                    ">
                        🛍️
                    </div>
                    <div style="flex: 1; min-width: 0;">
                        <p style="margin: 0; font-size: 14px; font-weight: 600; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; line-height: 1.2;">
                            ${location} <span style="font-weight: 400; color: ${secondaryTextColor};">purchased</span>
                        </p>
                        <p style="margin: 4px 0 0 0; font-size: 14px; font-weight: 700; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; color: #4f46e5; line-height: 1.2;">
                            ${product}
                        </p>
                        <div style="display: flex; align-items: center; justify-content: space-between; margin-top: 6px;">
                            <p style="margin: 0; font-size: 12px; font-weight: 500; color: ${timeColor}; line-height: 1;">
                                ${time}
                            </p>
                            <div style="display: flex; align-items: center; gap: 4px; opacity: 0.7;">
                                <span style="font-size: 10px; text-transform: uppercase; font-weight: 700; letter-spacing: 0.05em; color: #22c55e; line-height: 1;">Verified</span>
                                <svg width="12" height="12" style="color: #22c55e;" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path></svg>
                            </div>
                        </div>
                    </div>
                </div>
                ${branding ? `
                <div style="margin-top: 8px; margin-left: 8px;">
                    <a href="https://ohc.app" target="_blank" rel="noopener noreferrer" style="
                        font-size: 10px;
                        font-weight: 700;
                        text-transform: uppercase;
                        letter-spacing: 0.05em;
                        color: ${textColor};
                        opacity: 0.6;
                        text-decoration: none;
                        transition: opacity 0.2s;
                        display: inline-block;
                    " onmouseover="this.style.opacity=1" onmouseout="this.style.opacity=0.6">
                        ⚡ Powered by OHC
                    </a>
                </div>
                ` : ''}
            </div>
        `;

        container.innerHTML = widgetHtml;

        const widget = document.getElementById('ohc-social-proof-widget');
        const closeBtn = document.getElementById('ohc-social-proof-close');

        // Show animation
        setTimeout(() => {
            if (widget) {
                widget.style.opacity = '1';
                widget.style.transform = 'translateY(0)';
            }
        }, 500);

        // Hide animation
        const hideWidget = () => {
            if (widget) {
                widget.style.opacity = '0';
                widget.style.transform = 'translateY(20px)';
                setTimeout(() => {
                    widget.remove();
                }, 500);
            }
        };

        if (closeBtn) {
            closeBtn.addEventListener('click', hideWidget);
        }

        // Auto-hide after 15 seconds
        setTimeout(hideWidget, 15000);
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initSocialProofWidget);
    } else {
        initSocialProofWidget();
    }
})();
