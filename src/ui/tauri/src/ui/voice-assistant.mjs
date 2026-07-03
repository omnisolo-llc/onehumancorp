document.addEventListener('DOMContentLoaded', () => {
    // 1. Inject Styles
    const style = document.createElement('style');
    style.textContent = \`
        .voice-recording { animation: pulse-red 1.5s infinite; background: #ef4444 !important; border-color: #f87171 !important; transform: scale(1.1); }
        @keyframes pulse-red { 0% { box-shadow: 0 0 0 0 rgba(239, 68, 68, 0.7); } 70% { box-shadow: 0 0 0 20px rgba(239, 68, 68, 0); } 100% { box-shadow: 0 0 0 0 rgba(239, 68, 68, 0); } }
    \`;
    document.head.appendChild(style);

    // 2. Inject UI
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

            btn.classList.add("voice-recording");
            voiceIcon.style.color = "white";
            statusDiv.style.display = "block";
            statusDiv.innerHTML = '<div style="display: flex; align-items: center; justify-content: center; gap: 8px;"><div style="width: 12px; height: 12px; border-radius: 50%; background: #ef4444; animation: pulse-red 1.5s infinite;"></div>Listening...</div>';

        } catch (err) {
            console.error("Failed to start recording:", err);
            isInitializing = false;
            statusDiv.style.display = "block";
            statusDiv.textContent = "Error accessing microphone. Check permissions.";
            setTimeout(() => { statusDiv.style.display = "none"; }, 3000);
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
            btn.classList.remove("voice-recording");
            voiceIcon.style.color = "#0066FF";
            statusDiv.innerHTML = '<div style="display: flex; align-items: center; justify-content: center; gap: 8px;"><div style="width: 12px; height: 12px; border-radius: 50%; background: #3b82f6; animation: pulse-red 1.5s infinite;"></div>Thinking...</div>';
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

            statusDiv.innerHTML = '<div style="display: flex; align-items: center; justify-content: center; gap: 8px;"><div style="width: 12px; height: 12px; border-radius: 50%; background: #22c55e;"></div>Action Prepared!</div><p id="voice-transcription-text" style="margin-top: 8px; font-size: 12px; color: #6b7280; font-style: italic;"></p>';
            document.getElementById('voice-transcription-text').textContent = \`"\${data.transcription}"\`;

            setTimeout(() => {
                statusDiv.style.display = "none";
            }, 5000);
        } catch (err) {
            console.error("Error sending voice command:", err);
            statusDiv.textContent = "Error. Try again.";
            setTimeout(() => { statusDiv.style.display = "none"; }, 3000);
        }
    }

    btn.addEventListener("mousedown", startVoiceRecording);
    btn.addEventListener("mouseup", stopVoiceRecording);
    btn.addEventListener("mouseleave", stopVoiceRecording);
    btn.addEventListener("touchstart", startVoiceRecording, {passive: false});
    btn.addEventListener("touchend", stopVoiceRecording, {passive: false});
});
