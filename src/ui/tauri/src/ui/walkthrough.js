(function() {
    // Inject walkthrough CSS
    const style = document.createElement('style');
    style.textContent = `
        .ohc-walkthrough-overlay {
            position: fixed;
            top: 0; left: 0; width: 100vw; height: 100vh;
            background: rgba(0, 0, 0, 0.4);
            backdrop-filter: blur(2px);
            z-index: 10000;
            pointer-events: auto;
        }
        .ohc-walkthrough-highlight {
            position: absolute;
            box-shadow: 0 0 0 9999px rgba(0, 0, 0, 0.6);
            border-radius: 8px;
            pointer-events: none;
            z-index: 10001;
            transition: all 0.3s ease;
        }
        .ohc-walkthrough-bubble {
            position: absolute;
            background: rgba(255, 255, 255, 0.95);
            backdrop-filter: blur(20px) saturate(210%);
            border: 1px solid rgba(0, 102, 255, 0.3);
            border-radius: 16px;
            padding: 20px;
            width: 280px;
            box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
            z-index: 10002;
            font-family: "Outfit", sans-serif;
            color: #1d1d1f;
            transition: all 0.3s ease;
            opacity: 0;
            transform: translateY(10px);
        }
        .ohc-walkthrough-bubble.visible {
            opacity: 1;
            transform: translateY(0);
        }
        .ohc-walkthrough-title {
            font-size: 16px;
            font-weight: 700;
            margin: 0 0 8px 0;
        }
        .ohc-walkthrough-text {
            font-size: 14px;
            color: #86868b;
            margin: 0 0 16px 0;
            line-height: 1.4;
        }
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
            background: rgba(0, 0, 0, 0.05);
            color: #1d1d1f;
            border: none;
            padding: 8px 16px;
            border-radius: 8px;
            font-size: 14px;
            font-weight: 600;
            cursor: pointer;
        }
        .ohc-walkthrough-close {
            position: absolute;
            top: 12px; right: 12px;
            background: none; border: none; color: #86868b; cursor: pointer;
        }
        @media (prefers-color-scheme: dark) {
            .ohc-walkthrough-bubble {
                background: rgba(22, 22, 26, 0.95);
                border-color: rgba(255, 255, 255, 0.1);
                color: #f5f5f7;
            }
            .ohc-walkthrough-btn-secondary {
                background: rgba(255, 255, 255, 0.1);
                color: #f5f5f7;
            }
        }
    `;
    document.head.appendChild(style);

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
            // Skip or end? Let's just end for now if target is missing
            endWalkthrough();
            return;
        }

        targetEl.scrollIntoView({ behavior: 'smooth', block: 'center' });

        setTimeout(() => {
            const rect = targetEl.getBoundingClientRect();

            highlightEl.style.top = (rect.top - 5) + 'px';
            highlightEl.style.left = (rect.left - 5) + 'px';
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

            // Position bubble
            let bubbleTop = rect.bottom + 15;
            let bubbleLeft = rect.left + (rect.width / 2) - 140; // center

            // Adjust bounds
            if (bubbleLeft < 10) bubbleLeft = 10;
            if (bubbleLeft + 280 > window.innerWidth - 10) bubbleLeft = window.innerWidth - 290;
            if (bubbleTop + 200 > window.innerHeight) {
                bubbleTop = rect.top - 180; // place above
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

        }, 300); // Wait for scroll
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

        endWalkthrough(); // cleanup existing

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

})();
