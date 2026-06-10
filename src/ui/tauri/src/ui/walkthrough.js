// Walkthrough functionality

(function() {
    // Inject walkthrough CSS
    const style = document.createElement('style');
    style.textContent = `
        #ohc-walkthrough-overlay {
            position: fixed;
            top: 0;
            left: 0;
            width: 100vw;
            height: 100vh;
            pointer-events: none;
            z-index: 10000;
        }

        .ohc-walkthrough-highlight {
            position: absolute;
            box-shadow: 0 0 0 9999px rgba(0, 0, 0, 0.5);
            border-radius: 8px;
            pointer-events: none;
            transition: all 0.3s ease;
            z-index: 10001;
        }

        #ohc-walkthrough-bubble {
            position: absolute;
            background: rgba(255, 255, 255, 0.85);
            backdrop-filter: blur(30px) saturate(210%);
            -webkit-backdrop-filter: blur(30px) saturate(210%);
            border: 1px solid rgba(255, 255, 255, 0.6);
            border-radius: 16px;
            padding: 20px;
            box-shadow: 0 10px 40px rgba(0, 0, 0, 0.15);
            width: 300px;
            max-width: 90vw;
            font-family: "Outfit", -apple-system, BlinkMacSystemFont, sans-serif;
            pointer-events: auto;
            z-index: 10002;
            transition: top 0.3s ease, left 0.3s ease;
        }

        .ohc-walkthrough-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 10px;
        }

        .ohc-walkthrough-title {
            margin: 0;
            font-size: 16px;
            font-weight: 600;
            color: #1d1d1f;
        }

        .ohc-walkthrough-close {
            background: none;
            border: none;
            color: #86868b;
            cursor: pointer;
            padding: 4px;
            display: flex;
            align-items: center;
            justify-content: center;
        }

        .ohc-walkthrough-close:hover {
            color: #1d1d1f;
        }

        .ohc-walkthrough-content {
            font-size: 14px;
            color: #48484a;
            line-height: 1.5;
            margin-bottom: 20px;
        }

        .ohc-walkthrough-footer {
            display: flex;
            justify-content: flex-end;
            gap: 10px;
        }

        .ohc-walkthrough-btn {
            background: #0066FF;
            color: white;
            border: none;
            padding: 8px 16px;
            border-radius: 8px;
            font-size: 14px;
            font-weight: 500;
            cursor: pointer;
        }

        .ohc-walkthrough-btn:hover {
            background: #005ce6;
        }

        .ohc-walkthrough-btn-secondary {
            background: rgba(0, 0, 0, 0.05);
            color: #1d1d1f;
            border: none;
            padding: 8px 16px;
            border-radius: 8px;
            font-size: 14px;
            font-weight: 500;
            cursor: pointer;
        }

        .ohc-walkthrough-btn-secondary:hover {
            background: rgba(0, 0, 0, 0.1);
        }

        @media (prefers-color-scheme: dark) {
            #ohc-walkthrough-bubble {
                background: rgba(22, 22, 26, 0.85);
                border-color: rgba(255, 255, 255, 0.1);
            }
            .ohc-walkthrough-title { color: #f5f5f7; }
            .ohc-walkthrough-content { color: #d1d1d6; }
            .ohc-walkthrough-btn-secondary {
                background: rgba(255, 255, 255, 0.1);
                color: #f5f5f7;
            }
            .ohc-walkthrough-btn-secondary:hover {
                background: rgba(255, 255, 255, 0.15);
            }
        }
    `;
    document.head.appendChild(style);

    let currentSteps = [];
    let currentStepIndex = 0;
    let overlayEl = null;
    let highlightEl = null;
    let bubbleEl = null;

    window.startWalkthrough = function(steps) {
        if (!steps || steps.length === 0) return;
        currentSteps = steps;
        currentStepIndex = 0;

        if (!overlayEl) {
            overlayEl = document.createElement('div');
            overlayEl.id = 'ohc-walkthrough-overlay';

            highlightEl = document.createElement('div');
            highlightEl.className = 'ohc-walkthrough-highlight';
            overlayEl.appendChild(highlightEl);

            bubbleEl = document.createElement('div');
            bubbleEl.id = 'ohc-walkthrough-bubble';
            bubbleEl.setAttribute('role', 'dialog');
            overlayEl.appendChild(bubbleEl);

            document.body.appendChild(overlayEl);
        }

        overlayEl.style.display = 'block';
        renderStep();
    };

    function renderStep() {
        const step = currentSteps[currentStepIndex];
        const targetEl = document.getElementById(step.targetId);

        if (!targetEl) {
            console.warn('Walkthrough target not found:', step.targetId);
            // Auto advance or close if last step
            if (currentStepIndex < currentSteps.length - 1) {
                currentStepIndex++;
                renderStep();
            } else {
                closeWalkthrough();
            }
            return;
        }

        // Scroll to target if needed
        targetEl.scrollIntoView({ behavior: 'smooth', block: 'center' });

        // Update bubble content
        const isLastStep = currentStepIndex === currentSteps.length - 1;
        bubbleEl.innerHTML = `
            <div class="ohc-walkthrough-header">
                <h3 class="ohc-walkthrough-title">${step.title}</h3>
                <button class="ohc-walkthrough-close" aria-label="Close walkthrough">
                    <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path></svg>
                </button>
            </div>
            <div class="ohc-walkthrough-content">${step.content}</div>
            <div class="ohc-walkthrough-footer">
                <button class="ohc-walkthrough-btn-secondary" id="ohc-walkthrough-skip">Skip</button>
                <button class="ohc-walkthrough-btn" id="ohc-walkthrough-next">${isLastStep ? 'Finish' : 'Next'}</button>
            </div>
        `;

        // Position highlight and bubble (wait for layout)
        setTimeout(() => {
            const rect = targetEl.getBoundingClientRect();
            const padding = 10;

            highlightEl.style.top = (rect.top - padding) + 'px';
            highlightEl.style.left = (rect.left - padding) + 'px';
            highlightEl.style.width = (rect.width + padding * 2) + 'px';
            highlightEl.style.height = (rect.height + padding * 2) + 'px';

            // Position bubble below or above the target
            const bubbleRect = bubbleEl.getBoundingClientRect();
            let bubbleTop = rect.bottom + padding + 10;
            let bubbleLeft = rect.left;

            if (bubbleTop + bubbleRect.height > window.innerHeight) {
                bubbleTop = rect.top - padding - 10 - bubbleRect.height; // Place above
            }
            if (bubbleLeft + bubbleRect.width > window.innerWidth) {
                bubbleLeft = window.innerWidth - bubbleRect.width - 20; // Prevent overflow right
            }

            bubbleEl.style.top = bubbleTop + 'px';
            bubbleEl.style.left = Math.max(20, bubbleLeft) + 'px';
        }, 300); // 300ms for smooth scroll to finish

        // Attach event listeners
        document.querySelector('.ohc-walkthrough-close').addEventListener('click', closeWalkthrough);
        document.getElementById('ohc-walkthrough-skip').addEventListener('click', closeWalkthrough);
        document.getElementById('ohc-walkthrough-next').addEventListener('click', () => {
            if (isLastStep) {
                closeWalkthrough();
            } else {
                currentStepIndex++;
                renderStep();
            }
        });
    }

    function closeWalkthrough() {
        if (overlayEl) {
            overlayEl.style.display = 'none';
        }
    }
})();
