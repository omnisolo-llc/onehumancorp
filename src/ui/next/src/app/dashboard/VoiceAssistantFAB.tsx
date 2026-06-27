'use client';

import { useState, useRef, useEffect } from 'react';

export function VoiceAssistantFAB() {
  const [isListening, setIsListening] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  const [isOffline, setIsOffline] = useState(false);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      setIsOffline(!navigator.onLine);
      const handleOnline = () => setIsOffline(false);
      const handleOffline = () => setIsOffline(true);
      window.addEventListener('online', handleOnline);
      window.addEventListener('offline', handleOffline);
      return () => {
        window.removeEventListener('online', handleOnline);
        window.removeEventListener('offline', handleOffline);
      };
    }
  }, []);
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
      if (isOffline || !navigator.onLine) {
        const reader = new FileReader();
        reader.readAsDataURL(audioBlob);
        reader.onloadend = async () => {
          const base64Data = (reader.result as string).split(',')[1];
          const action = {
            id: crypto.randomUUID(),
            type: 'voice_note_sync',
            payload: { audio_data: base64Data },
            timestamp: Date.now()
          };
          const { enqueueAction } = await import('../utils/offlineQueue');
          await enqueueAction(action);
          window.dispatchEvent(new CustomEvent('ohc_queue_updated'));
          // Dispatch event to show it was processed locally
          window.dispatchEvent(new CustomEvent('voice-command-processed', { detail: { status: 'PROPOSED_OFFLINE' } }));
        };
      } else {
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
      }
    } catch (error) {
      console.error('Error processing voice command:', error);
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div className="fixed bottom-24 right-6 z-50 flex flex-col items-center gap-2">
      {isOffline && (
        <div className="px-4 py-2 bg-amber-500/90 text-white rounded-full text-sm font-medium shadow-lg backdrop-blur-[30px] saturate-[210%] border border-amber-400/30 mb-2">
          Offline - Changes Saved Locally
        </div>
      )}

      {isListening && (
        <div className="px-4 py-2 bg-indigo-600/90 text-white rounded-full text-sm font-medium animate-pulse shadow-lg backdrop-blur-[30px] saturate-[210%] border border-indigo-400/30">
          Listening... Release to send
        </div>
      )}

      {isProcessing && (
        <div className="px-4 py-2 bg-gray-800/90 text-white rounded-full text-sm font-medium shadow-lg backdrop-blur-[30px] saturate-[210%] border border-gray-600/30">
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
        className={`w-16 h-16 min-w-[44px] min-h-[44px] rounded-full shadow-2xl flex items-center justify-center text-2xl transition-all duration-200 border-2 ${
          isListening
            ? 'bg-[#FF3B30] scale-110 border-red-400 animate-pulse shadow-[0_0_20px_rgba(239,68,68,0.6)]'
            : 'bg-indigo-600 hover:bg-indigo-500 hover:scale-105 border-indigo-400/50 backdrop-blur-[30px] saturate-[210%] bg-opacity-80'
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
