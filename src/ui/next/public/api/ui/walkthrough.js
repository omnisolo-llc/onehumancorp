function initWalkthrough() {
    window.startWalkthrough = function(steps) {
        if (!steps || steps.length === 0) return;

        let currentStep = 0;

        // Create overlay
        const overlay = document.createElement('div');
        overlay.id = 'walkthrough-overlay';
        overlay.className = 'fixed z-[90]';
        overlay.style.cssText = 'position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); z-index: 99998;';
        document.body.appendChild(overlay);

        // Create bubble
        const bubble = document.createElement('div');
        bubble.id = 'walkthrough-bubble';
        bubble.setAttribute('role', 'dialog');
        bubble.style.cssText = 'position: fixed; background: white; border-radius: 8px; padding: 16px; box-shadow: 0 4px 12px rgba(0,0,0,0.15); z-index: 99999; max-width: 300px; display: flex; flex-direction: column; gap: 8px; font-family: Outfit, sans-serif;';
        document.body.appendChild(bubble);

        function renderStep() {
            const step = steps[currentStep];

            // Clean up previous highlights
            document.querySelectorAll('.walkthrough-highlight').forEach(el => {
                el.classList.remove('walkthrough-highlight');
                el.style.position = '';
                el.style.zIndex = '';
                el.style.background = '';
            });

            // Find target
            const target = document.getElementById(step.targetId) || document.querySelector(step.targetId);

            bubble.innerHTML = `
                <div style="display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid #eee; padding-bottom: 8px; margin-bottom: 8px;">
                    <h4 style="margin: 0; font-size: 16px; font-weight: bold;">${step.title || 'Tour'}</h4>
                    <button id="wt-close" style="background: none; border: none; cursor: pointer; font-size: 18px;">&times;</button>
                </div>
                <p style="margin: 0; font-size: 14px; color: #333;">${step.content}</p>
                <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 8px;">
                    ${currentStep > 0 ? '<button id="wt-prev" style="padding: 6px 12px; border: 1px solid #ccc; border-radius: 4px; background: white; cursor: pointer;">Back</button>' : ''}
                    <button id="wt-next" style="padding: 6px 12px; border: none; border-radius: 4px; background: #0066FF; color: white; cursor: pointer;">${currentStep === steps.length - 1 ? 'Finish' : 'Next'}</button>
                </div>
            `;

            document.getElementById('wt-close').onclick = closeWalkthrough;
            if (document.getElementById('wt-prev')) document.getElementById('wt-prev').onclick = () => { currentStep--; renderStep(); };
            document.getElementById('wt-next').onclick = () => {
                if (currentStep === steps.length - 1) {
                    closeWalkthrough();
                } else {
                    currentStep++;
                    renderStep();
                }
            };

            if (target) {
                target.classList.add('walkthrough-highlight');
                target.style.position = 'relative';
                target.style.zIndex = '99999';
                target.style.background = 'white'; // Make it pop

                const rect = target.getBoundingClientRect();
                // Position bubble below target if space permits, else above
                if (rect.bottom + 200 < window.innerHeight) {
                    bubble.style.top = (rect.bottom + 10) + 'px';
                } else {
                    bubble.style.top = (rect.top - bubble.offsetHeight - 10) + 'px';
                }
                bubble.style.left = Math.max(10, Math.min(rect.left, window.innerWidth - 320)) + 'px';
            } else {
                // Center if no target
                bubble.style.top = '50%';
                bubble.style.left = '50%';
                bubble.style.transform = 'translate(-50%, -50%)';
            }
        }

        function closeWalkthrough() {
            document.querySelectorAll('.walkthrough-highlight').forEach(el => {
                el.classList.remove('walkthrough-highlight');
                el.style.position = '';
                el.style.zIndex = '';
                el.style.background = '';
            });
            if (overlay.parentNode) overlay.parentNode.removeChild(overlay);
            if (bubble.parentNode) bubble.parentNode.removeChild(bubble);
        }

        renderStep();
    };
}
