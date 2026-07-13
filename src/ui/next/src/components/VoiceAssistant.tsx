"use client";

import { WithTooltip } from "./TooltipRegistry";
import React, { useState, useRef, useCallback, useEffect } from "react";

type VoiceStatus = "idle" | "listening" | "processing" | "success" | "error";

export function VoiceAssistant() {
  const [isRecording, setIsRecording] = useState(false);
  const [status, setStatus] = useState<VoiceStatus>("idle");
  const [transcription, setTranscription] = useState("");
  const mountedRef = useRef(false);
  const mediaRecorderRef = useRef<MediaRecorder | null>(null);
  const mediaStreamRef = useRef<MediaStream | null>(null);
  const audioChunksRef = useRef<Blob[]>([]);
  const recordingRef = useRef(false);
  const pendingRequestRef = useRef(false);
  const wantsRecordingRef = useRef(false);
  const holdActiveRef = useRef(false);
  const sessionRef = useRef(0);
  const releasedTracksRef = useRef(new WeakSet<MediaStreamTrack>());
  const resetTimersRef = useRef(new Set<ReturnType<typeof setTimeout>>());
  const suppressSyntheticClickRef = useRef(false);
  const keyboardClickResetRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const releaseStream = useCallback((stream: MediaStream | null) => {
    if (!stream) return;
    stream.getTracks().forEach((track) => {
      if (releasedTracksRef.current.has(track)) return;
      releasedTracksRef.current.add(track);
      track.stop();
    });
    if (mediaStreamRef.current === stream) mediaStreamRef.current = null;
  }, []);

  const scheduleReset = useCallback((delay: number, resetTranscription: boolean) => {
    const timer = setTimeout(() => {
      resetTimersRef.current.delete(timer);
      if (!mountedRef.current) return;
      setStatus("idle");
      if (resetTranscription) setTranscription("");
    }, delay);
    resetTimersRef.current.add(timer);
  }, []);

  const sendVoiceCommand = useCallback(async (audioBlob: Blob, session: number) => {
    if (!mountedRef.current || session !== sessionRef.current) return;
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
      if (!mountedRef.current || session !== sessionRef.current) return;

      setTranscription(data.transcription);
      setStatus("success");
      window.dispatchEvent(new CustomEvent('voice-command-processed', { detail: data }));
      scheduleReset(5000, true);
    } catch {
      if (!mountedRef.current || session !== sessionRef.current) return;
      console.error("Failed to process voice command.");
      setStatus("error");
      scheduleReset(3000, false);
    }
  }, [scheduleReset]);

  const startRecording = useCallback(async () => {
    if (pendingRequestRef.current || recordingRef.current) return;
    wantsRecordingRef.current = true;
    pendingRequestRef.current = true;
    const session = ++sessionRef.current;

    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      if (!mountedRef.current || session !== sessionRef.current || !wantsRecordingRef.current) {
        releaseStream(stream);
        return;
      }

      pendingRequestRef.current = false;
      mediaStreamRef.current = stream;
      const mediaRecorder = new MediaRecorder(stream);
      mediaRecorderRef.current = mediaRecorder;
      audioChunksRef.current = [];

      mediaRecorder.ondataavailable = (event) => {
        if (!mountedRef.current
          || session !== sessionRef.current
          || mediaRecorderRef.current !== mediaRecorder) return;
        if (event.data.size > 0) {
          audioChunksRef.current.push(event.data);
        }
      };

      mediaRecorder.onstop = async () => {
        releaseStream(stream);
        if (!mountedRef.current
          || session !== sessionRef.current
          || mediaRecorderRef.current !== mediaRecorder) return;
        mediaRecorderRef.current = null;
        recordingRef.current = false;
        const audioBlob = new Blob(audioChunksRef.current, { type: 'audio/webm' });
        await sendVoiceCommand(audioBlob, session);
      };

      mediaRecorder.start();
      recordingRef.current = true;
      setIsRecording(true);
      setStatus("listening");
      setTranscription("");
    } catch {
      if (!mountedRef.current || session !== sessionRef.current) return;
      pendingRequestRef.current = false;
      wantsRecordingRef.current = false;
      const stream = mediaStreamRef.current;
      mediaStreamRef.current = null;
      mediaRecorderRef.current = null;
      recordingRef.current = false;
      releaseStream(stream);
      console.error("Failed to start voice recording.");
      setStatus("error");
    }
  }, [releaseStream, sendVoiceCommand]);

  const stopRecording = useCallback(() => {
    wantsRecordingRef.current = false;
    if (pendingRequestRef.current && !recordingRef.current) {
      pendingRequestRef.current = false;
      sessionRef.current += 1;
      return;
    }
    const mediaRecorder = mediaRecorderRef.current;
    if (!mediaRecorder || !recordingRef.current) return;

    recordingRef.current = false;
    setIsRecording(false);
    setStatus("processing");
    try {
      if (mediaRecorder.state !== "inactive") mediaRecorder.stop();
    } catch {
      if (mountedRef.current) {
        console.error("Failed to stop voice recording.");
        setStatus("error");
      }
    }
    releaseStream(mediaStreamRef.current);
  }, [releaseStream]);

  const beginHold = useCallback(() => {
    holdActiveRef.current = true;
    void startRecording();
  }, [startRecording]);

  const endHold = useCallback(() => {
    if (!holdActiveRef.current) return;
    holdActiveRef.current = false;
    stopRecording();
  }, [stopRecording]);

  const handleKeyDown = useCallback((event: React.KeyboardEvent<HTMLButtonElement>) => {
    if (event.key !== "Enter" && event.key !== " ") return;
    event.preventDefault();
    suppressSyntheticClickRef.current = true;
    if (event.repeat) return;
    beginHold();
  }, [beginHold]);

  const handleKeyUp = useCallback((event: React.KeyboardEvent<HTMLButtonElement>) => {
    if (event.key !== "Enter" && event.key !== " ") return;
    event.preventDefault();
    endHold();
    if (keyboardClickResetRef.current) clearTimeout(keyboardClickResetRef.current);
    keyboardClickResetRef.current = setTimeout(() => {
      suppressSyntheticClickRef.current = false;
      keyboardClickResetRef.current = null;
    }, 0);
  }, [endHold]);

  const handleClick = useCallback((event: React.MouseEvent<HTMLButtonElement>) => {
    if (event.detail !== 0) return;
    if (suppressSyntheticClickRef.current) {
      suppressSyntheticClickRef.current = false;
      if (keyboardClickResetRef.current) clearTimeout(keyboardClickResetRef.current);
      keyboardClickResetRef.current = null;
      return;
    }
    if (wantsRecordingRef.current || recordingRef.current || pendingRequestRef.current) {
      stopRecording();
    } else {
      void startRecording();
    }
  }, [startRecording, stopRecording]);

  useEffect(() => {
    mountedRef.current = true;
    return () => {
      mountedRef.current = false;
      wantsRecordingRef.current = false;
      pendingRequestRef.current = false;
      sessionRef.current += 1;
      resetTimersRef.current.forEach(clearTimeout);
      resetTimersRef.current.clear();
      if (keyboardClickResetRef.current) clearTimeout(keyboardClickResetRef.current);
      keyboardClickResetRef.current = null;

      const mediaRecorder = mediaRecorderRef.current;
      if (mediaRecorder) {
        mediaRecorder.ondataavailable = null;
        mediaRecorder.onstop = null;
        try {
          if (mediaRecorder.state !== "inactive") mediaRecorder.stop();
        } catch {
          // Teardown still releases the stream below.
        }
      }
      mediaRecorderRef.current = null;
      recordingRef.current = false;
      audioChunksRef.current = [];
      releaseStream(mediaStreamRef.current);
    };
  }, [releaseStream]);

  const accessibleLabel = isRecording
    ? "Voice Assistant listening. Release Enter or Space, or activate again, to stop."
    : "Voice Assistant. Press and hold Enter or Space to speak; assistive technology users can activate to start and activate again to stop.";

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
          role="status"
          aria-live={status === "error" ? "assertive" : "polite"}
          aria-atomic="true"
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
          onMouseDown={beginHold}
          onMouseUp={endHold}
          onMouseLeave={endHold}
          onTouchStart={(event) => { event.preventDefault(); beginHold(); }}
          onTouchEnd={(event) => { event.preventDefault(); endHold(); }}
          onTouchCancel={endHold}
          onKeyDown={handleKeyDown}
          onKeyUp={handleKeyUp}
          onClick={handleClick}
          aria-label={accessibleLabel}
          aria-pressed={isRecording}
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
