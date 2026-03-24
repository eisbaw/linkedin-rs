# Create Post (Text Share) - Reverse Engineering Notes

## Endpoint Discovery

### Legacy REST Endpoint (Zephyr + International)

```
POST /voyager/api/contentcreation/normShares
```

- Route constant: `CONTENT_CREATION` in `Routes.java`
- Model: `NormShare`
- Still registered in both builds but the international build has migrated to Dash.

### Dash GraphQL Mutation (International)

```
POST /voyager/api/graphql?action=execute&queryId=voyagerContentcreationDashShares.f8a4f57de961be2d370fbcc862e867cf&queryName=CreateContentcreationDashShares
```

- Route constant: `CONTENT_CREATION_DASH("voyagerContentcreationDashShares")` in `Routes.java`
- Registered in `SharingGraphQLClient.java` as `createContentcreationDashShares`
- Operation type: CREATE (GraphQL mutation)
- Query ID: `voyagerContentcreationDashShares.f8a4f57de961be2d370fbcc862e867cf`

### Other Related Queries in SharingGraphQLClient

| Operation | QueryId | Type |
|---|---|---|
| `contentcreationDashSharesByIds` | `voyagerContentcreationDashShares.b6c8295dc377a63224101ecce6d3c1ca` | GET |
| `createContentcreationDashShares` | `voyagerContentcreationDashShares.f8a4f57de961be2d370fbcc862e867cf` | CREATE |
| `updateContentcreationDashShares` | `voyagerContentcreationDashShares.008717642af22508082fbe2c8faae589` | UPDATE |
| `deleteContentcreationDashShares` | `voyagerContentcreationDashShares.3f82a2271a8c6a93dae7661d0e57ed26` | DELETE |

## Data Models

### ShareData (Input)

From `com.linkedin.android.sharing.compose.dash.ShareData`:

Key fields for a text-only post:
- `shareText` (`TextViewModel`): `{ text: "..." }` - the post body
- `visibilityType` (`VisibilityType` enum): `ANYONE`, `ANYONE_TWITTER`, `CONNECTIONS_ONLY`
- `shareVisibility` (`Integer`): ordinal value, `0` for standard
- `origin` (`Origin` enum): `FEED` for standard compose flow
- `allowedScope` (`AllowedScope`): `NONE` for personal posts (not group/page context)
- `shareMediaForCreate` (`ShareMediaForCreate`): absent for text-only posts

Optional fields (not needed for simple text posts):
- `containerEntityUrn`: for group/page posts
- `parentUrn`: for reshares
- `referenceUrn`: for reference content
- `scheduleAt`: for scheduled posts
- `nonMemberActorUrn`: for posting as a page

### VisibilityType Enum

From `com.linkedin.android.pegasus.dash.gen.voyager.dash.contentcreation.VisibilityType`:

| Value | Description |
|---|---|
| `ANYONE` | Public - visible to all LinkedIn users |
| `ANYONE_TWITTER` | Public + cross-post to Twitter/X |
| `CONNECTIONS_ONLY` | Only visible to connections |

### Share (Response)

From `com.linkedin.android.pegasus.dash.gen.voyager.dash.contentcreation.Share`:

- `entityUrn` (`Urn`): the created share's URN
- `status` (`ShareStatus`): post status
- `scheduledAt` (`Long`): null for immediate posts

## Mutation Variables

The CREATE mutation uses JSON variables (not Rest.li-encoded), following the
`BaseGraphQLClient.generateRequestBuilder()` pattern for mutations.

```json
{
  "queryId": "voyagerContentcreationDashShares.f8a4f57de961be2d370fbcc862e867cf",
  "queryName": "CreateContentcreationDashShares",
  "variables": {
    "entity": {
      "visibilityType": "ANYONE",
      "origin": "FEED",
      "allowedScope": "NONE",
      "shareText": {
        "text": "Hello LinkedIn!"
      },
      "shareVisibility": 0
    }
  }
}
```

## Source Files

- `SharingGraphQLClient.java` - Query ID registration (`classes6.dex`)
- `ShareData.java` - Input model (`classes6.dex`)
- `Share.java` / `ShareBuilder.java` - Response model (`classes7.dex`)
- `VisibilityType.java` - Visibility enum (`classes7.dex`)
- `Origin.java` - Origin enum (`classes7.dex`)
- `ShareComposeNewPostFeature*.java` - UI compose flow (`classes6.dex`)
- `Routes.java` - Route constants including both legacy and Dash paths

## Notes

- The international build exclusively uses the Dash GraphQL mutation for creating posts.
  The legacy `contentcreation/normShares` REST endpoint is still registered in `Routes.java`
  but is not used by the sharing flow.
- The `shareVisibility` field is an integer (enum ordinal). Value `0` corresponds to the
  default/standard visibility setting.
- The `allowedScope` field controls whether the post is in a container (group, page).
  `NONE` means a standard personal post.
- The `origin` field is for analytics/tracking. `FEED` matches the standard compose flow
  that users see when clicking "Start a post" from the feed.
- **NOT VALIDATED AGAINST LIVE API** - The exact payload structure is derived from
  decompiled code analysis. The actual server may require additional fields or different
  field names. Test cautiously.
