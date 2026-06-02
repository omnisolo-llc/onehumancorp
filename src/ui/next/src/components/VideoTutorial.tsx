import React from 'react';

export interface VideoData {
  id: number;
  title: string;
  duration: string;
  description: string;
  url: string;
}

interface VideoTutorialProps {
  video: VideoData;
}

export function VideoTutorial({ video }: VideoTutorialProps) {
  return (
    <div className="bg-white/80 backdrop-blur-[20px] saturate-200 rounded-xl overflow-hidden shadow-sm border border-gray-100 flex flex-col h-full hover:shadow-md transition-all">
      <div className="relative pt-[177.77%] sm:pt-[56.25%] w-full bg-black">
        {/* We use aspect-video on larger screens, but mobile-first is portrait style or we can just use a normal responsive iframe approach. Let's make it responsive. */}
        <video
          className="absolute top-0 left-0 w-full h-full object-cover"
          controls
          src={video.url}
          preload="metadata"
        >
          Your browser does not support the video tag.
        </video>
      </div>
      <div className="p-4 flex flex-col flex-grow">
        <div className="flex justify-between items-start mb-2">
          <h3 className="font-bold text-gray-900 font-outfit text-lg">{video.title}</h3>
          <span className="text-xs font-medium bg-blue-100 text-blue-800 px-2 py-1 rounded-full">{video.duration}</span>
        </div>
        <p className="text-gray-600 text-sm flex-grow">{video.description}</p>
      </div>
    </div>
  );
}