# Auto-Generated Video Conferencing Links

## Problem Statement
Setting up virtual meetings for lessons or consultations involves manual steps to create links and send them to clients, often leading to confusion and lost links.

## Research Report

**Market Context:**
Videotelephony, also known as videoconferencing, video calling, or telepresence, is the use of audio and video for simultaneous two-way communication.
Videophones were standalone devices for video calling (compare Telephone). As smartphones and computers have become capable of video calling, the demand for a separate category of videophones has disappeared.
Videoconferencing implies group communication and is used in telepresence, the goal of which is to create the illusion that remote participants are in the same room.
The concept of videotelephony was conceived in the late 19th century, and versions were demonstrated to the public starting in the 1930s. In April, 1930, reporters gathered at AT&T corporate headquarters on Broadway in New York City for the first public demonstration of two-way video telephony. The event linked the headquarters building with a Bell laboratories building on West Street. Early demonstrations were installed at booths in post offices and shown at world expositions. AT&T demonstrated Picturephone at the 1964 World’s Fair in New York City.  In 1970, AT&T launched Picturephone as the first commercial personal videotelephone system. In addition to videophones, there existed image phones which exchanged still images between units every few seconds over conventional telephone lines. The development of advanced video codecs, more powerful CPUs, and high-bandwidth Internet service in the early 2000s allowed the new category of smartphones to provide high-quality low-cost color service between users almost anywhere in the world, eliminating the videophone as a separate product concept.
Applications of videotelephony include sign language transmission for deaf and speech-impaired people, distance education, telemedicine, and overcoming mobility issues. News media organizations have used videotelephony for broadcasting.

**Evaluated Tools:**

#### In-Depth Evaluation: Zoom
**Market Position**: Ubiquitous for video conferencing post-pandemic.
**Pricing**: Free for 40 mins, $15/mo for pro.
**Integration Approach**: Server-to-Server OAuth. When an appointment is booked, OHC calls the Zoom API to generate a meeting, grabs the join URL, and saves it to the OHC database. Very straightforward for both Cloud and Standalone.
**Persona Impact**: Seamless client experience. The link is just *there* in the calendar invite.

#### In-Depth Evaluation: Google Meet
**Market Position**: Bundled with Google Workspace, deeply integrated with Google Calendar.
**Pricing**: Included in Workspace.
**Integration Approach**: If the user has already synced their Google Calendar via OAuth, adding a Meet link to an event is often just setting a boolean flag in the Google Calendar API request. Extremely low friction if the user is already in the Google ecosystem.

#### In-Depth Evaluation: Microsoft Teams
**Market Position**: Dominant in corporate environments, less so for micro-businesses, but important for B2B users.
**Pricing**: Bundled with Office 365.
**Integration Approach**: Requires Microsoft Graph API integration. More complex OAuth scopes required compared to Zoom.

## Design Doc
Integrate Zoom and Google Meet APIs. When a virtual service is booked (via the Calendar module), OHC automatically creates a meeting link and embeds it in the calendar invite and confirmation emails.

## Implementation Prompt
Enhance the appointment booking flow to include a 'Virtual Meeting' toggle. If selected, automatically generate a Zoom or Google Meet link and display it prominently on the confirmation page and in notifications.

## Priority
P2

## Estimated Scope
Small
