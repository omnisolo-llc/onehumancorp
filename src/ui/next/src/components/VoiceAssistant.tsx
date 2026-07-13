"use client";

import { WithTooltip } from "./TooltipRegistry";
import React, { useState, useRef, useCallback } from "react";

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
        const audioBlob = new Blob(audioChunksRef.current, { type: 'audio/webm' });
        await sendVoiceCommand(audioBlob);

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

  const sendVoiceCommand = async (audioBlob: Blob) => {
    try {
      const formData = new FormData();
      formData.append('audio', audioBlob, 'command.webm');
      formData.append('tenant_id', localStorage.getItem('tenant_id') || 'default');

      const response = await fetch("/api/v1/voice/command", {
        method: "POST",
        headers: {
          "Authorization": `Bearer ${localStorage.getItem('token')}`
        },
        body: formData,
      });

      if (!response.ok) throw new Error("Voice command failed");

      const data = await response.json();
      setTranscription(data.transcription);
      setStatus("success");

      // Dispatch event for Agent Feed / Unified Inbox
      window.dispatchEvent(new CustomEvent('voice-command-processed', { detail: data }));

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
    <div
      className="relative z-[100] flex w-full min-w-0 flex-col items-end gap-2 pointer-events-none sm:fixed sm:bottom-6 sm:left-1/2 sm:w-full sm:max-w-[375px] sm:-translate-x-1/2 sm:items-center sm:gap-4 sm:px-4"
      data-voice-assistant-root
    >
      {status !== "idle" && (
        <div
          className="w-full min-w-0 p-4 glassmorphism border border-white/40 shadow-2xl rounded-2xl animate-fade-in pointer-events-auto"
          data-voice-assistant-state={status}
          data-voice-assistant-surface="status"
        >
          <div className="flex items-center gap-3">
            <div className={`w-3 h-3 rounded-full ${status === 'listening' ? 'bg-red-500 animate-pulse' : status === 'processing' ? 'bg-blue-500 animate-bounce' : status === 'error' ? 'bg-red-600' : 'bg-green-500'}`} />
            <span className="text-sm font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">
              {status === 'listening' ? 'Listening...' : status === 'processing' ? 'Processing command...' : status === 'error' ? 'Error. Try again.' : 'Action Prepared!'}
            </span>
          </div>
          {transcription && (
            <p className="mt-2 text-xs text-gray-600 dark:text-gray-400 italic">"{transcription}"</p>
          )}
        </div>
      )}

      <WithTooltip id="voice-assistant-tooltip" defaultText="Hold to speak a command to your AI Assistant.">
        <button
          className={`w-16 h-16 rounded-full flex items-center justify-center shadow-2xl transition-all duration-300 pointer-events-auto touch-none ${
            isRecording
              ? "bg-red-500 ring-8 ring-red-500/20 sm:scale-110"
              : "glassmorphism border border-white/40 hover:scale-105 active:scale-95"
          }`}
          onMouseDown={startRecording}
          onMouseUp={stopRecording}
          onMouseLeave={stopRecording}
          onTouchStart={(e) => { e.preventDefault(); startRecording(); }}
          onTouchEnd={(e) => { e.preventDefault(); stopRecording(); }}
          aria-label="Voice Assistant"
          data-voice-assistant-surface="trigger"
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
      </WithTooltip>

      <style>{`
        .glassmorphism {
          background: rgba(255, 255, 255, 0.65);
          backdrop-filter: blur(30px) saturate(210%);
          -webkit-backdrop-filter: blur(30px) saturate(210%);
          border: 1px solid rgba(255, 255, 255, 0.4);
        }
        @media (prefers-color-scheme: dark) {
          .glassmorphism {
            background: rgba(22, 22, 26, 0.7);
            border: 1px solid rgba(255, 255, 255, 0.1);
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
