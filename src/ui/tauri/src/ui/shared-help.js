document.addEventListener('DOMContentLoaded', () => {
    // Inject tooltip CSS
    const tooltipStyle = document.createElement('style');
    tooltipStyle.textContent = `
        .ohc-tooltip {
            position: fixed;
            background: rgba(255, 255, 255, 0.85);
            backdrop-filter: blur(20px) saturate(210%);
            -webkit-backdrop-filter: blur(20px) saturate(210%);
            border: 1px solid rgba(255, 255, 255, 0.6);
            border-radius: 16px;
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
                background: rgba(22, 22, 26, 0.7);
                border-color: rgba(255, 255, 255, 0.1);
                color: #f5f5f7;
            }
        }
    `;
    document.head.appendChild(tooltipStyle);

    if (!window.OHC_TOOLTIPS) {
        window.OHC_TOOLTIPS = {};
        fetch("/api/tooltips").then(r => r.json()).then(data => { window.OHC_TOOLTIPS = data; }).catch(console.error);
    }

    const tooltipEl = document.createElement('div');
    tooltipEl.className = 'ohc-tooltip';
    document.body.appendChild(tooltipEl);

    function showTooltip(e, text) {
        if (!text) return;
        tooltipEl.textContent = text;
        const rect = e.target.getBoundingClientRect();

        let top = rect.top - tooltipEl.offsetHeight - 10;
        let left = rect.left + (rect.width / 2) - (tooltipEl.offsetWidth / 2);

        if (top < 10) {
            top = rect.bottom + 10;
        }
        if (left < 10) left = 10;
        if (left + tooltipEl.offsetWidth > window.innerWidth - 10) {
            left = window.innerWidth - tooltipEl.offsetWidth - 10;
        }

        tooltipEl.style.top = top + 'px';
        tooltipEl.style.left = left + 'px';
        tooltipEl.classList.add('visible');
    }

    function hideTooltip() {
        tooltipEl.classList.remove('visible');
    }

    // Hover (desktop)
    document.addEventListener('mouseover', (e) => {
        const t = e.target;
        const target = t && typeof t.closest === 'function' ? t.closest('[data-tooltip]') : null;
        if (target) {
            showTooltip(e, (window.OHC_TOOLTIPS && window.OHC_TOOLTIPS[target.id]) || target.getAttribute('data-tooltip'));
        }
    });

    document.addEventListener('mouseout', (e) => {
        const target = e.target;
        if (target && typeof target.closest === 'function' && target.closest('[data-tooltip]')) {
            hideTooltip();
        }
    });

    // Mobile long press (contextmenu) or touchstart
    let touchTimeout;
    let hideTimeout;

    document.addEventListener('touchstart', (e) => {
        const t = e.target;
        const target = t && typeof t.closest === 'function' ? t.closest('[data-tooltip]') : null;
        if (target) {
            clearTimeout(hideTimeout);
            touchTimeout = setTimeout(() => {
                showTooltip(e, (window.OHC_TOOLTIPS && window.OHC_TOOLTIPS[target.id]) || target.getAttribute('data-tooltip'));
            }, 500); // 500ms long press
        } else {
            // Tap anywhere else hides tooltip immediately
            hideTooltip();
        }
    });

    document.addEventListener('touchend', () => {
        clearTimeout(touchTimeout);
        // Do not hide immediately on mobile so user can read it.
        hideTimeout = setTimeout(() => {
            hideTooltip();
        }, 2500); // Hide after 2.5 seconds
    });

    document.addEventListener('touchmove', () => {
        clearTimeout(touchTimeout);
        // hideTooltip(); // We might not want to hide immediately on move if they are just scrolling slightly, but let's clear the show timeout.
    });

    // --- WALKTHROUGH LOGIC ---
    const walkthroughStyle = document.createElement('style');
    walkthroughStyle.textContent = `
        .ohc-walkthrough-overlay {
            position: fixed;
            top: 0; left: 0; right: 0; bottom: 0;
            background: rgba(0, 0, 0, 0.4);
            backdrop-filter: blur(4px);
            z-index: 9998;
            animation: fadeIn 0.3s ease;
        }
        .ohc-walkthrough-highlight {
            position: absolute;
            background: transparent;
            box-shadow: 0 0 0 9999px rgba(0, 0, 0, 0.5), 0 0 20px rgba(0, 102, 255, 0.5);
            border: 2px solid #0066FF;
            border-radius: 12px;
            z-index: 9999;
            pointer-events: none;
            transition: all 0.3s ease;
        }
        .ohc-walkthrough-bubble {
            position: absolute;
            background: rgba(255, 255, 255, 0.9);
            backdrop-filter: blur(20px) saturate(210%);
            border: 1px solid rgba(255, 255, 255, 0.6);
            border-radius: 16px;
            padding: 20px;
            width: 280px;
            z-index: 10000;
            box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
            font-family: "Outfit", -apple-system, sans-serif;
            opacity: 0;
            transform: translateY(10px);
            transition: opacity 0.3s ease, transform 0.3s ease, left 0.3s ease, top 0.3s ease;
        }
        .ohc-walkthrough-bubble.visible {
            opacity: 1;
            transform: translateY(0);
        }
        @media (prefers-color-scheme: dark) {
            .ohc-walkthrough-bubble {
                background: rgba(30, 30, 30, 0.9);
                border-color: rgba(255, 255, 255, 0.1);
                color: #f5f5f7;
            }
        }
        .ohc-walkthrough-title {
            font-size: 16px;
            font-weight: 700;
            margin: 0 0 8px 0;
            color: #1d1d1f;
        }
        @media (prefers-color-scheme: dark) { .ohc-walkthrough-title { color: #fff; } }
        .ohc-walkthrough-text {
            font-size: 14px;
            color: #6b7280;
            margin: 0 0 16px 0;
            line-height: 1.4;
        }
        @media (prefers-color-scheme: dark) { .ohc-walkthrough-text { color: #a1a1aa; } }
        .ohc-walkthrough-controls {
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        .ohc-walkthrough-btn {
            background: #0066FF;
            color: white;
            border: none;
            padding: 8px 16px;
            border-radius: 8px;
            font-size: 14px;
            font-weight: 600;
            cursor: pointer;
        }
        .ohc-walkthrough-btn-secondary {
            background: transparent;
            color: #6b7280;
            border: none;
            padding: 8px;
            font-size: 14px;
            cursor: pointer;
        }
        .ohc-walkthrough-close {
            position: absolute;
            top: 12px; right: 12px;
            background: transparent;
            border: none;
            color: #86868b;
            cursor: pointer;
            padding: 4px;
        }
    `;
    document.head.appendChild(walkthroughStyle);

    let currentSteps = [];
    let currentStepIndex = 0;
    let overlayEl = null;
    let highlightEl = null;
    let bubbleEl = null;

    function renderStep() {
        if (!currentSteps || currentStepIndex >= currentSteps.length) {
            endWalkthrough();
            return;
        }

        const step = currentSteps[currentStepIndex];
        const targetEl = document.querySelector(step.selector);

        if (!targetEl) {
            console.warn('Walkthrough target not found:', step.selector);
            endWalkthrough();
            return;
        }

        targetEl.scrollIntoView({ behavior: 'smooth', block: 'center' });

        setTimeout(() => {
            const rect = targetEl.getBoundingClientRect();

            highlightEl.style.top = (rect.top - 5 + window.scrollY) + 'px';
            highlightEl.style.left = (rect.left - 5 + window.scrollX) + 'px';
            highlightEl.style.width = (rect.width + 10) + 'px';
            highlightEl.style.height = (rect.height + 10) + 'px';

            bubbleEl.innerHTML = `
                <button class="ohc-walkthrough-close" aria-label="Close walkthrough">
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M18 6L6 18M6 6l12 12"></path></svg>
                </button>
                <h4 class="ohc-walkthrough-title">${step.title}</h4>
                <p class="ohc-walkthrough-text">${step.text}</p>
                <div class="ohc-walkthrough-controls">
                    ${currentStepIndex > 0 ? '<button class="ohc-walkthrough-btn-secondary prev-btn">Back</button>' : '<div></div>'}
                    <button class="ohc-walkthrough-btn next-btn">${currentStepIndex === currentSteps.length - 1 ? 'Finish' : 'Next'}</button>
                </div>
            `;

            let bubbleTop = rect.bottom + 15 + window.scrollY;
            let bubbleLeft = rect.left + (rect.width / 2) - 140 + window.scrollX;

            if (bubbleLeft < 10) bubbleLeft = 10;
            if (bubbleLeft + 280 > window.innerWidth - 10) bubbleLeft = window.innerWidth - 290;
            if (bubbleTop + 200 > window.innerHeight + window.scrollY) {
                bubbleTop = rect.top - 180 + window.scrollY;
            }

            bubbleEl.style.top = bubbleTop + 'px';
            bubbleEl.style.left = bubbleLeft + 'px';
            bubbleEl.classList.add('visible');

            bubbleEl.querySelector('.ohc-walkthrough-close').onclick = endWalkthrough;
            bubbleEl.querySelector('.next-btn').onclick = () => {
                bubbleEl.classList.remove('visible');
                setTimeout(() => {
                    currentStepIndex++;
                    renderStep();
                }, 300);
            };
            const prevBtn = bubbleEl.querySelector('.prev-btn');
            if (prevBtn) {
                prevBtn.onclick = () => {
                    bubbleEl.classList.remove('visible');
                    setTimeout(() => {
                        currentStepIndex--;
                        renderStep();
                    }, 300);
                };
            }

        }, 300);
    }

    function endWalkthrough() {
        if (overlayEl) {
            overlayEl.remove();
            overlayEl = null;
        }
        if (highlightEl) {
            highlightEl.remove();
            highlightEl = null;
        }
        if (bubbleEl) {
            bubbleEl.remove();
            bubbleEl = null;
        }
        currentSteps = [];
        currentStepIndex = 0;
    }

    window.startWalkthrough = function(steps) {
        if (!steps || steps.length === 0) return;
        currentSteps = steps;
        currentStepIndex = 0;

        endWalkthrough();

        overlayEl = document.createElement('div');
        overlayEl.className = 'ohc-walkthrough-overlay';

        highlightEl = document.createElement('div');
        highlightEl.className = 'ohc-walkthrough-highlight';

        bubbleEl = document.createElement('div');
        bubbleEl.className = 'ohc-walkthrough-bubble';

        document.body.appendChild(overlayEl);
        document.body.appendChild(highlightEl);
        document.body.appendChild(bubbleEl);

        renderStep();
    };
});
