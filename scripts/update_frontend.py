with open('src/server/lib.rs', 'r') as f:
    content = f.read()
target = """                        function nextStep(step) {
                            document.getElementById('setup-screen').querySelectorAll('div[id^="step-"]').forEach(d => d.style.display = 'none');
                            const target = document.getElementById('step-' + step);
                            if (target) target.style.display = 'block';
                        }"""
replacement = """                        function nextStep(step) {
                            document.getElementById('setup-screen').querySelectorAll('div[id^="step-"]').forEach(d => d.style.display = 'none');
                            const target = document.getElementById('step-' + step);
                            if (target) target.style.display = 'block';

                            // Cross-device sync via /api/onboarding/state
                            fetch('/api/onboarding/state', {
                                method: 'POST',
                                headers: { 'Content-Type': 'application/json' },
                                body: JSON.stringify({ step: step })
                            }).catch(e => console.error('Failed to save state:', e));
                        }"""
with open('src/server/lib.rs', 'w') as f:
    f.write(content.replace(target, replacement))
