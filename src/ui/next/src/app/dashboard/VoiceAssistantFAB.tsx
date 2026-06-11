'use client';

import { useState, useRef } from 'react';

export function VoiceAssistantFAB() {
  const [isListening, setIsListening] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
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
        const audioBlob = new Blob(audioChunksRef.current, { type: 'audio/webm' });
        await processVoiceCommand(audioBlob);

        // Stop all tracks to release the microphone
        stream.getTracks().forEach(track => track.stop());
      };

      mediaRecorder.start();
      setIsListening(true);
    } catch (error) {
      console.error('Error accessing microphone:', error);
      alert('Could not access microphone. Please check permissions.');
    }
  };

  const stopListening = () => {
    if (mediaRecorderRef.current && mediaRecorderRef.current.state === 'recording') {
      mediaRecorderRef.current.stop();
      setIsListening(false);
    }
  };

  const processVoiceCommand = async (audioBlob: Blob) => {
    setIsProcessing(true);
    try {
      const formData = new FormData();
      formData.append('audio', audioBlob, 'command.webm');
      formData.append('tenant_id', localStorage.getItem('tenant_id') || 'default'); // Would normally get from context

      const response = await fetch('/api/v1/voice/command', {
        method: 'POST',
        body: formData,
      });

      if (!response.ok) {
        throw new Error('Failed to process voice command');
      }

      const result = await response.json();

      // In a real implementation, this would trigger a refetch of the Agent Feed
      // For now, we can dispatch a custom event to notify UnifiedAgentFeed
      window.dispatchEvent(new CustomEvent('voice-command-processed', { detail: result }));

    } catch (error) {
      console.error('Error processing voice command:', error);
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div className="fixed bottom-24 right-6 z-50 flex flex-col items-center gap-2">
      {isListening && (
        <div className="px-4 py-2 bg-indigo-600/90 text-white rounded-full text-sm font-medium animate-pulse shadow-lg backdrop-blur-md border border-indigo-400/30">
          Listening... Release to send
        </div>
      )}

      {isProcessing && (
        <div className="px-4 py-2 bg-gray-800/90 text-white rounded-full text-sm font-medium shadow-lg backdrop-blur-md border border-gray-600/30">
          Processing command...
        </div>
      )}

      <button
        onMouseDown={startListening}
        onMouseUp={stopListening}
        onMouseLeave={stopListening}
        onTouchStart={startListening}
        onTouchEnd={stopListening}
        disabled={isProcessing}
        className={`w-16 h-16 rounded-full shadow-2xl flex items-center justify-center text-2xl transition-all duration-200 border-2 ${
          isListening
            ? 'bg-red-500 scale-110 border-red-400 animate-pulse shadow-[0_0_20px_rgba(239,68,68,0.6)]'
            : 'bg-indigo-600 hover:bg-indigo-500 hover:scale-105 border-indigo-400/50 backdrop-blur-xl bg-opacity-80'
        } ${isProcessing ? 'opacity-50 cursor-not-allowed' : ''}`}
        aria-label="Voice Command Assistant"
        title="Hold to speak to your assistant"
      >
        <span className="text-white drop-shadow-md">
          {isListening ? '🎙️' : '🎤'}
        </span>
      </button>
    </div>
  );
}
