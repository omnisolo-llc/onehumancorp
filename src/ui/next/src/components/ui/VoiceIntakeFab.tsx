"use client";

import React, { useState, useRef } from 'react';
import { Button } from './button';
import { FaMicrophone, FaStop } from 'react-icons/fa';

export function VoiceIntakeFab() {
  const [isRecording, setIsRecording] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  const mediaRecorder = useRef<MediaRecorder | null>(null);
  const audioChunks = useRef<BlobPart[]>([]);

  const handleStartRecording = async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      mediaRecorder.current = new MediaRecorder(stream);
      audioChunks.current = [];

      mediaRecorder.current.ondataavailable = (e) => {
        if (e.data.size > 0) {
          audioChunks.current.push(e.data);
        }
      };

      mediaRecorder.current.onstop = async () => {
        setIsProcessing(true);
        const audioBlob = new Blob(audioChunks.current, { type: 'audio/webm' });
        const formData = new FormData();
        formData.append('audio', audioBlob, 'voice-command.webm');

        try {
          const response = await fetch('/api/v1/voice/intake', {
            method: 'POST',
            body: formData,
          });

          if (!response.ok) {
            console.error('Voice intake failed:', response.statusText);
          }
        } catch (err) {
          console.error('Error sending voice command:', err);
        } finally {
          setIsProcessing(false);
        }
      };

      mediaRecorder.current.start();
      setIsRecording(true);
    } catch (err) {
      console.error('Error accessing microphone:', err);
      setIsRecording(false);
    }
  };

  const handleStopRecording = () => {
    if (mediaRecorder.current && mediaRecorder.current.state === 'recording') {
      mediaRecorder.current.stop();
    }
    setIsRecording(false);
  };

  return (
    <>
      {isRecording && (
        <div className="fixed inset-0 bg-black/40 backdrop-blur-sm z-50 flex flex-col items-center justify-center p-4">
          <div className="bg-white/10 backdrop-blur-md border border-white/20 p-8 rounded-2xl flex flex-col items-center gap-6 shadow-2xl glassmorphism" style={{ borderRadius: '16px' }}>
            <div className="flex items-center gap-2 h-12">
              <div className="w-2 bg-blue-500 rounded-full animate-[wave_1s_ease-in-out_infinite]" style={{ height: '40%' }}></div>
              <div className="w-2 bg-blue-500 rounded-full animate-[wave_1s_ease-in-out_0.2s_infinite]" style={{ height: '80%' }}></div>
              <div className="w-2 bg-blue-500 rounded-full animate-[wave_1s_ease-in-out_0.4s_infinite]" style={{ height: '100%' }}></div>
              <div className="w-2 bg-blue-500 rounded-full animate-[wave_1s_ease-in-out_0.6s_infinite]" style={{ height: '60%' }}></div>
              <div className="w-2 bg-blue-500 rounded-full animate-[wave_1s_ease-in-out_0.8s_infinite]" style={{ height: '40%' }}></div>
            </div>
            <p className="text-white text-lg font-medium">Listening...</p>
            <Button onClick={handleStopRecording} variant="destructive" className="rounded-full w-16 h-16 flex items-center justify-center" aria-label="Stop recording">
              <FaStop className="text-2xl" />
            </Button>
          </div>
        </div>
      )}

      {isProcessing && (
        <div className="fixed inset-0 bg-black/40 backdrop-blur-sm z-50 flex items-center justify-center">
          <div className="bg-white p-6 rounded-2xl shadow-xl flex items-center gap-4">
            <div className="w-6 h-6 border-4 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
            <span className="font-medium text-gray-800">Agent Triage in progress...</span>
          </div>
        </div>
      )}

      <div className="fixed bottom-[80px] right-4 z-40">
        <Button
          onClick={handleStartRecording}
          disabled={isProcessing}
          className="rounded-full w-14 h-14 bg-blue-600 hover:bg-blue-700 shadow-lg flex items-center justify-center"
          aria-label="Start voice intake"
        >
          <FaMicrophone className="text-xl text-white" />
        </Button>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @keyframes wave {
          0%, 100% { transform: scaleY(0.5); }
          50% { transform: scaleY(1); }
        }
      `}} />
    </>
  );
}
