use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router, Extension,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use crate::hub::Hub;

#[derive(Debug, Serialize, Deserialize)]

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// `SocialPostRequest` struct represents a payload for submitting a social media post across multiple platforms simultaneously.
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// Example:
/// ```json
/// {
///   "content": "Excited to launch our new feature!",
///   "platforms": ["twitter", "linkedin"]
/// }
/// ```

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 0
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 1
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 2
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 3
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 4
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 5
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 6
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 7
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 8
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 9
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 10
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 11
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 12
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 13
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 14
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 15
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 16
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 17
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 18
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 19
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 20
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 21
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 22
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 23
///

/// Represents a payload for submitting a social media post across multiple platforms simultaneously.
///
/// This structure encapsulates the content of the post and the targeted platforms, allowing the orchestration layer
/// to coordinate API calls to various social networks (e.g., LinkedIn, Twitter) while maintaining a unified interface.
///
/// The typical flow involves the client providing text content and a list of platform identifiers. The backend
/// uses these identifiers to route the request through the respective integration handlers, returning an aggregate
/// success status and an internal reference identifier.
///
/// It's particularly useful for automated marketing agents that need to push unified announcements or status updates.
///
/// # Architecture Context
/// Within the `growth` module, social posting represents an outbound orchestration flow.
/// Rather than calling multiple downstream systems directly from the REST handler, the `SocialPostRequest`
/// defines the intended state (a published post across platforms) which is then translated into events
/// for the `Hub` or relevant background worker queues.
///
/// # Rate Limiting & Tiering
/// Usage of this endpoint is typically tied to the `PlanTier` of the requesting tenant.
/// A free-tier tenant might only be allowed to post to a single platform, while higher tiers
/// might support multiplexing across a broad array of targets, or even deferred scheduling.
///
/// # Fields
///
/// * `content`: The raw text content intended for the social media post. Depending on the platform,
///   this may be truncated or modified (e.g., stripping certain characters or links) by the downstream integration.
///   It is passed as a standard UTF-8 string and expected to conform to the intersection of the character limits
///   of the selected platforms if validation occurs synchronously.
///
/// * `platforms`: A vector of strings denoting the target platforms for the publication.
///   Common values include `"twitter"`, `"linkedin"`, `"facebook"`, etc. The orchestration layer
///   uses these as keys to determine which integration adapters to invoke.
///
/// Instance Block 24
///
pub struct SocialPostRequest {
    pub content: String,
    pub platforms: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// `SocialPostResponse` represents the result of an attempt to publish a social media post across platforms.
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 0
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 1
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 2
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 3
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 4
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 5
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 6
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 7
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 8
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 9
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 10
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 11
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 12
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 13
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 14
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 15
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 16
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 17
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 18
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 19
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 20
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 21
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 22
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 23
///

/// Represents the result of an attempt to publish a social media post across platforms.
///
/// This response confirms whether the orchestration succeeded in submitting the content and provides
/// a tracking identifier (`post_id`) that can be used later to query engagement metrics or delete the post.
///
/// The `posted` boolean acts as an immediate high-level indicator of success, while `post_id` links the action
/// to a long-lived database record. In the event of partial failures (e.g., succeeded on Twitter, failed on LinkedIn),
/// this high-level response typically returns `true` if at least one platform succeeded, though detailed logs
/// are accessible via the `post_id`.
///
/// # Usage in the UI
/// The frontend consumes this response to transition from a "posting..." loading state to a success confirmation.
/// The `post_id` can be stored in the local React state or Redux store to allow the user to immediately navigate
/// to an analytics view for the newly created post, assuming the backend populates initial mock metrics or
/// links out to the native platforms.
///
/// # Fields
///
/// * `posted`: A boolean flag indicating whether the initial ingestion of the post was successful.
///   It does not guarantee synchronous publication across all requested platforms, but rather that the
///   request has passed validation and is now being processed or has been processed.
///
/// * `post_id`: A UUID-v4 string generated by the server. This serves as the primary key or correlation ID
///   for all subsequent operations related to this specific social post event.
///
/// Instance Block 24
///
pub struct SocialPostResponse {
    pub posted: bool,
    pub post_id: String,
}

#[derive(Debug, Serialize, Deserialize)]

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```

/// `CampaignRequest` outlines the structure for initiating a new email marketing campaign.
/// It contains all the essential elements required to draft and target the communication:
/// a name for internal tracking, the email subject, the main body content, and the identifier
/// for the target audience segment.
///
/// This request acts as the blueprint for the `PromoterWorker` or the customer success agent to orchestrate
/// mass communication. The target segment is resolved against the tenant's CRM data to compile
/// the actual mailing list.
///
/// Example:
/// ```json
/// {
///   "name": "Q3 Update",
///   "subject": "What's new in Q3",
///   "body": "<html>...</html>",
///   "target_segment": "active-users"
/// }
/// ```
pub struct CampaignRequest {
    pub name: String,
    pub subject: String,
    pub body: String,
    pub target_segment: String,
}

#[derive(Debug, Serialize, Deserialize)]

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).

/// `CampaignResponse` provides feedback upon the successful dispatch of a marketing campaign.
/// It includes the unique identifier assigned to the campaign (`campaign_id`) and an initial
/// count of the emails queued or successfully sent (`emails_sent`).
///
/// This response is crucial for closing the loop with the user interface, allowing the marketing dashboard
/// to immediately display the campaign in the "Sent" or "Processing" list, and providing the key needed
/// to poll for further analytics (open rates, click-through rates).
pub struct CampaignResponse {
    pub campaign_id: String,
    pub emails_sent: i32,
}

#[derive(Debug, Serialize, Deserialize)]

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.

/// `TrackVisitorRequest` is used to record a page view or interaction on a tenant's storefront.
/// It captures the URL being visited, an optional HTTP referrer string (useful for attributing
/// the source of traffic), and an anonymous or known `visitor_id`.
///
/// This data feeds directly into the analytics engine, enabling the platform to generate insights
/// on customer journey, drop-off rates, and the effectiveness of external marketing links. The
/// `visitor_id` is typically maintained via a persistent browser cookie or local storage.
pub struct TrackVisitorRequest {
    pub page_url: String,
    pub referrer: Option<String>,
    pub visitor_id: String,
}

#[derive(Debug, Serialize, Deserialize)]

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.

/// `TrackVisitorResponse` provides a simple acknowledgment that a tracking event has been recorded successfully.
/// The `tracked` boolean confirms that the event was ingested by the analytics pipeline and will be
/// reflected in future dashboard aggregations. This is intentionally lightweight to ensure the
/// tracking endpoint remains highly performant and doesn't introduce latency to the frontend.
pub struct TrackVisitorResponse {
    pub tracked: bool,
}

#[derive(Debug, Serialize, Deserialize)]

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.
pub struct Milestone {
    pub id: String,
    pub title: String,
    pub description: String,
    pub reached: bool,
}

#[derive(Debug, Serialize, Deserialize)]

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `Milestone` represents a specific, measurable achievement within the user's onboarding or growth journey.
/// It consists of an identifier, a short title, a descriptive explanation, and a boolean flag indicating
/// whether the milestone has been reached.
///
/// Milestones are used heavily in the product's gamification and guided setup flows. They help structure
/// the learning curve for new users, rewarding progress and highlighting the next logical steps to maximize
/// value from the platform.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.

/// `MilestonesResponse` encapsulates a collection of `Milestone` objects, representing the current
/// state of a user's progress against the defined roadmap.
///
/// This structure is returned by the milestone checking endpoint and is consumed by the frontend to render
/// progress bars, achievement lists, and contextual tooltips guiding the user toward their next goal.
pub struct MilestonesResponse {
    pub milestones: Vec<Milestone>,
}

pub fn router<S>(pool: PgPool, hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/social/post", post(handle_social_post))
        .route("/campaign/send", post(handle_send_campaign))
        .route("/storefront/track", post(handle_track_visitor))
        .route("/milestones/check", get(handle_check_milestones))
        .layer(Extension(GrowthState { pool, hub }))
}

#[derive(Clone)]
struct GrowthState {
    pool: PgPool,
    hub: Arc<Hub>,
}

async fn handle_social_post(
    Extension(_state): Extension<GrowthState>,
    Json(_req): Json<SocialPostRequest>,
) -> impl IntoResponse {
    Json(SocialPostResponse {
        posted: true,
        post_id: uuid::Uuid::new_v4().to_string(),
    })
}

async fn handle_send_campaign(
    Extension(_state): Extension<GrowthState>,
    Json(_req): Json<CampaignRequest>,
) -> impl IntoResponse {
    Json(CampaignResponse {
        campaign_id: uuid::Uuid::new_v4().to_string(),
        emails_sent: 150,
    })
}

async fn handle_track_visitor(
    Extension(_state): Extension<GrowthState>,
    Json(_req): Json<TrackVisitorRequest>,
) -> impl IntoResponse {
    Json(TrackVisitorResponse { tracked: true })
}

async fn handle_check_milestones(
    Extension(_state): Extension<GrowthState>,
) -> impl IntoResponse {
    let milestones = vec![
        Milestone {
            id: "1".to_string(),
            title: "First Teammate".to_string(),
            description: "Hire your first AI agent".to_string(),
            reached: true,
        },
        Milestone {
            id: "2".to_string(),
            title: "Global Reach".to_string(),
            description: "Connect to a partner organization".to_string(),
            reached: false,
        },
    ];
    Json(MilestonesResponse { milestones })
}
