document.addEventListener('DOMContentLoaded', () => {
    // 1. Inject Styles
    const style = document.createElement('style');
    style.textContent = \`
        .voice-recording { animation: pulse-red 1.5s infinite; background: #ef4444 !important; border-color: #f87171 !important; transform: scale(1.1); }
        @keyframes pulse-red { 0% { box-shadow: 0 0 0 0 rgba(239, 68, 68, 0.7); } 70% { box-shadow: 0 0 0 20px rgba(239, 68, 68, 0); } 100% { box-shadow: 0 0 0 0 rgba(239, 68, 68, 0); } }
    \`;
    document.head.appendChild(style);

    // 2. Inject UI

    // Create the full-screen overlay (Walk-up screen)
    const overlay = document.createElement('div');
    overlay.id = 'voice-overlay';
    overlay.className = 'glassmorphism';
    overlay.style.cssText = 'position: fixed; top: 0; left: 0; width: 100vw; height: 100vh; z-index: 1001; display: none; flex-direction: column; align-items: center; justify-content: center; background: rgba(255, 255, 255, 0.85); backdrop-filter: blur(40px) saturate(220%); padding: 24px; transition: opacity 0.3s ease; opacity: 0; pointer-events: auto;';

    const overlayContent = document.createElement('div');
    overlayContent.id = 'voice-overlay-content';
    overlayContent.style.cssText = 'display: flex; flex-direction: column; align-items: center; justify-content: center; width: 100%; max-width: 375px; text-align: center; gap: 24px;';

    const overlayStatus = document.createElement('div');
    overlayStatus.id = 'voice-overlay-status';
    overlayStatus.style.cssText = 'font-size: 24px; font-weight: 600; color: #1D1D1F;';
    overlayStatus.innerHTML = 'Listening...';

    const micIndicator = document.createElement('div');
    micIndicator.id = 'voice-overlay-mic';
    micIndicator.className = 'voice-recording';
    micIndicator.style.cssText = 'width: 96px; height: 96px; border-radius: 50%; display: flex; align-items: center; justify-content: center; background: #ef4444; color: white; margin-bottom: 32px; box-shadow: 0 4px 12px rgba(239, 68, 68, 0.3);';
    micIndicator.innerHTML = `<svg style="width: 48px; height: 48px;" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"></path></svg>`;

    const resultCard = document.createElement('div');
    resultCard.id = 'voice-overlay-result';
    resultCard.style.cssText = 'display: none; flex-direction: column; width: 100%; background: #ffffff; border-radius: 16px; padding: 24px; box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08); border: 1px solid rgba(0, 0, 0, 0.05); text-align: left; gap: 16px;';

    const resultTitle = document.createElement('div');
    resultTitle.style.cssText = 'font-size: 16px; font-weight: 500; color: #6b7280; text-transform: uppercase; letter-spacing: 0.05em;';
    resultTitle.textContent = 'Drafted Order';

    const resultText = document.createElement('div');
    resultText.id = 'voice-result-text';
    resultText.style.cssText = 'font-size: 24px; font-weight: 600; color: #1D1D1F;';

    const confirmBtn = document.createElement('button');
    confirmBtn.id = 'voice-confirm-btn';
    confirmBtn.style.cssText = 'width: 100%; padding: 16px; background: #0066FF; color: white; border: none; border-radius: 12px; font-size: 18px; font-weight: 600; cursor: pointer; transition: background 0.2s; margin-top: 8px;';
    confirmBtn.textContent = 'Confirm & Add to List';
    confirmBtn.onclick = () => {
        overlay.style.opacity = '0';
        setTimeout(() => {
            overlay.style.display = 'none';
            // Trigger a refresh or redirect to feed if needed
            if (window.location.pathname !== '/triage' && window.location.pathname !== '/dashboard') {
                window.location.href = '/triage';
            } else {
                window.location.reload();
            }
        }, 300);
    };

    const cancelBtn = document.createElement('button');
    cancelBtn.id = 'voice-cancel-btn';
    cancelBtn.style.cssText = 'width: 100%; padding: 12px; background: transparent; color: #6b7280; border: none; font-size: 16px; font-weight: 500; cursor: pointer; margin-top: -8px;';
    cancelBtn.textContent = 'Cancel';
    cancelBtn.onclick = () => {
        overlay.style.opacity = '0';
        setTimeout(() => overlay.style.display = 'none', 300);
    };

    resultCard.appendChild(resultTitle);
    resultCard.appendChild(resultText);
    resultCard.appendChild(confirmBtn);
    resultCard.appendChild(cancelBtn);

    overlayContent.appendChild(micIndicator);
    overlayContent.appendChild(overlayStatus);
    overlayContent.appendChild(resultCard);
    overlay.appendChild(overlayContent);
    document.body.appendChild(overlay);


    const container = document.createElement('div');
    container.id = 'voice-assistant-container';
    container.className = 'fixed bottom-6 left-1/2 -translate-x-1/2 z-[100] flex flex-col items-center gap-4 w-full max-w-[375px] px-4 pointer-events-none';
    container.style.cssText = 'position: fixed; bottom: 80px; left: 50%; transform: translateX(-50%); z-index: 1000; display: flex; flex-direction: column; align-items: center; pointer-events: none;';

    const statusDiv = document.createElement('div');
    statusDiv.id = 'voice-status';
    statusDiv.style.cssText = 'display: none; width: 100%; padding: 16px; background: rgba(255, 255, 255, 0.85); backdrop-filter: blur(30px); border: 1px solid rgba(255, 255, 255, 0.4); border-radius: 16px; box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1); pointer-events: auto; margin-bottom: 16px; font-weight: 500; font-size: 14px; text-align: center;';

    const btn = document.createElement('button');
    btn.id = 'voice-assistant-tooltip';
    btn.setAttribute('aria-label', 'Voice Assistant');
    btn.className = 'w-16 h-16 rounded-full flex items-center justify-center shadow-2xl transition-all duration-300 pointer-events-auto touch-none glassmorphism border border-white/40';
    btn.style.cssText = 'width: 64px; height: 64px; border-radius: 50%; display: flex; align-items: center; justify-content: center; background: rgba(255, 255, 255, 0.65); backdrop-filter: blur(30px); border: 1px solid rgba(255, 255, 255, 0.4); box-shadow: 0 4px 12px rgba(0,0,0,0.15); cursor: pointer; pointer-events: auto;';

    btn.innerHTML = \`<svg id="voice-mic-icon" style="width: 32px; height: 32px; color: #0066FF;" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"></path></svg>\`;

    container.appendChild(statusDiv);
    container.appendChild(btn);
    document.body.appendChild(container);

    // 3. Logic
    let isRecording = false;
    let isInitializing = false;
    let mediaRecorder = null;
    let audioChunks = [];
    let stoppedEarly = false;
    const voiceIcon = document.getElementById("voice-mic-icon");

    async function startVoiceRecording(e) {
        if (e && e.cancelable) e.preventDefault();
        if (isRecording || isInitializing) return;
        isInitializing = true;
        stoppedEarly = false;

        try {
            const stream = await navigator.mediaDevices.getUserMedia({ audio: true });

            // Check if user let go of the button BEFORE the stream was ready
            if (stoppedEarly) {
                stream.getTracks().forEach(track => track.stop());
                isInitializing = false;
                return;
            }

            mediaRecorder = new MediaRecorder(stream);
            audioChunks = [];
            mediaRecorder.ondataavailable = (event) => {
                if (event.data.size > 0) audioChunks.push(event.data);
            };
            mediaRecorder.onstop = async () => {
                if (audioChunks.length === 0) return;
                const audioBlob = new Blob(audioChunks, { type: "audio/m4a" });
                const reader = new FileReader();
                reader.readAsDataURL(audioBlob);
                reader.onloadend = async () => {
                    const base64Audio = reader.result.split(",")[1];
                    await sendVoiceCommand(base64Audio);
                };
                stream.getTracks().forEach(track => track.stop());
            };

            mediaRecorder.start();
            isRecording = true;
            isInitializing = false;

            // Show full screen overlay
            overlay.style.display = 'flex';
            setTimeout(() => overlay.style.opacity = '1', 10);
            micIndicator.style.display = 'flex';
            overlayStatus.innerHTML = 'Listening...';
            resultCard.style.display = 'none';

        } catch (err) {
            console.error("Failed to start recording:", err);
            isInitializing = false;

            overlay.style.display = 'flex';
            setTimeout(() => overlay.style.opacity = '1', 10);
            micIndicator.style.display = 'none';
            resultCard.style.display = 'none';
            overlayStatus.innerHTML = 'Microphone access denied';

            setTimeout(() => {
                overlay.style.opacity = '0';
                setTimeout(() => overlay.style.display = 'none', 300);
            }, 3000);
        }
    }

    function stopVoiceRecording(e) {
        if (e && e.cancelable) e.preventDefault();

        if (isInitializing) {
            stoppedEarly = true;
            return;
        }

        if (isRecording && mediaRecorder) {
            mediaRecorder.stop();
            isRecording = false;

            micIndicator.classList.remove("voice-recording");
            micIndicator.style.background = '#3b82f6'; // Thinking blue
            overlayStatus.innerHTML = 'Processing command...';
        }
    }

    async function sendVoiceCommand(base64Audio) {
        try {
            const res = await fetch("/api/v1/voice/command", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                    "x-spiffe-id": \`spiffe://ohc/org/\${localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default"}/agent/ui\`
                },
                body: JSON.stringify({ audio_data: base64Audio })
            });
            if (!res.ok) throw new Error("Voice command failed");
            const data = await res.json();

            // Display Result Card
            micIndicator.style.display = 'none';
            overlayStatus.innerHTML = 'Action Prepared!';
            overlayStatus.style.color = '#22c55e'; // Success green

            resultCard.style.display = 'flex';

            // Format the text elegantly
            let text = data.transcription;
            // E.g., if it says "Drafted Order: 3x Chicken Tacos", we strip "Drafted Order: "
            text = text.replace(/^Drafted Order:\s*/i, '');
            document.getElementById('voice-result-text').textContent = text;

            // Add a hidden id or text for testing purposes
            const hiddenStatus = document.createElement('div');
            hiddenStatus.id = 'voice-transcription-text';
            hiddenStatus.style.display = 'none';
            hiddenStatus.textContent = `"${data.transcription}"`;
            resultCard.appendChild(hiddenStatus);

        } catch (err) {
            console.error("Error sending voice command:", err);
            micIndicator.style.display = 'none';
            resultCard.style.display = 'none';
            overlayStatus.innerHTML = 'Error Processing Request';
            overlayStatus.style.color = '#ef4444';

            setTimeout(() => {
                overlay.style.opacity = '0';
                setTimeout(() => overlay.style.display = 'none', 300);
                overlayStatus.style.color = '#1D1D1F';
            }, 3000);
        }
    }

    btn.addEventListener("mousedown", startVoiceRecording);
    btn.addEventListener("mouseup", stopVoiceRecording);
    btn.addEventListener("mouseleave", stopVoiceRecording);
    btn.addEventListener("touchstart", startVoiceRecording, {passive: false});
    btn.addEventListener("touchend", stopVoiceRecording, {passive: false});
});
