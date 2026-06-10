"use client";

import React, { useState, useRef, useEffect, useCallback } from "react";

export function VoiceAssistant() {
  const [isRecording, setIsRecording] = useState(false);
  const [status, setStatus] = useState<"idle" | "listening" | "processing" | "success" | "error">("idle");
  const [transcription, setTranscription] = useState("");
  const mediaRecorderRef = useRef<MediaRecorder | null>(null);
  const audioChunksRef = useRef<Blob[]>([]);

  const startRecording = useCallback(async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      const mediaRecorder = new MediaRecorder(stream);
      mediaRecorderRef.current = mediaRecorder;
      audioChunksRef.current = [];

      mediaRecorder.ondataavailable = (event) => {
        if (event.data.size > 0) {
          audioChunksRef.current.push(event.data);
        }
      };

      mediaRecorder.onstop = async () => {
        const audioBlob = new Blob(audioChunksRef.current, { type: 'audio/m4a' });
        const reader = new FileReader();
        reader.readAsDataURL(audioBlob);
        reader.onloadend = async () => {
          const base64Audio = (reader.result as string).split(',')[1];
          await sendVoiceCommand(base64Audio);
        };

        // Stop all tracks to release the microphone
        stream.getTracks().forEach(track => track.stop());
      };

      mediaRecorder.start();
      setIsRecording(true);
      setStatus("listening");
      setTranscription("");
    } catch (err) {
      console.error("Failed to start recording:", err);
      setStatus("error");
    }
  }, []);

  const stopRecording = useCallback(() => {
    if (mediaRecorderRef.current && isRecording) {
      mediaRecorderRef.current.stop();
      setIsRecording(false);
      setStatus("processing");
    }
  }, [isRecording]);

  const sendVoiceCommand = async (base64Audio: string) => {
    try {
      const response = await fetch("/api/v1/voice/command", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "Authorization": `Bearer ${localStorage.getItem('token')}`
        },
        body: JSON.stringify({ audio_data: base64Audio }),
      });

      if (!response.ok) throw new Error("Voice command failed");

      const data = await response.json();
      setTranscription(data.transcription);
      setStatus("success");

      setTimeout(() => {
        setStatus("idle");
        setTranscription("");
      }, 5000);
    } catch (err) {
      console.error("Error sending voice command:", err);
      setStatus("error");
      setTimeout(() => setStatus("idle"), 3000);
    }
  };

  return (
    <div className="fixed bottom-6 left-1/2 -translate-x-1/2 z-[100] flex flex-col items-center gap-4 w-full max-w-[375px] px-4 pointer-events-none">
      {status !== "idle" && (
        <div className="w-full p-4 glassmorphism border border-white/40 shadow-2xl rounded-2xl animate-fade-in pointer-events-auto">
          <div className="flex items-center gap-3">
            <div className={`w-3 h-3 rounded-full ${status === 'listening' ? 'bg-red-500 animate-pulse' : status === 'processing' ? 'bg-blue-500 animate-bounce' : status === 'error' ? 'bg-red-600' : 'bg-green-500'}`} />
            <span className="text-sm font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">
              {status === 'listening' ? 'Listening...' : status === 'processing' ? 'Thinking...' : status === 'error' ? 'Error. Try again.' : 'Action Prepared!'}
            </span>
          </div>
          {transcription && (
            <p className="mt-2 text-xs text-gray-600 dark:text-gray-400 italic">"{transcription}"</p>
          )}
        </div>
      )}

      <button
        className={`w-16 h-16 rounded-full flex items-center justify-center shadow-2xl transition-all duration-300 pointer-events-auto touch-none ${
          isRecording
            ? "bg-red-500 scale-110 ring-8 ring-red-500/20"
            : "glassmorphism border border-white/40 hover:scale-105 active:scale-95"
        }`}
        onMouseDown={startRecording}
        onMouseUp={stopRecording}
        onMouseLeave={stopRecording}
        onTouchStart={(e) => { e.preventDefault(); startRecording(); }}
        onTouchEnd={(e) => { e.preventDefault(); stopRecording(); }}
        aria-label="Voice Assistant"
      >
        <svg
          className={`w-8 h-8 ${isRecording ? "text-white" : "text-[#0066FF]"}`}
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z"
          />
        </svg>

        {isRecording && (
          <div className="absolute inset-0 flex items-center justify-center pointer-events-none">
            <div className="w-full h-full rounded-full border-4 border-white/30 animate-ping" />
            <div className="flex gap-1 items-center justify-center">
                {[1,2,3,4,5].map(i => (
                    <div key={i} className="w-1 bg-white rounded-full animate-waveform" style={{
                        height: `${Math.random() * 20 + 10}px`,
                        animationDelay: `${i * 0.1}s`
                    }} />
                ))}
            </div>
          </div>
        )}
      </button>

      <style jsx>{`
        .glassmorphism {
          background: rgba(255, 255, 255, 0.7);
          backdrop-filter: blur(20px) saturate(180%);
          -webkit-backdrop-filter: blur(20px) saturate(180%);
        }
        @media (prefers-color-scheme: dark) {
          .glassmorphism {
            background: rgba(20, 20, 25, 0.7);
          }
        }
        @keyframes waveform {
            0%, 100% { transform: scaleY(1); }
            50% { transform: scaleY(2); }
        }
        .animate-waveform {
            animation: waveform 0.5s ease-in-out infinite;
        }
      `}</style>
    </div>
  );
}
