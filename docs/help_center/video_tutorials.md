# Video Tutorials System

*Note: This document details how short-form video tutorials are embedded and managed within the app.*

## Philosophy
Small business owners don't have time to watch 10-minute webinars. Our video tutorials are modeled after TikTok/Reels: they are portrait-oriented (9:16 aspect ratio), highly visual, and strictly under 90 seconds.

## Top 10 Video Library
We maintain 10 core videos covering the most critical paths. These are accessible from the Help Center and embedded directly on relevant screens.

1. **Adding Your First Product (0:45)**
2. **Connecting Your Bank Account (0:30)**
3. **Fulfilling an Order (1:15)**
4. **Processing a Refund (0:40)**
5. **Hiring an AI Support Agent (1:20)**
6. **Setting Up a Discount Code (0:50)**
7. **Sending an Email Campaign (1:10)**
8. **Reading Your Daily Sales Chart (0:55)**
9. **Inviting an Employee (0:35)**
10. **Customizing Store Colors (0:45)**

## Technical Implementation

### Storage
Videos are stored in an S3 bucket and served via a global CDN to ensure fast loading even on slow cellular connections.

### Metadata API
The backend serves video metadata to the app so the client knows what video to show on what screen.

```json
{
  "id": "vid_add_product",
  "title": "Adding Your First Product",
  "duration_seconds": 45,
  "url_hls": "https://cdn.onehumancorp.com/videos/add_product/master.m3u8",
  "thumbnail_url": "https://cdn.onehumancorp.com/videos/add_product/thumb.jpg",
  "context_screen": "inventory_empty_state"
}
```

### Video Player UI
- The player uses `react-player`.
- Automatically defaults to muted auto-play when appearing in an empty state on desktop.
- On mobile, it displays a large Play button over the thumbnail to respect data caps.
- Features standard controls: Play/Pause, timeline scrubber, and full-screen toggle.

## Video Creation Guidelines

If you are on the internal content team creating new videos, please follow these rules:

1. **Aspect Ratio:** All videos must be shot and edited in 9:16 (vertical/portrait). They are designed to be viewed on mobile devices.
2. **Length:** Absolute maximum length is 90 seconds. 45-60 seconds is the sweet spot. If a topic takes longer than 90 seconds, break it down into two separate videos.
3. **Pacing:** Get straight to the point. Skip long intro animations or "Hi everyone, welcome back to..." speeches. Start the action in the first 3 seconds.
4. **Visual Cues:** Since many users watch on mute, use large, legible on-screen text to highlight key steps. Use animated circles or arrows to point out small buttons.
5. **Tone:** Upbeat, encouraging, and clear. Speak as if you are helping a friend set up their phone.

## Translating Videos
We aim to support our global user base.
- We do not currently dub the audio for videos.
- Instead, we provide closed caption (VTT) files for Spanish, French, and German.
- The video player automatically selects the correct subtitle track based on the user's device language settings.

## Updating Existing Videos
When the app's UI changes significantly, the videos must be re-recorded.
1. Create the new video following the guidelines above.
2. Upload it to the S3 bucket using the *exact same filename* as the old video, but append a version number (e.g., `add_product_v2.mp4`).
3. Update the metadata API to point to the new URL. Do not delete the old video immediately; keep it for 30 days as a fallback.
