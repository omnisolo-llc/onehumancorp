import React from 'react';

type HelpVideo = { id: number; title: string; duration: string; video_url?: string; };

interface HelpVideosProps {
  videos: HelpVideo[];
  setActiveVideo: (video: HelpVideo) => void;
}

export function HelpVideos({ videos, setActiveVideo }: HelpVideosProps) {
  return (
    <div className="backdrop-blur-[30px] saturate-[210%] bg-[rgba(255,255,255,0.65)] border-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.7)] dark:border-[rgba(255,255,255,0.1)] p-4 rounded-xl border shadow-sm">
      <h3 className="font-bold font-outfit text-gray-900 mb-4 text-xl">Tutorials</h3>
      <div className="grid grid-cols-2 gap-4">
        {videos.map((v) => (
          <div key={v.id} onClick={() => setActiveVideo(v)} className="aspect-[9/16] bg-gray-200 rounded-2xl flex items-center justify-center relative overflow-hidden group cursor-pointer shadow-sm border border-white/30 dark:border-white/10">
            <div className="absolute inset-0 bg-black/30 group-hover:bg-black/20 transition-all"></div>
              <div className="w-10 h-10 bg-white/90 backdrop-blur-3xl saturate-[210%] rounded-full flex items-center justify-center shadow-lg z-10 group-hover:scale-110 transition-transform">
              <svg className="w-5 h-5 text-blue-600 ml-1" fill="currentColor" viewBox="0 0 20 20"><path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clipRule="evenodd" /></svg>
            </div>
            <div className="absolute bottom-2 left-2 right-2 z-10">
              <p className="text-white text-xs font-bold drop-shadow-md line-clamp-2 leading-tight">{v.title}</p>
              <p className="text-white/80 text-[10px] font-medium mt-0.5">{v.duration}</p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
