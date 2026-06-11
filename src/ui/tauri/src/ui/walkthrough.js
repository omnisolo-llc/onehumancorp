window.startWalkthrough = function(steps) {
    if (!steps || steps.length === 0) return;

    let currentStep = 0;

    // Create overlay
    let overlay = document.getElementById('ohc-walkthrough-overlay');
    if (!overlay) {
        overlay = document.createElement('div');
        overlay.id = 'ohc-walkthrough-overlay';
        overlay.style.position = 'fixed';
        overlay.style.top = '0';
        overlay.style.left = '0';
        overlay.style.width = '100vw';
        overlay.style.height = '100vh';
        overlay.style.backgroundColor = 'rgba(0, 0, 0, 0.5)';
        overlay.style.zIndex = '9998';
        overlay.style.display = 'none';
        document.body.appendChild(overlay);
    }

    // Create speech bubble
    let bubble = document.getElementById('ohc-walkthrough-bubble');
    if (!bubble) {
        bubble = document.createElement('div');
        bubble.id = 'ohc-walkthrough-bubble';
        bubble.style.position = 'absolute';
        bubble.style.zIndex = '10000';
        bubble.style.backgroundColor = 'white';
        bubble.style.borderRadius = '16px';
        bubble.style.padding = '20px';
        bubble.style.boxShadow = '0 10px 30px rgba(0, 0, 0, 0.2)';
        bubble.style.maxWidth = '300px';
        bubble.style.fontFamily = '"Outfit", -apple-system, sans-serif';
        bubble.style.display = 'none';
        bubble.style.flexDirection = 'column';
        bubble.style.gap = '10px';

        // Add dark mode support logic via CSS injection
        const style = document.createElement('style');
        style.textContent = `
            @media (prefers-color-scheme: dark) {
                #ohc-walkthrough-bubble {
                    background-color: #1c1c1e !important;
                    color: #f5f5f7 !important;
                    border: 1px solid rgba(255,255,255,0.1);
                }
                #ohc-walkthrough-bubble h3 { color: #f5f5f7 !important; }
            }
        `;
        document.head.appendChild(style);

        document.body.appendChild(bubble);
    }

    let originalStyles = new Map();

    function showStep(index) {
        if (index >= steps.length) {
            endWalkthrough();
            return;
        }

        const step = steps[index];
        const target = document.querySelector(step.selector);

        if (!target) {
            console.warn('Walkthrough target not found:', step.selector);
            showStep(index + 1); // Skip if not found
            return;
        }

        // Restore previous target if any
        restoreOriginalStyles();

        // Highlight new target
        originalStyles.set(target, {
            position: target.style.position,
            zIndex: target.style.zIndex,
            backgroundColor: target.style.backgroundColor,
            pointerEvents: target.style.pointerEvents
        });

        const computedStyle = window.getComputedStyle(target);
        if (computedStyle.position === 'static') {
            target.style.position = 'relative';
        }
        target.style.zIndex = '9999';
        if (computedStyle.backgroundColor === 'rgba(0, 0, 0, 0)' && computedStyle.backgroundImage === 'none') {
            target.style.backgroundColor = 'white'; // give it a background so it stands out if transparent

            // support dark mode transparent background fallback
            if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
               target.style.backgroundColor = '#2c2c2e';
            }
        }

        // Scroll to target
        target.scrollIntoView({ behavior: 'smooth', block: 'center' });

        // Update bubble content
        bubble.innerHTML = `
            <h3 style="margin: 0; font-size: 18px; color: #1d1d1f;">${step.title}</h3>
            <p style="margin: 0; font-size: 14px; line-height: 1.4;">${step.text}</p>
            <div style="display: flex; justify-content: space-between; margin-top: 10px;">
                <button id="ohc-wt-skip" style="background: none; border: none; color: #86868b; cursor: pointer; padding: 5px 0; font-family: Outfit;">Skip</button>
                <button id="ohc-wt-next" style="background: #0066FF; color: white; border: none; padding: 8px 16px; border-radius: 8px; cursor: pointer; font-family: Outfit; font-weight: 600;">${index === steps.length - 1 ? 'Finish' : 'Next'}</button>
            </div>
        `;

        document.getElementById('ohc-wt-skip').addEventListener('click', endWalkthrough);
        document.getElementById('ohc-wt-next').addEventListener('click', () => {
            currentStep++;
            showStep(currentStep);
        });

        // Position bubble
        const rect = target.getBoundingClientRect();

        let top = rect.bottom + window.scrollY + 15;
        let left = rect.left + window.scrollX;

        // Ensure bubble is on screen
        if (left + 300 > window.innerWidth) {
            left = window.innerWidth - 320;
        }
        if (left < 10) left = 10;

        // If it goes below the screen, put it above
        if (top + 150 > window.scrollY + window.innerHeight) {
            top = rect.top + window.scrollY - 160;
        }

        bubble.style.top = `${top}px`;
        bubble.style.left = `${left}px`;

        overlay.style.display = 'block';
        bubble.style.display = 'flex';
    }

    function restoreOriginalStyles() {
        originalStyles.forEach((styles, element) => {
            element.style.position = styles.position;
            element.style.zIndex = styles.zIndex;
            element.style.backgroundColor = styles.backgroundColor;
            element.style.pointerEvents = styles.pointerEvents;
        });
        originalStyles.clear();
    }

    function endWalkthrough() {
        restoreOriginalStyles();
        overlay.style.display = 'none';
        bubble.style.display = 'none';
    }

    // Start
    showStep(0);
};
