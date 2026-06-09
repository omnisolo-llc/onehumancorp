document.addEventListener('DOMContentLoaded', () => {
    // Inject tooltip CSS
    const style = document.createElement('style');
    style.textContent = `
        .ohc-tooltip {
            position: fixed;
            background: rgba(255, 255, 255, 0.85);
            backdrop-filter: blur(20px) saturate(210%);
            -webkit-backdrop-filter: blur(20px) saturate(210%);
            border: 1px solid rgba(255, 255, 255, 0.6);
            border-radius: 12px;
            padding: 12px 16px;
            color: #1d1d1f;
            font-family: "Outfit", -apple-system, BlinkMacSystemFont, sans-serif;
            font-size: 14px;
            line-height: 1.4;
            max-width: 250px;
            box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1);
            z-index: 10000;
            pointer-events: none;
            opacity: 0;
            transform: translateY(10px);
            transition: opacity 0.2s ease, transform 0.2s ease;
            text-align: center;
        }
        .ohc-tooltip.visible {
            opacity: 1;
            transform: translateY(0);
        }
        @media (prefers-color-scheme: dark) {
            .ohc-tooltip {
                background: rgba(22, 22, 26, 0.85);
                border-color: rgba(255, 255, 255, 0.1);
                color: #f5f5f7;
            }
        }
    `;
    document.head.appendChild(style);

    const tooltipEl = document.createElement('div');
    tooltipEl.className = 'ohc-tooltip';
    document.body.appendChild(tooltipEl);

    let hideTimeout;

    function showTooltip(e, text) {
        clearTimeout(hideTimeout);
        tooltipEl.textContent = text;

        const targetRect = e.target.getBoundingClientRect();

        // Position below the element by default
        let top = targetRect.bottom + 10;
        let left = targetRect.left + (targetRect.width / 2) - (tooltipEl.offsetWidth / 2);

        // Adjust if it goes off screen
        if (left < 10) left = 10;
        if (left + tooltipEl.offsetWidth > window.innerWidth - 10) {
            left = window.innerWidth - tooltipEl.offsetWidth - 10;
        }

        if (top + tooltipEl.offsetHeight > window.innerHeight - 10) {
            // Position above if it doesn't fit below
            top = targetRect.top - tooltipEl.offsetHeight - 10;
        }

        tooltipEl.style.top = `${top}px`;
        tooltipEl.style.left = `${left}px`;
        tooltipEl.classList.add('visible');
    }

    function hideTooltip() {
        hideTimeout = setTimeout(() => {
            tooltipEl.classList.remove('visible');
        }, 100); // slight delay to prevent flicker
    }

    // Desktop hover
    document.addEventListener('mouseover', (e) => {
        const target = e.target.closest('[data-tooltip]');
        if (target) {
            showTooltip(e, target.getAttribute('data-tooltip'));
        }
    });

    document.addEventListener('mouseout', (e) => {
        if (e.target.closest('[data-tooltip]')) {
            hideTooltip();
        }
    });

    // Mobile long press (contextmenu) or touchstart
    let touchTimeout;
    document.addEventListener('touchstart', (e) => {
        const target = e.target.closest('[data-tooltip]');
        if (target) {
            touchTimeout = setTimeout(() => {
                showTooltip(e, target.getAttribute('data-tooltip'));
            }, 500); // 500ms long press
        }
    });

    document.addEventListener('touchend', () => {
        clearTimeout(touchTimeout);
        hideTooltip();
    });

    document.addEventListener('touchmove', () => {
        clearTimeout(touchTimeout);
        hideTooltip();
    });
});
