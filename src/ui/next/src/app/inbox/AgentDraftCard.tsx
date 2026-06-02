import React, { useState, useRef } from 'react';

type Props = {
  msgId: number;
  draft: string;
  editingId: number | null;
  replyInput: string;
  setReplyInput: (val: string) => void;
  setEditingId: (id: number | null) => void;
  sendReply: (msgId: number) => void;
};

export default function AgentDraftCard({
  msgId,
  draft,
  editingId,
  replyInput,
  setReplyInput,
  setEditingId,
  sendReply,
}: Props) {
  const [swipeOffset, setSwipeOffset] = useState(0);
  const startX = useRef<number | null>(null);

  const handleTouchStart = (e: React.TouchEvent | React.PointerEvent) => {
    if (editingId === msgId) return;
    if ('touches' in e) {
      startX.current = e.touches[0].clientX;
    } else {
      startX.current = e.clientX;
      (e.target as HTMLElement).setPointerCapture(e.pointerId);
    }
  };

  const handleTouchMove = (e: React.TouchEvent | React.PointerEvent) => {
    if (editingId === msgId || startX.current === null) return;

    let currentX = 0;
    if ('touches' in e) {
        currentX = e.touches[0].clientX;
    } else {
        currentX = e.clientX;
    }

    const diff = currentX - startX.current;

    // Limit swipe distance
    if (diff > 100) setSwipeOffset(100);
    else if (diff < -100) setSwipeOffset(-100);
    else setSwipeOffset(diff);
  };

  const handleTouchEnd = () => {
    if (editingId === msgId || startX.current === null) return;
    startX.current = null;

    if (swipeOffset > 75) {
      // Swiped right enough to trigger Send
      sendReply(msgId);
    } else if (swipeOffset < -75) {
      // Swiped left enough to trigger Edit
      setEditingId(msgId);
      setReplyInput(draft);
    }
    setSwipeOffset(0);
  };

  const isEditing = editingId === msgId;

  // Render background actions to show during swipe
  const renderBackgroundActions = () => {
    if (isEditing) return null;
    return (
      <div className="absolute inset-0 flex items-center justify-between px-4 rounded-[16px] pointer-events-none" style={{ zIndex: 0 }}>
        <div className={`font-bold flex items-center gap-1 transition-opacity ${swipeOffset > 0 ? 'opacity-100 text-[#00C24B]' : 'opacity-0'}`}>
          <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" /></svg>
          Send
        </div>
        <div className={`font-bold flex items-center gap-1 transition-opacity ${swipeOffset < 0 ? 'opacity-100 text-[#0066FF]' : 'opacity-0'}`}>
           <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" /></svg>
           Edit
        </div>
      </div>
    );
  };

  return (
    <div className="mt-3 ml-4 relative">
      {renderBackgroundActions()}
      <div
        className="bg-[rgba(255,255,255,0.65)] border border-[rgba(255,255,255,0.4)] rounded-[16px] p-3 shadow-md relative backdrop-blur-[30px] saturate-[210%] transition-transform duration-100 ease-out z-10 select-none"
        style={{ transform: `translateX(${swipeOffset}px)` }}
        onPointerDown={handleTouchStart}
        onPointerMove={handleTouchMove}
        onPointerUp={handleTouchEnd}
        onPointerCancel={handleTouchEnd}
        onTouchStart={handleTouchStart}
        onTouchMove={handleTouchMove}
        onTouchEnd={handleTouchEnd}
      >
        <div className="absolute -top-3 left-4 bg-white/80 text-[#0066FF] border border-[#0066FF]/20 text-[10px] font-bold px-2 py-0.5 rounded-full uppercase tracking-wide flex items-center gap-1 backdrop-blur-sm">
            <svg className="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
            AI Draft
        </div>

        {isEditing ? (
            <div className="mt-2">
              <textarea
                id={`reply-input-edit-${msgId}`}
                value={replyInput}
                onChange={e => setReplyInput(e.target.value)}
                className="w-full border border-gray-200 rounded-[8px] p-2 text-sm text-black bg-white/80 focus:outline-none focus:ring-1 focus:ring-[#0066FF]"
                rows={3}
              />
              <div className="flex justify-end mt-2 gap-2">
                  <button onClick={() => setEditingId(null)} className="text-xs font-semibold text-gray-500 hover:text-gray-700 px-3 py-1.5 rounded-[8px]">Cancel</button>
                  <button onClick={() => sendReply(msgId)} className="bg-[#0066FF] text-white text-xs font-bold px-4 py-1.5 rounded-[8px] shadow-sm hover:bg-[#005bb5] transition-colors">Send</button>
              </div>
            </div>
        ) : (
            <>
              <p className="text-sm text-gray-800 mt-2 italic pointer-events-none">"{draft}"</p>
              <div className="flex gap-2 mt-3 pt-3 border-t border-gray-200/50">
                  <button onClick={() => sendReply(msgId)} className="flex-1 bg-[#34C759] text-white font-bold py-2 rounded-[8px] text-sm shadow-sm hover:bg-[#2eb350] transition-colors flex items-center justify-center gap-1">
                      <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" /></svg>
                      Send
                  </button>
                  <button onClick={() => { setEditingId(msgId); setReplyInput(draft); }} className="flex-1 bg-white/80 text-[#0066FF] border border-[#0066FF]/20 font-bold py-2 rounded-[8px] text-sm shadow-sm hover:bg-white transition-colors flex items-center justify-center gap-1">
                      <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" /></svg>
                      Edit
                  </button>
              </div>
            </>
        )}
      </div>
    </div>
  );
}
