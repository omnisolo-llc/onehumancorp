import { NextResponse } from 'next/server';

const mockVideos = [
  {
    id: "vid1",
    title: "Set up your store in 60 seconds",
    url: "https://example.com/setup-store.mp4",
    duration: 60,
    thumbnail: "https://example.com/thumb1.jpg"
  },
  {
    id: "vid2",
    title: "How to issue a refund",
    url: "https://example.com/refund.mp4",
    duration: 45,
    thumbnail: "https://example.com/thumb2.jpg"
  },
  {
    id: "vid3",
    title: "Adding team members",
    url: "https://example.com/team.mp4",
    duration: 85,
    thumbnail: "https://example.com/thumb3.jpg"
  }
];

export async function GET() {
  return NextResponse.json(mockVideos);
}
