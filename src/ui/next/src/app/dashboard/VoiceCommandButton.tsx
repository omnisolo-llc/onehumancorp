'use client';

import React, { useState, useRef, useEffect } from 'react';

export function VoiceCommandButton() {
  const [isListening, setIsListening] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  const mediaRecorderRef = useRef<MediaRecorder | null>(null);
  const audioChunksRef = useRef<BlobPart[]>([]);

  useEffect(() => {
    // Cleanup on unmount
    return () => {
      if (mediaRecorderRef.current && mediaRecorderRef.current.state === 'recording') {
        mediaRecorderRef.current.stop();
      }
    };
  }, []);

  const startRecording = async () => {
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
        await sendAudioCommand(audioBlob);

        // Stop all tracks to release the microphone
        stream.getTracks().forEach((track) => track.stop());
      };

      mediaRecorder.start();
      setIsListening(true);
    } catch (err) {
      console.error('Microphone access denied or not available', err);
    }
  };

  const stopRecording = () => {
    if (mediaRecorderRef.current && mediaRecorderRef.current.state === 'recording') {
      mediaRecorderRef.current.stop();
      setIsListening(false);
    }
  };

  const sendAudioCommand = async (audioBlob: Blob) => {
    setIsProcessing(true);
    const formData = new FormData();
    formData.append('audio', audioBlob, 'command.webm');

    try {
      const response = await fetch('/api/v1/voice/command', {
        method: 'POST',
        body: formData,
      });

      if (!response.ok) {
        console.error('Failed to send voice command', await response.text());
        return;
      }

      const result = await response.json();
      console.log('Voice command result:', result);

      // Force a revalidation or refresh of the page to show the new card
      // In a real app we might use a React Context or an event to trigger a refresh of the UnifiedAgentFeed
      window.dispatchEvent(new Event('refreshAgentFeed'));

    } catch (err) {
      console.error('Error sending voice command', err);
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div className="fixed bottom-6 w-full flex justify-center z-50 pointer-events-none px-4" style={{ paddingLeft: 'env(safe-area-inset-left)', paddingRight: 'env(safe-area-inset-right)' }}>
      <button
        onPointerDown={startRecording}
        onPointerUp={stopRecording}
        onPointerLeave={stopRecording}
        disabled={isProcessing}
        data-testid="voice-command-button"
        className={`pointer-events-auto flex items-center justify-center rounded-full shadow-2xl transition-all duration-300 ease-out
          ${isListening
            ? 'w-20 h-20 bg-blue-600 scale-110 shadow-blue-500/50'
            : 'w-16 h-16 bg-white/80 dark:bg-[#16161A]/80 backdrop-blur-xl border border-white/40 dark:border-white/10 text-gray-900 dark:text-white'}
          ${isProcessing ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer hover:scale-105'}`}
      >
        {isListening ? (
          <div className="flex items-center gap-1">
            <span className="w-1.5 h-4 bg-white rounded-full animate-[bounce_1s_infinite]"></span>
            <span className="w-1.5 h-6 bg-white rounded-full animate-[bounce_1s_infinite_0.2s]"></span>
            <span className="w-1.5 h-4 bg-white rounded-full animate-[bounce_1s_infinite_0.4s]"></span>
          </div>
        ) : (
          <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6 text-blue-600 dark:text-blue-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z" />
          </svg>
        )}
      </button>

      {/* Optional: Add the existing FAB beside it or handle it in layout */}
    </div>
  );
}
