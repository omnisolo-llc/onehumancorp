import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json([
    { id: 1, title: "How to set up your first store easily", duration: "1:20" },
    { id: 2, title: "Linking your own website name", duration: "0:45" },
    { id: 3, title: "Getting paid for the first time", duration: "1:10" },
    { id: 4, title: "Hiring your first AI helper", duration: "1:05" },
    { id: 5, title: "Adding and editing your products", duration: "0:55" },
    { id: 6, title: "Sending emails to your customers", duration: "1:15" },
    { id: 7, title: "Seeing how much you sold", duration: "0:50" },
    { id: 8, title: "What to do when you get an order", duration: "1:00" },
    { id: 9, title: "Changing colors and logos", duration: "1:25" },
    { id: 10, title: "Adding staff to your account", duration: "0:40" }
  ]);
}
