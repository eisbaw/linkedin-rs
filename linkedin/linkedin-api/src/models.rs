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
}
