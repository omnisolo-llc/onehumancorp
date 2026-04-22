# Buffer Integration
## Problem Statement
Small business owners, like Maya the baker, need a unified way to manage and schedule their social media posts across platforms (Instagram, Facebook, X, LinkedIn) from a single interface, rather than logging into each app individually.

## Research Report
**Tool**: Buffer
**Ease of use**: High. Provides a clean, intuitive dashboard designed specifically for small businesses and creators.
**Pricing**: Free tier includes up to 3 channels and 10 scheduled posts per channel. Paid plans start at $6/month per channel.
**Reputation**: Buffer is one of the most established and beloved social media management tools for small teams and solo creators.

## Design Doc
**Cloud Mode**: Integrate via Buffer API. OHC allows business owners to connect their Buffer account. Users can compose posts within the OHC dashboard, which are then sent to the Buffer API for scheduling and publication across selected channels.
**Standalone Mode**: As a cloud service, Buffer requires internet connectivity to schedule and post. The OHC standalone mode can queue posts locally and push them to the Buffer API when online.
**Triggers**: User initiates a social media post creation or scheduling from the OHC "Marketing & Advertising" department interface.
**User Experience**: Business owners draft their posts (with AI assistance, if desired) within OHC. They select target channels and publish times. OHC hands the post off to Buffer. The owner can view published post performance directly in OHC via basic metrics pulled from the Buffer API.

## Implementation Prompt
Integrate Buffer into the OHC platform to enable social media scheduling.
**Acceptance Criteria**:
1. Business owners can connect their Buffer account via OAuth.
2. Provide a post composer in the OHC dashboard allowing users to draft text and attach media.
3. Users can select connected social channels and a publish date/time.
4. OHC submits the drafted post to the Buffer API for scheduling.

## Priority
P2

## Estimated Scope
Medium
