"use client";

import React, { useEffect, useState } from 'react';
import Link from 'next/link';
import { VideoTutorial, VideoData } from '../../../components/VideoTutorial';

export default function TutorialsPage() {
  const [videos, setVideos] = useState<VideoData[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch('/api/videos')
      .then(res => res.json())
      .then(data => {
        setVideos(data);
        setLoading(false);
      })
      .catch(err => {
        console.error(err);
        setLoading(false);
      });
  }, []);

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-6xl mx-auto">
        <div className="mb-8">
          <Link href="/help" className="text-blue-600 hover:text-blue-800 font-medium flex items-center gap-2">
            &larr; Back to Help Center
          </Link>
        </div>

        <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-2">Video Tutorials</h1>
        <p className="text-gray-600 mb-8 text-lg">Watch short guides on how to make the most out of your store.</p>

        {loading ? (
          <div className="flex justify-center py-20">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
          </div>
        ) : (
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
            {videos.map((video) => (
              <VideoTutorial key={video.id} video={video} />
            ))}
          </div>
        )}
      </div>
    </div>
  );
}