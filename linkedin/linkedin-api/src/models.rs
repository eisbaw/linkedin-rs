//! Data models for LinkedIn API responses.
//!
//! Maps LinkedIn's Rest.li response format into typed Rust structs.
//! These models are intentionally loose (heavy use of `Option<T>` and
//! `Option<Value>`) because we haven't validated them against the live API yet.
//! Fields will be tightened as we confirm the actual response shapes.
//!
//! Reference: `re/pegasus_models.md`, `re/restli_protocol.md` section 7.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Standard Rest.li collection response paging metadata.
///
/// Returned as `paging` in all collection endpoints. See
/// `re/restli_protocol.md` section 7.2.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paging {
    /// 0-based offset of the current page.
    #[serde(default)]
    pub start: u32,

    /// Number of items requested (page size).
    #[serde(default)]
    pub count: u32,

    /// Total number of items available. May be absent if the server
    /// doesn't know or doesn't want to disclose.
    #[serde(default)]
    pub total: Option<u32>,

    /// HATEOAS-style links (rarely used by mobile client).
    #[serde(default)]
    pub links: Option<Vec<Value>>,
}

/// Top-level response from the `feed/updates` endpoint.
///
/// Wraps a standard Rest.li collection of `UpdateV2` items.
/// See `re/restli_protocol.md` section 7.1 for the generic structure
/// and `re/pegasus_models.md` for the `UpdateV2` model.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedResponse {
    /// Array of feed update items. Each element is an `UpdateV2` record,
    /// but we keep it as `Value` until we've validated the full shape.
    #[serde(default)]
    pub elements: Vec<Value>,

    /// Pagination metadata for this page of results.
    #[serde(default)]
    pub paging: Option<Paging>,

    /// Collection-level metadata (type varies by endpoint).
    #[serde(default)]
    pub metadata: Option<Value>,

    /// URN identifying this collection.
    #[serde(default)]
    pub entity_urn: Option<String>,
}

/// Minimal representation of an `UpdateV2` feed item.
///
/// Only the fields we actually display in the CLI are typed; everything
/// else is captured as `Option<Value>` so we don't drop unknown fields.
///
/// Reference: `re/pegasus_models.md` -- `UpdateV2 (voyager.feed.render)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateV2 {
    /// URN identifying this feed update.
    #[serde(default)]
    pub entity_urn: Option<String>,

    /// The actor (author) component of the update.
    #[serde(default)]
    pub actor: Option<Value>,

    /// Post text / commentary.
    #[serde(default)]
    pub commentary: Option<Value>,

    /// Content attachment (article, image, video, etc.).
    #[serde(default)]
    pub content: Option<Value>,

    /// Social engagement metadata (likes, comments, shares).
    #[serde(default)]
    pub social_detail: Option<Value>,

    /// Update metadata (tracking, visibility, etc.).
    #[serde(default)]
    pub update_metadata: Option<Value>,

    /// Contextual header (e.g., "John Doe liked this").
    #[serde(default)]
    pub contextual_header: Option<Value>,

    /// Header component.
    #[serde(default)]
    pub header: Option<Value>,

    /// Reshared update (recursive -- contains another UpdateV2).
    #[serde(default)]
    pub reshared_update: Option<Value>,
}

/// Social engagement metadata for a feed item.
///
/// Reference: `re/pegasus_models.md` -- `SocialDetail (voyager.feed)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialDetail {
    /// URN for the social detail.
    #[serde(default)]
    pub urn: Option<String>,

    /// URN identifying this entity.
    #[serde(default)]
    pub entity_urn: Option<String>,

    /// Aggregated activity counts (likes, comments, shares, views).
    #[serde(default)]
    pub total_social_activity_counts: Option<SocialActivityCounts>,

    /// Whether commenting is disabled.
    #[serde(default)]
    pub commenting_disabled: Option<bool>,

    /// Whether the share button is shown.
    #[serde(default)]
    pub show_share_button: Option<bool>,

    /// Thread identifier.
    #[serde(default)]
    pub thread_id: Option<String>,
}

/// Aggregated social activity counts.
///
/// Reference: `re/pegasus_models.md` -- `SocialActivityCounts (voyager.feed.shared)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialActivityCounts {
    /// Number of likes.
    #[serde(default)]
    pub num_likes: Option<u64>,

    /// Number of comments.
    #[serde(default)]
    pub num_comments: Option<u64>,

    /// Number of shares.
    #[serde(default)]
    pub num_shares: Option<u64>,

    /// Number of views.
    #[serde(default)]
    pub num_views: Option<u64>,

    /// Whether the current user has liked this item.
    #[serde(default)]
    pub liked: Option<bool>,
}

/// Top-level response from the `messaging/conversations` endpoint.
///
/// Wraps a standard Rest.li collection of `Conversation` items.
/// See `re/api_endpoint_catalog.md` section 6 and `re/pegasus_models.md`
/// for the `Conversation` model definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationsResponse {
    /// Array of conversation items.
    #[serde(default)]
    pub elements: Vec<Value>,

    /// Pagination metadata for this page of results.
    #[serde(default)]
    pub paging: Option<Paging>,

    /// Collection-level metadata (type varies by endpoint).
    #[serde(default)]
    pub metadata: Option<Value>,
}

/// A messaging conversation (thread).
///
/// Reference: `re/pegasus_models.md` -- `Conversation (voyager.messaging)`.
/// Fields kept as `Option` since we haven't validated against live API yet.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conversation {
    /// URN identifying this conversation, e.g. `urn:li:messagingThread:...`.
    #[serde(default)]
    pub entity_urn: Option<String>,

    /// Backend URN.
    #[serde(default)]
    pub backend_urn: Option<String>,

    /// Participants in this conversation (union: member/company/bot).
    #[serde(default)]
    pub participants: Option<Vec<Value>>,

    /// Messages/events in this conversation (may be inline or empty).
    #[serde(default)]
    pub events: Option<Vec<Value>>,

    /// Whether the conversation has been read.
    #[serde(default)]
    pub read: Option<bool>,

    /// Whether the conversation is muted.
    #[serde(default)]
    pub muted: Option<bool>,

    /// Whether the conversation is archived.
    #[serde(default)]
    pub archived: Option<bool>,

    /// Whether the conversation is blocked.
    #[serde(default)]
    pub blocked: Option<bool>,

    /// Unread message count.
    #[serde(default)]
    pub unread_count: Option<u32>,

    /// Total number of events in the conversation.
    #[serde(default)]
    pub total_event_count: Option<u32>,

    /// Group chat name (if any).
    #[serde(default)]
    pub name: Option<String>,

    /// Whether this is with a non-connection.
    #[serde(default)]
    pub with_non_connection: Option<bool>,

    /// Last activity timestamp.
    #[serde(default)]
    pub last_activity_at: Option<u64>,

    /// Read receipts.
    #[serde(default)]
    pub receipts: Option<Vec<Value>>,

    /// Notification status.
    #[serde(default)]
    pub notification_status: Option<String>,

    /// Message request state (ACCEPTED, DECLINED, PENDING).
    #[serde(default)]
    pub message_request_state: Option<String>,

    /// Catch-all for fields not explicitly modelled.
    #[serde(flatten)]
    pub extra: Option<std::collections::HashMap<String, Value>>,
}

/// A single messaging event (message, participant change, etc.).
///
/// Reference: `re/pegasus_models.md` -- `Event (voyager.messaging)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagingEvent {
    /// URN identifying this event.
    #[serde(default)]
    pub entity_urn: Option<String>,

    /// Backend URN.
    #[serde(default)]
    pub backend_urn: Option<String>,

    /// Timestamp when the event was created (epoch millis).
    #[serde(default)]
    pub created_at: Option<u64>,

    /// Timestamp when the event expires (epoch millis).
    #[serde(default)]
    pub expires_at: Option<u64>,

    /// The sender of this event (union: MessagingProfile).
    #[serde(default)]
    pub from: Option<Value>,

    /// Event subtype (MEMBER_TO_MEMBER, INMAIL, etc.).
    #[serde(default)]
    pub subtype: Option<String>,

    /// The event content (union: MessageEvent, ParticipantChangeEvent, etc.).
    #[serde(default)]
    pub event_content: Option<Value>,

    /// Quick reply options.
    #[serde(default)]
    pub quick_replies: Option<Vec<Value>>,

    /// URN of the previous event in the conversation.
    #[serde(default)]
    pub previous_event_in_conversation: Option<String>,

    /// Catch-all for fields not explicitly modelled.
    #[serde(flatten)]
    pub extra: Option<std::collections::HashMap<String, Value>>,
}

/// Top-level response from the `messaging/conversations/{id}/events` endpoint.
///
/// Wraps a standard Rest.li collection of `MessagingEvent` items.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationEventsResponse {
    /// Array of event items.
    #[serde(default)]
    pub elements: Vec<Value>,

    /// Pagination metadata.
    #[serde(default)]
    pub paging: Option<Paging>,

    /// Collection-level metadata.
    #[serde(default)]
    pub metadata: Option<Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paging_deserializes_minimal() {
        let json = r#"{"start": 0, "count": 10}"#;
        let paging: Paging = serde_json::from_str(json).unwrap();
        assert_eq!(paging.start, 0);
        assert_eq!(paging.count, 10);
        assert!(paging.total.is_none());
    }

    #[test]
    fn paging_deserializes_full() {
        let json = r#"{"start": 5, "count": 10, "total": 42, "links": []}"#;
        let paging: Paging = serde_json::from_str(json).unwrap();
        assert_eq!(paging.start, 5);
        assert_eq!(paging.count, 10);
        assert_eq!(paging.total, Some(42));
    }

    #[test]
    fn feed_response_deserializes_empty() {
        let json = r#"{"elements": [], "paging": {"start": 0, "count": 10}}"#;
        let resp: FeedResponse = serde_json::from_str(json).unwrap();
        assert!(resp.elements.is_empty());
        assert_eq!(resp.paging.as_ref().unwrap().start, 0);
    }

    #[test]
    fn feed_response_handles_missing_fields() {
        let json = r#"{}"#;
        let resp: FeedResponse = serde_json::from_str(json).unwrap();
        assert!(resp.elements.is_empty());
        assert!(resp.paging.is_none());
        assert!(resp.metadata.is_none());
    }

    #[test]
    fn social_activity_counts_deserializes() {
        let json = r#"{"numLikes": 42, "numComments": 5, "liked": true}"#;
        let counts: SocialActivityCounts = serde_json::from_str(json).unwrap();
        assert_eq!(counts.num_likes, Some(42));
        assert_eq!(counts.num_comments, Some(5));
        assert_eq!(counts.liked, Some(true));
        assert!(counts.num_shares.is_none());
    }

    #[test]
    fn conversation_deserializes_minimal() {
        let json = r#"{}"#;
        let conv: Conversation = serde_json::from_str(json).unwrap();
        assert!(conv.entity_urn.is_none());
        assert!(conv.participants.is_none());
        assert!(conv.read.is_none());
    }

    #[test]
    fn conversation_deserializes_with_fields() {
        let json = r#"{
            "entityUrn": "urn:li:messagingThread:2-abc123",
            "read": true,
            "unreadCount": 0,
            "totalEventCount": 15,
            "name": "Test Group",
            "participants": []
        }"#;
        let conv: Conversation = serde_json::from_str(json).unwrap();
        assert_eq!(
            conv.entity_urn.as_deref(),
            Some("urn:li:messagingThread:2-abc123")
        );
        assert_eq!(conv.read, Some(true));
        assert_eq!(conv.unread_count, Some(0));
        assert_eq!(conv.total_event_count, Some(15));
        assert_eq!(conv.name.as_deref(), Some("Test Group"));
    }

    #[test]
    fn messaging_event_deserializes_minimal() {
        let json = r#"{}"#;
        let event: MessagingEvent = serde_json::from_str(json).unwrap();
        assert!(event.entity_urn.is_none());
        assert!(event.subtype.is_none());
        assert!(event.event_content.is_none());
    }

    #[test]
    fn messaging_event_deserializes_with_fields() {
        let json = r#"{
            "entityUrn": "urn:li:fs_event:abc123",
            "createdAt": 1711234567890,
            "subtype": "MEMBER_TO_MEMBER",
            "eventContent": {
                "com.linkedin.voyager.messaging.event.MessageEvent": {
                    "body": "Hello!"
                }
            }
        }"#;
        let event: MessagingEvent = serde_json::from_str(json).unwrap();
        assert_eq!(event.entity_urn.as_deref(), Some("urn:li:fs_event:abc123"));
        assert_eq!(event.created_at, Some(1711234567890));
        assert_eq!(event.subtype.as_deref(), Some("MEMBER_TO_MEMBER"));
        assert!(event.event_content.is_some());
    }

    #[test]
    fn conversations_response_deserializes_empty() {
        let json = r#"{"elements": [], "paging": {"start": 0, "count": 10}}"#;
        let resp: ConversationsResponse = serde_json::from_str(json).unwrap();
        assert!(resp.elements.is_empty());
        assert_eq!(resp.paging.as_ref().unwrap().start, 0);
    }

    #[test]
    fn conversation_events_response_handles_missing_fields() {
        let json = r#"{}"#;
        let resp: ConversationEventsResponse = serde_json::from_str(json).unwrap();
        assert!(resp.elements.is_empty());
        assert!(resp.paging.is_none());
    }
}
