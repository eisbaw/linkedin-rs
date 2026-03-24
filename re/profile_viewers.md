# Profile Viewers (Who Viewed My Profile) Endpoint

## Endpoint

```
GET /voyager/api/identity/wvmpCards
```

**Auth**: Requires `li_at` + `JSESSIONID` (standard session cookies).
**Premium**: Not required for basic viewer data (names, occupations, view count change).

## Request

No parameters required. Simple GET request.

```
GET https://www.linkedin.com/voyager/api/identity/wvmpCards
Csrf-Token: ajax:...
Cookie: li_at=...; JSESSIONID="ajax:..."
```

## Response Structure

The response uses deeply nested Rest.li union encoding. The top-level shape:

```json
{
  "elements": [
    {
      "value": {
        "com.linkedin.voyager.identity.me.wvmpOverview.WvmpViewersCard": {
          "insightCards": [
            {
              "value": {
                "com.linkedin.voyager.identity.me.wvmpOverview.WvmpSummaryInsightCard": {
                  "numViewsChangeInPercentage": 613.0,
                  "cards": [ ... ]
                }
              }
            }
          ]
        }
      }
    }
  ]
}
```

### Union Nesting Levels

1. **Top level**: `elements[].value` -- union key `WvmpViewersCard`
2. **Insight cards**: `insightCards[].value` -- union key `WvmpSummaryInsightCard`
3. **Individual cards**: `cards[].value` -- one of three union types:
   - `com.linkedin.voyager.identity.me.WvmpProfileViewCard` -- named viewer
   - `com.linkedin.voyager.identity.me.WvmpGenericCard` -- aggregated/anonymous
   - `com.linkedin.voyager.identity.me.PrivateProfileViewer` -- private viewer

### WvmpProfileViewCard (Named Viewer)

```json
{
  "com.linkedin.voyager.identity.me.WvmpProfileViewCard": {
    "viewer": {
      "com.linkedin.voyager.identity.me.FullProfileViewer": {
        "profile": {
          "miniProfile": {
            "firstName": "Jane",
            "lastName": "Doe",
            "occupation": "Software Engineer at Example Corp",
            "entityUrn": "urn:li:fs_miniProfile:ACoAABxxxxxxxxx",
            "publicIdentifier": "jane-doe-123"
          }
        }
      }
    }
  }
}
```

### WvmpGenericCard (Aggregated)

```json
{
  "com.linkedin.voyager.identity.me.WvmpGenericCard": {
    "text": "1 person with the job title Recruiter",
    "insightText": "1 person with the job title Recruiter"
  }
}
```

### PrivateProfileViewer

```json
{
  "com.linkedin.voyager.identity.me.PrivateProfileViewer": {
    "headline": "Someone in private mode"
  }
}
```

## Key Fields

| Field | Type | Description |
|-------|------|-------------|
| `numViewsChangeInPercentage` | `f64` | Week-over-week view change percentage |
| `cards` | `array` | Individual viewer entries (union-typed) |
| `miniProfile.firstName` | `string` | Viewer's first name |
| `miniProfile.lastName` | `string` | Viewer's last name |
| `miniProfile.occupation` | `string` | Viewer's headline/occupation |

## Notes

- This is a legacy REST endpoint, not a GraphQL/Dash migration.
- Works without Premium, though Premium may show more viewer details.
- The union nesting is 4 levels deep, typical of LinkedIn's Rest.li encoding.
- Discovered and validated via live API testing (March 2026).
