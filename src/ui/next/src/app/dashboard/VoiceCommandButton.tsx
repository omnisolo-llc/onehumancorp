"use client";

import { useState, useRef } from "react";

export function VoiceCommandButton() {
  const [isListening, setIsListening] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  const mediaRecorder = useRef<MediaRecorder | null>(null);
  const audioChunks = useRef<Blob[]>([]);

  const handlePointerDown = async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      mediaRecorder.current = new MediaRecorder(stream);
      audioChunks.current = [];

      mediaRecorder.current.ondataavailable = (event) => {
        if (event.data.size > 0) {
          audioChunks.current.push(event.data);
        }
      };

      mediaRecorder.current.onstop = async () => {
        setIsProcessing(true);
        const audioBlob = new Blob(audioChunks.current, { type: 'audio/webm' });
        const reader = new FileReader();
        reader.readAsDataURL(audioBlob);
        reader.onloadend = async () => {
          const base64Audio = reader.result?.toString().split(',')[1];
          if (base64Audio) {
            try {
              const res = await fetch("/api/v1/voice/command", {
                method: "POST",
                headers: {
                  "Content-Type": "application/json",
                  Authorization: `Bearer ${localStorage.getItem("token") || ""}`,
                },
                body: JSON.stringify({ audio_base64: base64Audio }),
              });
              if (res.ok) {
                // Let the agent feed polling pick up the new proposal
                console.log("Voice command submitted successfully");
              }
            } catch (e) {
              console.error("Voice command failed", e);
            }
          }
          setIsProcessing(false);
        };
      };

      mediaRecorder.current.start();
      setIsListening(true);
    } catch (err) {
      console.error("Microphone access denied", err);
    }
  };

  const handlePointerUp = () => {
    if (mediaRecorder.current && mediaRecorder.current.state === "recording") {
      mediaRecorder.current.stop();
      mediaRecorder.current.stream.getTracks().forEach((track) => track.stop());
    }
    setIsListening(false);
  };

  return (
    <div className="fixed bottom-6 left-1/2 -translate-x-1/2 z-50">
      <button
        onPointerDown={handlePointerDown}
        onPointerUp={handlePointerUp}
        onPointerLeave={handlePointerUp}
        className={`w-16 h-16 rounded-full glassmorphism flex items-center justify-center shadow-lg transition-all ${
          isListening ? "bg-red-500 scale-110" : "bg-white/80 hover:bg-white"
        } ${isProcessing ? "animate-pulse" : ""}`}
        aria-label="Hold to speak"
        data-testid="voice-command-button"
      >
        <span className="text-2xl">{isListening ? "🎙️" : "🎤"}</span>
      </button>
      {isListening && (
        <div className="absolute -top-10 left-1/2 -translate-x-1/2 bg-black/70 text-white text-xs px-3 py-1 rounded-full whitespace-nowrap">
          Listening...
        </div>
      )}
      {isProcessing && (
        <div className="absolute -top-10 left-1/2 -translate-x-1/2 bg-black/70 text-white text-xs px-3 py-1 rounded-full whitespace-nowrap">
          Processing...
        </div>
      )}
    </div>
  );
}
