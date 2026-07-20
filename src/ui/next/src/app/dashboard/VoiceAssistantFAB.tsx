'use client';

import { useState, useRef } from 'react';
import { WithTooltip } from "../../components/TooltipRegistry";

export function VoiceAssistantFAB() {
  const [isListening, setIsListening] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  const [status, setStatus] = useState<"idle" | "listening" | "processing" | "success" | "error">("idle");
  const [transcription, setTranscription] = useState("");
  const mediaRecorderRef = useRef<MediaRecorder | null>(null);
  const audioChunksRef = useRef<BlobPart[]>([]);

  const startListening = async () => {
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
        const audioBlob = new Blob(audioChunksRef.current);
        await processVoiceCommand(audioBlob);

        // Stop all tracks to release the microphone
        stream.getTracks().forEach(track => track.stop());
      };

      mediaRecorder.start();
      setIsListening(true);
      setStatus("listening");
      setTranscription("");
    } catch (error) {
      console.error('Error accessing microphone:', error);
      alert('Could not access microphone. Please check permissions.');
      setStatus("error");
    }
  };

  const stopListening = () => {
    if (mediaRecorderRef.current && mediaRecorderRef.current.state === 'recording') {
      mediaRecorderRef.current.stop();
      setIsListening(false);
      setStatus("processing");
    }
  };

  const processVoiceCommand = async (audioBlob: Blob) => {
    setIsProcessing(true);
    setStatus("processing");
    try {
      if (!navigator.onLine) {
        const reader = new FileReader();
        reader.readAsDataURL(audioBlob);
        reader.onloadend = () => {
          const base64Audio = reader.result as string;
          import('../utils/offlineQueue').then(({ enqueueAction }) => {
            enqueueAction({
              id: crypto.randomUUID(),
              type: 'sync_event',
              timestamp: Date.now(),
              payload: {
                entity_type: 'audio_intent',
                entity_id: crypto.randomUUID(),
                action_type: 'ProcessVoiceCommand',
                payload: {
                  audio_data: base64Audio
                }
              }
            });
          });
          setTranscription("Audio captured. (Queued for Sync)");
          setStatus("success");
          if ('vibrate' in navigator) navigator.vibrate(200);

          setTimeout(() => {
            setStatus("idle");
            setTranscription("");
          }, 3000);
        };
        setIsProcessing(false);
        return;
      }

      const formData = new FormData();
      formData.append('audio', audioBlob, 'command.webm');
      const response = await fetch('/api/v1/voice/command', {
        method: 'POST',
        body: formData,
      });

      if (!response.ok) {
        throw new Error('Failed to process voice command');
      }

      const result = await response.json();
      setTranscription(result.transcription);
      setStatus("success");
      if ('vibrate' in navigator) navigator.vibrate(200);

      // In a real implementation, this would trigger a refetch of the Agent Feed
      // Dispatch a custom event to notify UnifiedAgentFeed
      window.dispatchEvent(new CustomEvent('voice-command-processed', { detail: result }));

      // Dispatch agent-feed-updated to immediately refresh the unified agent feed cards
      window.dispatchEvent(new CustomEvent('agent-feed-updated'));

      setTimeout(() => {
        setStatus("idle");
        setTranscription("");
      }, 5000);
    } catch (error) {
      console.error('Error processing voice command:', error);
      setStatus("error");
      setTimeout(() => setStatus("idle"), 3000);
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <section
      aria-label="Voice commands"
      className="mb-6 flex w-full items-center justify-between gap-4 rounded-[12px] border border-white/40 bg-white/65 p-4 shadow-sm backdrop-blur-[30px] backdrop-saturate-[2.1] dark:border-white/10 dark:bg-[#16161a]/70"
    >
      <div className="min-w-0 flex-1">
        <h2 className="font-outfit text-base font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">Voice commands</h2>
        <p className="text-sm text-gray-600 dark:text-gray-400">Hold the microphone to prepare an assistant action.</p>
      {status !== "idle" && (
        <div className="mt-3 rounded-xl border border-white/40 bg-white/65 p-3 shadow-sm backdrop-blur-[30px] backdrop-saturate-[2.1] dark:border-white/10 dark:bg-zinc-900/70">
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
      </div>

      <WithTooltip id="voice-assistant-tooltip" defaultText="Hold to speak a command to your AI Assistant.">
        <button
          onMouseDown={startListening}
          onMouseUp={stopListening}
          onMouseLeave={stopListening}
          onTouchStart={(e) => { e.preventDefault(); startListening(); }}
          onTouchEnd={(e) => { e.preventDefault(); stopListening(); }}
          disabled={isProcessing}
          className={`relative w-14 h-14 min-w-[56px] min-h-[56px] rounded-full shadow-lg flex items-center justify-center transition-all duration-300 border border-white/40 touch-none ${
            isListening
              ? 'bg-red-500 scale-110 ring-8 ring-red-500/20 shadow-[0_0_20px_rgba(239,68,68,0.6)]'
              : 'bg-white/65 dark:bg-zinc-900/70 hover:scale-105 active:scale-95 backdrop-blur-[30px] saturate-[210%]'
          } ${isProcessing ? 'opacity-50 cursor-not-allowed' : ''}`}
          aria-label="Voice Assistant"
          title="Hold to speak to your assistant"
        >
          <svg
            className={`w-8 h-8 ${isListening ? "text-white" : "text-[#0066FF]"}`}
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

          {isListening && (
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
        @keyframes waveform {
            0%, 100% { transform: scaleY(1); }
            50% { transform: scaleY(2); }
        }
        .animate-waveform {
            animation: waveform 0.5s ease-in-out infinite;
        }
      `}</style>
    </section>
  );
}
