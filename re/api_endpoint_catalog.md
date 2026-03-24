# LinkedIn API Endpoint Catalog

Extracted from `com.linkedin.android` APK decompiled via jadx. Source of truth: `Routes.java` enum + `*Routes.java` / `*RouteUtils.java` helper classes + `DataProvider` / `Repository` classes for HTTP method inference.

**Base URL**: `https://www.linkedin.com`
**API prefix**: `/voyager/api/` (international) or `/zephyr/api/` (China/Zephyr variant)
**Protocol**: Rest.li 2.0 (`X-RestLi-Protocol-Version: 2.0.0`)
**Field projection**: `n` query parameter (decoration/recipe ID)

All paths below are relative to `{baseUrl}/voyager/api/` unless noted otherwise.

---

## Table of Contents

1. [Configuration and Infrastructure](#1-configuration-and-infrastructure)
2. [Authentication (non-API-prefix)](#2-authentication)
3. [Identity / Profile](#3-identity--profile)
4. [Feed / Content](#4-feed--content)
5. [Content Creation / Publishing](#5-content-creation--publishing)
6. [Messaging](#6-messaging)
7. [Jobs](#7-jobs)
8. [Network / Connections / Relationships](#8-network--connections--relationships)
9. [Search](#9-search)
10. [Typeahead](#10-typeahead)
11. [Notifications](#11-notifications)
12. [Organizations / Companies](#12-organizations--companies)
13. [Premium](#13-premium)
14. [Growth / Onboarding](#14-growth--onboarding)
15. [Groups](#15-groups)
16. [Real-Time](#16-real-time)
17. [Media Upload](#17-media-upload)
18. [Tracking / Analytics](#18-tracking--analytics)
19. [Geographic / Location](#19-geographic--location)
20. [Zephyr-Specific (China)](#20-zephyr-specific-china)

---

## 1. Configuration and Infrastructure

| Route Constant | Path | Method | Query Params | Pegasus Model | Notes |
|---|---|---|---|---|---|
| `CONFIGURATION` | `configuration` | GET | | | App configuration / feature flags |
| `LIX` | `lixTreatments` | GET | | | A/B test treatments (LinkedIn Internal eXperiments) |
| `MUX` | `mux` | POST | | | Request multiplexing (batch multiple requests) |
| `SECURE_MUX` | `mux/secure` | POST | | | Secure request multiplexing |
| `PSETTINGS_GROUP` | `/psettings/group` | GET | | | Privacy settings group (absolute path) |

---

## 2. Authentication

These use the base URL directly (NOT the `/voyager/api/` prefix):

| Path | Method | Content-Type | Parameters | Notes |
|---|---|---|---|---|
| `/uas/authenticate` | GET | | | Obtain CSRF token (JSESSIONID cookie) |
| `/uas/authenticate` | POST | `application/x-www-form-urlencoded` | `session_key`, `session_password`, `googleIdToken`, `flashIdToken`, `appleIdToken`, `Challenge_id`, `rememberMeOptIn`, `client_enabled_features` | Login with credentials |
| `/uas/issueLoginCookie` | POST | | | Get additional session cookies after login |
| `/uas/directLogout` | POST | | | Logout |
| `/oauth/mobilesdk/authorization` | GET | | `response_type`, `client_id`, `packageHash`, `scope`, `redirect_uri`, `code_challenge`, `code_challenge_method`, `state`, `_l` | Third-party OAuth2+PKCE authorization |
| `/uas/mobilesdk/authorize` | POST | `application/x-www-form-urlencoded` | `scope`, `csrfToken`, `packageName`, `packageHash`, `userAuthorized` | Third-party OAuth token grant |
| `/checkpoint/login/fastrackProfileV2` | POST | | | Fast-track profile fetch |

---

## 3. Identity / Profile

### Core Profile

| Route Constant | Path | Method | Query Params | Recipe / Decoration | Pegasus Model |
|---|---|---|---|---|---|
| `ME` | `me` | GET | | | `Me` |
| `SETTINGS` | `me/settings` | GET/POST | `action` (for updates) | | `MySettings` |
| `PROFILE` | `identity/profiles` | GET | | | `Profile` |
| `PROFILE/{memberId}` | `identity/profiles/{memberId}` | GET | | | `Profile` |
| `PROFILE/{id}/profileView` | `identity/profiles/{id}/profileView` | GET | | | `ProfileView` |
| `NORMALIZED_PROFILE` | `identity/normProfiles` | GET/POST | | | `NormProfile` |
| `NORMALIZED_PROFILE/{id}` | `identity/normProfiles/{id}` | POST | | | Profile edit (PATCH via POST) |
| `MINIPROFILE` | `identity/miniprofiles` | GET | | | `MiniProfile` |
| `MINIPROFILE/{id}` | `identity/miniprofiles/{id}` | GET | | | `MiniProfile` |
| `IDENTITY_PROFILES` | `voyagerIdentityProfiles` | GET | | | `Profile` (v2) |
| `IDENTITY_PROFILES/{id}` | `voyagerIdentityProfiles/{id}` | GET | | | `Profile` (v2) |
| `IDENTITY_NORMALIZED_PROFILES` | `identity/normalizedProfiles` | GET | `q=connectionsByJobPosting`, `q=connectionsWorkingAtCompany`, `q=similarEmployeesByCompanyRecommendedJobPostings` | `com.linkedin.voyager.deco.identity.normalizedprofile.shared.ApplicantProfile-13`, `com.linkedin.voyager.deco.identity.normalizedprofile.shared.ListedProfile-6` | `ApplicantProfile`, `ListedProfile` |
| `IDENTITY_PROFILE_ACTIONS_V2` | `voyagerIdentityProfileActionsV2` | GET | | | Profile actions |

### Profile Sub-Resources

| Path Pattern | Method | Purpose | Pegasus Model |
|---|---|---|---|
| `identity/profiles/{id}/positions` | GET | Positions/experience | `Position` |
| `identity/profiles/{id}/educations` | GET | Education entries | `Education` |
| `identity/profiles/{id}/skills` | GET | Skills | `Skill` |
| `identity/profiles/{id}/endorsedSkills` | GET | Endorsed skills | `EndorsedSkill` |
| `identity/profiles/{id}/following` | GET | Followed entities (`q=followedEntities`) | |
| `identity/profiles/{id}/following` | POST | Follow action (`action=follow`) | |
| `identity/profiles/{id}/following` | POST | Unfollow action (`action=unfollow`) | |
| `identity/profiles/{id}/treasuryMedias` | GET | Portfolio media | `TreasuryMedia` |
| `identity/profiles/{id}/treasuryMedias` | POST | Create portfolio media | `TreasuryMedia` |
| `identity/profiles/{id}/treasuryMedias/{mediaId}` | POST | Edit portfolio media | `TreasuryMedia` |
| `identity/profiles/{id}/treasuryMedias/{mediaId}` | DELETE | Delete portfolio media | |
| `identity/profiles/{id}/networkinfo` | GET | Network/connection info | |
| `identity/profiles/{id}/contactInfo` | GET | Contact info | |
| `identity/profiles/{id}/recommendations` | GET | Recommendations received | |
| `identity/profiles/{id}/normRecommendations` | POST | Create recommendation | `NormRecommendation` |
| `identity/profiles/{id}/normRecommendationRequests` | POST | Request recommendation | `NormRecommendationRequest` |
| `identity/profiles/{id}/privacySettings` | POST | Edit privacy settings (PATCH) | |
| `identity/profiles/{id}/disconnect` | POST | Disconnect from user | |
| `identity/profiles/{id}/browseMap` | GET | Browse map | |
| `identity/profiles/{id}/sameNameDirectory` | GET | Same name directory | |
| `identity/profiles/{id}/workWithUs` | GET | Work with us section | |
| `identity/profiles/{id}/skillComparison/{skills}` | POST | Batch skill comparison (`X-RestLi-Method: BATCH_PARTIAL_UPDATE`) | |

### Profile Completeness / Dashboard

| Route Constant | Path | Method | Pegasus Model |
|---|---|---|---|
| `PROFILE_COMPLETENESS` | `identity/profileCompleteness` | GET | `ProfileCompleteness` |
| `DASHBOARD` | `identity/profile/dashboard` | GET | `Dashboard` |
| `PROFILE_COMPLETION_METER` | `voyagerIdentityProfileCompletionMeter` | GET | `ProfileCompletionMeter` |
| `SEARCH_APPEARANCES` | `voyagerIdentitySearchAppearances` | GET | Search appearance analytics |

### Endorsements

| Route Constant | Path | Method | Pegasus Model |
|---|---|---|---|
| `SUGGESTED_ENDORSEMENT` | `identity/suggestedEndorsements` | GET | `SuggestedEndorsement` |
| `identity/suggestedEndorsements` | POST | Accept/reject (`action=acceptAndReject`) | |
| `identity/suggestedEndorsements` | POST | Track impression (`action=impression`) | |
| `identity/profiles/{id}/endorsement` | POST | Create endorsement | |
| `identity/profiles/{id}/endorsement/{eid}` | DELETE | Delete endorsement | |

### Guided Edit

| Route Constant | Path | Method | Pegasus Model |
|---|---|---|---|
| `GUIDED_EDIT_FLOWS` | `identity/ge` | GET/POST | Guided edit flows |
| `GUIDED_EDIT_U_EDIT` | `voyagerUbiquitousEdit` | GET | Ubiquitous edit data |
| `STANDARDIZED_TITLE` | `voyagerIdentityStandardizedTitles` | GET | Standardized job titles |
| `LOCAL_SKILL_EXPERT_SUGGESTIONS` | `voyagerIdentityLocalSkillExpertSuggestions` | GET | Local skill expert suggestions |

### Phone Numbers / Handles

| Route Constant | Path | Method |
|---|---|---|
| `MEMBER_PHONE_NUMBER` | `identity/phoneNumbers` | GET |
| `PHONE_NUMBER_REGISTRATION` | `phoneNumberRegistration` | POST |
| `ZEPHYR_CREATE_PHONE_NUMBER` | `voyagerOnboardingMemberHandles` | POST |

### Marketplace / Mentorship

| Route Constant | Path | Method | Pegasus Model |
|---|---|---|---|
| `IDENTITY_OPPORTUNITY_MARKETPLACE` | `voyagerIdentityMentorshipOpportunities` | GET | `MentorshipOpportunity` |
| `IDENTITY_MARKETPLACE_ROLES` | `voyagerIdentityMarketplaceRoles` | GET | `MarketplaceRole` |
| `IDENTITY_MARKETPLACE_CARDS` | `voyagerIdentityMarketplaceCards` | GET/POST | `MarketplaceCard` |
| `IDENTITY_MARKETPLACE_PREFERENCES_FORM` | `voyagerIdentityMarketplacePreferencesForm` | GET | |
| `IDENTITY_MARKETPLACE_PREFERENCES_FORM_RESPONSE` | `voyagerIdentityMarketplacePreferencesFormResponse` | POST | |
| `MENTORSHIP_CONVERSATIONS` | `voyagerMessagingConversations` | GET | Mentorship conversations |

---

## 4. Feed / Content

### Feed Reading

| Route Constant | Path | Method | Query Params | Pegasus Model |
|---|---|---|---|---|
| `FEED` | `feed/updates` | GET | `q=findFeed`, `numComments`, `numLikes`, `moduleKey`, `start`, `count`, `w`, `h`, `connectionType`, `battery` | `Update` |
| `FEED` | `feed/updates` | GET | `q=highlightedFeed`, `highlightedUpdateUrns=List(...)`, `highlightedUpdateTypes=List(...)` | Highlighted feed |
| `FEED` | `feed/updates` | GET | `q=reshareFeed`, `targetUrn` | Reshare feed |
| `FEED_HITS` | `feed/hits` | GET | | Feed hits |
| `FEED_BADGING` | `feed/badge` | GET | | Feed badge count |
| `FEED_SPONSORED_UPDATES` | `feed/sponsoredUpdates` | GET | | Sponsored content |
| `FEED_TRANSLATION` | `feed/dynamicTranslations` | GET | | Dynamic translations |
| `FEED_TRANSLATION/{urn}` | `feed/dynamicTranslations/{urn}` | GET | | Translation for specific update |
| `FEED_CONTENT_TOPIC_DATA` | `feed/contentTopicData` | GET | | Content topic data |
| `NEW_STORYLINES` | `voyagerNewsStorylines` | GET | | News storylines |
| `FEED_PACKAGE_RECOMMENDATIONS` | `feed/packageRecommendations` | GET | | Package recommendations |
| `FEED_RICH_RECOMMENDATIONS` | `feed/richRecommendedEntities` | GET | | Rich recommended entities |
| `FOLLOW_RECOMMENDATIONS` | `voyagerFeedRichRecommendedEntities` | GET | | Follow recommendations |

### Feed Actions

| Path Pattern | Method | Purpose | Pegasus Model |
|---|---|---|---|
| `feed/likes` | POST | Like a post (with `sponsoredMetadata` record param) | |
| `feed/likes/{likeId}` | DELETE | Unlike a post | |
| `feed/comments` | GET/POST | Get/create comments | `Comment` |
| `feed/social` | GET | Social interactions | `SocialDetail` |
| `feed/updates/{updateId}?action=feedback` | POST | Submit feedback | |
| `feed/updates/{updateId}?action=undoFeedback` | POST | Undo feedback | |
| `feed/updates/{updateId}?action=incorrectlyMentionedInTheNews` | POST | Report wrong mention | |
| `feed/updates?action=negativeFeedback` | POST | Negative feedback | |
| `feed/urlpreview` | GET | URL preview | |
| `feed/gdprConsent?action=consent` | POST | GDPR consent | |
| `feed/leadGenForm?action=submit` | POST | Submit lead gen form | |
| `feed/attachments` | GET | Feed attachments | |

### Feed Topics / Follows

| Route Constant | Path | Method | Query Params |
|---|---|---|---|
| `FEED_TOPICS` | `feed/topics` | GET | `q=blendedTopics`, `count` |
| `FOLLOWS` | `feed/follows` | POST | `action=followByEntityUrn` (follow), `action=unfollowByEntityUrn` (unfollow) |
| `TOPIC` | `voyagerFeedFollows` | GET/POST | Follow entity |
| `FEED_SAVE_ACTION` | `voyagerFeedSaveActions` | POST | `action=save` / `action=unsave` |

### Polls

| Route Constant | Path | Method | Pegasus Model |
|---|---|---|---|
| `FEED_POLL` | `zephyrNormPolls` | GET/POST | `NormPoll` |
| `FEED_POLL_VOTE` | `zephyrPollVotes` | POST | `PollVote` |
| `FEED_POLL_SUMMARY` | `voyagerFeedPollsPollSummary` | GET | `PollSummary` |

---

## 5. Content Creation / Publishing

| Route Constant | Path | Method | Query Params | Pegasus Model |
|---|---|---|---|---|
| `CONTENT_CREATION` | `contentcreation/normShares` | POST | | `NormShare` (create post) |
| `CONTENT_CREATION/{urn}` | `contentcreation/normShares/{urn}` | DELETE | | Delete post |
| `CONTENT_CREATION/{id}` | `contentcreation/normShares/{id}` | POST | | Edit post |
| `CONTENT_CREATION/{id}?action=removeMentions` | `contentcreation/normShares/{id}?action=removeMentions` | POST | | Remove mentions |
| `CONTENT_CREATION/{id}/status` | `contentcreation/normShares/{id}/status` | GET | | Video transcode status |
| `PUBLISHING_ARTICLES` | `publishing/firstPartyArticles` | GET | `q=relatedContent`, `url` | Related articles |
| `PUBLISHING_CONTENT` | `publishing/firstPartyContent` | GET | `q=url`, `url` | First-party content |
| `NORM_FIRST_PARTY_ARTICLE` | `publishing/normFirstPartyArticle` | GET/DELETE | | `NormFirstPartyArticle` |
| `PUBLISHING_FEED_MINI_ARTICLE` | `voyagerFeedMiniArticle` | GET | `q=url`, `url` | `FeedMiniArticle` |
| `MEDIA_ASSET_STATUS` | `contentcreation/mediaAssetStatuses/` | GET | | Media asset status |
| `MEDIA_OVERLAY` | `contentcreation/overlays` | GET | `q=available` | Available media overlays |
| `UPDATE_TARGETINGS` | `contentcreation/updateTargetings` | GET | | Update targeting options |
| `URL_PREVIEW_V2` | `contentcreation/urlPreview` | GET | | URL preview v2 |
| `VIDEO_STORY_ITEMS` | `video/storyItems` | GET | `q=story`, `storyUrn`, `entryPointUrn`, `prefetchedItemUrns=List(...)` | Video story items |
| `VIDEO_STORIES` | `video/stories` | GET | | Video stories |
| `TREASURY_URL_PREVIEW` | `voyagerTreasuryUrlpreview` | GET | | Treasury URL preview |

---

## 6. Messaging

### Conversations

| Route Constant | Path | Method | Query Params | Pegasus Model |
|---|---|---|---|---|
| `MESSAGING_ROOT` | `messaging` | GET | | Messaging root |
| `MESSAGING_ROOT/mailboxUnreadCounts` | `messaging/mailboxUnreadCounts` | GET | | Outer mailbox unread counts |
| `MESSAGING_CONVERSATIONS` | `messaging/conversations` | GET | `createdBefore`, `createdAfter`, `count`, `filters=List(...)`, `folders=List(...)` | `Conversation` |
| `MESSAGING_CONVERSATIONS` | `messaging/conversations` | GET | `q=search`, `keywords`, `count` | Search conversations |
| `MESSAGING_CONVERSATIONS` | `messaging/conversations` | GET | `q=participants`, `recipients=List(...)` | Find conversation by participants |
| `MESSAGING_CONVERSATIONS` | `messaging/conversations` | GET | `q=unrepliedJobOpportunityType` | Unreplied job opportunities |
| `MESSAGING_CONVERSATIONS` | `messaging/conversations` | GET | `q=latestOpportunities` | Latest opportunities |
| `MESSAGING_CONVERSATIONS?action=create` | `messaging/conversations?action=create` | POST | | Create conversation / send message (`EventCreateResponse`) |
| `MESSAGING_CONVERSATIONS?action=batchCreateJobPosterReachOutConversations` | `messaging/conversations?action=batchCreateJobPosterReachOutConversations` | POST | | Batch create conversations |
| `MESSAGING_CONVERSATIONS?action=syncExistence` | `messaging/conversations?action=syncExistence` | POST | | Sync conversation existence |
| `MESSAGING_CONVERSATIONS?action=addViewerThroughAccessCode` | `messaging/conversations?action=addViewerThroughAccessCode` | POST | | Join via access code |
| `MESSAGING_CONVERSATIONS?action=batchCreateJobSeekerApplyJobConversations` | `messaging/conversations?action=batchCreateJobSeekerApplyJobConversations` | POST | | Batch job seeker apply conversations |
| `messaging/conversations/{id}` | `messaging/conversations/{id}` | POST | | Update conversation (mark read, etc.) |
| `messaging/conversations/{id}` | `messaging/conversations/{id}` | DELETE | | Delete conversation |
| `messaging/conversations/{id}?action=changeParticipants` | `messaging/conversations/{id}?action=changeParticipants` | POST | | Add/remove participants |
| `messaging/conversations/{id}/events?action=createCapRestriction` | `messaging/conversations/{id}/events?action=createCapRestriction` | POST | | Create cap restriction event |

### Messaging Support

| Route Constant | Path | Method | Query Params | Pegasus Model |
|---|---|---|---|---|
| `MESSAGING_BADGING` | `messaging/badge` | GET | | Unread count |
| `MESSAGING_INMAIL_CREDITS` | `messaging/credits` | GET | | InMail credits |
| `MESSAGING_TYPEAHEAD` | `messaging/typeahead/hits` | GET | `q=typeaheadKeyword`, `keyword`, `types=List(CONNECTIONS,GROUP_THREADS)` | Recipient typeahead |
| `MESSAGING_PRESENCE_STATUSES` | `messaging/presenceStatuses` | GET | | Online presence status |
| `MESSAGING_SYNC_CONVERSATIONS` | `messaging/sync/conversations` | GET | | Sync conversations |
| `MESSAGING_CONVERSATION_ID` | `messaging/conversationId` | GET | | Get conversation ID by members |
| `MESSAGING_CONVERSATION_ACCESS_CODE_PREVIEWS` | `messaging/conversationAccessCodePreviews` | GET | `q=accessCode`, `accessCode` | Access code preview |
| `MESSAGING_SUGGESTED_RECIPIENTS` | `messaging/peripheral/recipientSuggestions` | GET | | Suggested recipients |
| `MESSAGING_RECONNECT_BANNER` | `messaging/peripheral/reconnectionSuggestions` | GET | | Reconnection suggestions |
| `MESSAGING_SEARCH_HISTORY` | `messaging/peripheral/messagingSearchHistory` | GET/POST | `action=createOrUpdate`, `action=clear` | Search history management |
| `MESSAGING_PROMO` | `messaging/peripheral/promo` | GET | `q=context`, `context` | Messaging promo |
| `MESSAGING_INBOT` | `messaging/bots/inbot` | GET | | InBot |
| `MESSAGING_THIRD_PARTY_MEDIA` | `messaging/thirdPartyMedia` | GET | `q=gifSearch`, `query`, `paginationToken`, `anonymousId` | GIF search (Tenor) |
| `MESSAGING_THIRD_PARTY_MEDIA` | `messaging/thirdPartyMedia` | POST | `action=registerGifShare` | Register GIF share |
| `MESSAGING_ATTACHMENTS` | `voyagerMessagingAttachments` | POST | `action=scan` | Virus scan attachment |
| `MESSAGING_TWILIO_VIDEO_ACCESS` | `messaging/videoPrototype` | GET | | Video access |

---

## 7. Jobs

### Job Listings

| Route Constant | Path | Method | Query Params | Recipe / Decoration | Pegasus Model |
|---|---|---|---|---|---|
| `JOB_POSTINGS` | `jobs/jobPostings` | GET | | `com.linkedin.voyager.deco.jobs.shared.FullJobPosting-71` | `FullJobPosting` |
| `JOB_POSTINGS/{id}` | `jobs/jobPostings/{id}` | GET | | `com.linkedin.voyager.deco.jobs.shared.FullJobPosting-71` | `FullJobPosting` |
| `JOB_POSTINGS/{id}` | `jobs/jobPostings/{id}` | POST | | | Save/unsave job (PATCH) |
| `JOB_POSTINGS` | `jobs/jobPostings` | GET | `q=saved` | `com.linkedin.voyager.deco.jobs.shared.ListedJobPosting-43` | Saved jobs |
| `JOB_POSTINGS` | `jobs/jobPostings` | GET | `q=applied` | `com.linkedin.voyager.deco.jobs.shared.ListedJobPosting-43` | Applied jobs |
| `JOB_POSTINGS` | `jobs/jobPostings` | GET | `q=company`, `company`, `premiumOnly`, `includeAffiliatedCompanies`, `sortBy` | `com.linkedin.voyager.deco.jobs.shared.ListedJobPosting-43` | Jobs at company |
| `JOB_POSTINGS` | `jobs/jobPostings` | GET | `q=profileIdForConsumerJobShares`, `profileId` | `com.linkedin.voyager.deco.jobs.shared.ListedJobPosting-43` | Jobs shared by profile |
| `JOB_POSTINGS?action=batchApplyClick` | `jobs/jobPostings?action=batchApplyClick` | POST | | | Batch apply |
| `JOB_POSTINGS/{id}/companyInsights` | `jobs/jobPostings/{id}/companyInsights` | GET | | `com.linkedin.voyager.deco.jobs.premiuminsights.FullCompanyInsights-11` | Company insights for job |
| `VOYAGER_JOBS_JOB_POSTINGS` | `voyagerJobsJobPostings` | GET | | | Job postings v2 |

### Job Search

| Route Constant | Path | Method | Query Params | Recipe / Decoration |
|---|---|---|---|---|
| `JOB_SEARCH` | `jobs/search` | GET | | | Job search |
| `SEARCH` (job variant) | `search/hits` | GET | `q=jserpAll`, `keywords`, `geoUrn`, `f_I`, `relatedSearchesEnabled` | `com.linkedin.voyager.deco.jobs.search.ListedJobSearchHit-50` |

### Job Applications

| Route Constant | Path | Method | Pegasus Model |
|---|---|---|---|
| `JOB_APPLICATIONS` | `jobs/jobApplications` | GET/POST | `JobApplication` |

### Job Recommendations

| Route Constant | Path | Method | Query Params | Recipe / Decoration | Pegasus Model |
|---|---|---|---|---|---|
| `JOB_RECOMMENDATIONS` | `jobs/jobPostingRecommendations` | GET | `q=jobsAtCompany`, `company`, `topNRequestedFlavors=List(...)`, `topN` | `com.linkedin.voyager.deco.jobs.LCPListedJobPostingRecommendation-13` | `ListedJobPostingRecommendation` |
| `JOB_RECOMMENDATIONS` | `jobs/jobPostingRecommendations` | GET | `start`, `count` | `com.linkedin.voyager.deco.jobs.ListedJobPostingRecommendation-45` | Job recommendations |
| `FLAVORED_JOB_RECOMMENDATIONS` | `voyagerJobsFlavoredJobPostingRecommendations` | GET | | | Flavored recommendations |

### Job Preferences / Profile

| Route Constant | Path | Method | Recipe / Decoration | Pegasus Model |
|---|---|---|---|---|
| `JOB_SEEKER_PREFERENCES` | `jobs/jobSeekerPreferences` | GET | `com.linkedin.voyager.deco.jobs.FullJobSeekerPreferences-32` | `FullJobSeekerPreferences` |
| `JOB_SEEKER_SAVED_ANSWERS` | `jobs/jobSeekerSavedAnswers` | GET | | |
| `JOB_MEMBER_RESUME` | `jobs/resumes` | GET | | Resume data |
| `JOB_APPLICANT_INSIGHTS` | `jobs/applicantInsights` | GET | `com.linkedin.voyager.deco.jobs.premiuminsights.ApplicantRankInsights-5`, `com.linkedin.voyager.deco.jobs.premiuminsights.FullApplicantInsights-6`, `com.linkedin.voyager.deco.jobs.premiuminsights.ApplicantTopSkills-5` | `ApplicantInsights` |
| `JOB_TOP_APPLICANT_JOB` | `jobs/topApplicantJobs` | GET | `sorted`, `count` | `com.linkedin.voyager.deco.jobs.premiuminsights.ListedTopApplicantJobs-39` | Top applicant jobs |

### Job Referrals

| Route Constant | Path | Method | Query Params | Recipe / Decoration |
|---|---|---|---|---|
| `JOB_POSTING_REFERRALS` | `jobs/jobPostingReferrals` | GET | `q=candidate`, `jobPosting`, `referralState` (REFERRED/PENDING) | `com.linkedin.voyager.deco.jobs.JobPostingReferralWithDecoratedEmployee-9` |
| `JOB_POSTING_REFERRALS` | `jobs/jobPostingReferrals` | GET | `q=jobPostingEmployee`, `jobPosting`, `referralStates=List(...)` | `com.linkedin.voyager.deco.jobs.JobPostingReferralWithDecoratedCandidate-10` |

### Job Misc

| Route Constant | Path | Method | Pegasus Model |
|---|---|---|---|
| `JOB_SALARY_STATUS` | `zephyrSalarySubmissionStatus` | GET | Salary submission status |
| `JOB_SALARY_SUBMISSION` | `voyagerSalarySubmission` | POST | Salary submission |
| `JOB_FAST_GROWING_COMPANIES` | `jobs/fastGrowingCompanies` | GET | Fast growing companies |
| `JOB_STATIC_IMAGE_MAP_URLS` | `jobs/staticImageMapUrls` | GET | Static map images for jobs |
| `JOB_COMMUTE_ROUTES` | `jobs/commuteRoutes` | GET | Commute routes |
| `JOB_RELEVANCE_FEEDBACK` | `jobs/entityRelevanceFeedback` | POST | Relevance feedback |
| `JOBS_HOME_TEMPLATES` | `jobs/jobsHomeTemplates` | GET | Jobs home page templates |
| `JOBS_SAVED_SEARCHES` | `voyagerJobsSavedSearches` | GET | Saved job searches |
| `JOBS_SUPERTITLE` | `jobs/superTitles` | GET | Super titles |
| `SAVE_JOB_SEARCHES` | `jobs/savedSearches` | GET/POST/DELETE | Saved job searches CRUD |
| `VOYAGER_JOBS_SKILLS` | `voyagerJobsSkills` | GET | Job-related skills |

---

## 8. Network / Connections / Relationships

### Connections

| Route Constant | Path | Method | Query Params | Pegasus Model |
|---|---|---|---|---|
| `CONNECTIONS` | `relationships/connections` | GET | `start`, `count`, `sortType` (RECENTLY_ADDED) | `Connection` |
| `CONNECTIONS/{id}` | `relationships/connections/{id}` | DELETE | | Remove connection |
| `CONNECTIONS_DASH` | `relationships/dash/connections` | GET | | Connections (dash) |
| `CONNECTIONS_SUMMARY` | `relationships/connectionsSummary` | GET | | `ConnectionsSummary` |
| `CONTACT_SYNC_CONNECTIONS` | `relationships/contactSyncConnections` | GET | | Contact-synced connections |

### Invitations

| Route Constant | Path | Method | Query Params | Pegasus Model |
|---|---|---|---|---|
| `RELATIONSHIPS_INVITATIONS` | `relationships/invitations` | GET | `start`, `count`, `folder` | `Invitation` |
| `RELATIONSHIPS_INVITATIONS/{id}?action=accept` | `relationships/invitations/{id}?action=accept` | POST | | Accept invitation |
| `RELATIONSHIPS_INVITATIONS/{id}?action=ignore` | `relationships/invitations/{id}?action=ignore` | POST | | Ignore invitation |
| `RELATIONSHIPS_INVITATIONS/{id}?action=withdraw` | `relationships/invitations/{id}?action=withdraw` | POST | | Withdraw invitation |
| `RELATIONSHIPS_INVITATIONS/{id}?action=reportSpam` | `relationships/invitations/{id}?action=reportSpam` | POST | | Report spam |
| `RELATIONSHIPS_INVITATIONS?action=batchAccept` | `relationships/invitations?action=batchAccept` | POST | | Batch accept |
| `RELATIONSHIPS_INVITATIONS_SUMMARY` | `relationships/invitationsSummary` | GET | | Invitation summary |
| `RELATIONSHIPS_INVITATIONS_SUMMARY?action=clearUnseenCount` | `relationships/invitationsSummary?action=clearUnseenCount` | POST | | Clear unseen count |
| `RELATIONSHIPS_INVITATION_VIEWS` | `relationships/invitationViews` | GET | `start`, `count`, `q=receivedInvitation` | Invitation views |
| `RELATIONSHIPS_NORM_INVITATIONS` | `relationships/normInvitations` | POST | | Normalized invitations |
| `NORM_INVITATIONS` | `voyagerGrowthNormInvitations` | POST | | Send connection invitation (`NormInvitation`) |
| `NORM_INVITATIONS?action=batchCreate` | `voyagerGrowthNormInvitations?action=batchCreate` | POST | | Batch invite |
| `RELATIONSHIPS_INVITATIONS/{id}/acceptByInvitee` | | POST | | Accept by invitee |

### People You May Know (PYMK)

| Route Constant | Path | Method | Query Params | Pegasus Model |
|---|---|---|---|---|
| `RELATIONSHIPS_PYMKS` | `relationships/pymks` | GET | | `PeopleYouMayKnow` |
| `RELATIONSHIPS_PEOPLE_YOU_MAY_KNOW` | `voyagerRelationshipsPeopleYouMayKnow` | GET/DELETE | | PYMK v2 |
| `RELATIONSHIPS_DISCOVERY` | `voyagerRelationshipsDiscovery` | GET | `start`, `count`, `origin`, `trackingId` | Discovery entities |
| `RELATIONSHIPS_DISCOVERY/{id}` | `voyagerRelationshipsDiscovery/{id}` | DELETE | | Dismiss PYMK |
| `PYMK_BY_PEOPLE_SEARCH` | `zephyrPymkByPeopleSearch` | GET | `q=GetSearchPeopleResponse`, `origin` | PYMK by people search |

### Network Badging / Misc

| Route Constant | Path | Method | Pegasus Model |
|---|---|---|---|
| `RELATIONSHIPS_BADGING` | `relationships/badge` | GET | Network badge count |
| `RELATIONSHIPS_THERMOMETER_CARD` | `relationships/thermometerCard` | GET | Thermometer card |
| `RELATIONSHIPS_MYNETWORK_NOTIFICATIONS` | `voyagerRelationshipsMyNetworkNotifications` | GET/POST | MyNetwork notifications |
| `RELATIONSHIPS_MYNETWORK_NOTIFICATIONS?action=markAllItemsAsSeenByTypesAndTimestamp` | | POST | | Mark items as seen |
| `MY_NETWORK_CONNECTION_INSIGHTS` | `voyagerRelationshipsConnectionInsights` | GET | Connection insights |
| `MY_NETWORK_CONNECTION_SUGGESTIONS` | `voyagerRelationshipsConnectionSuggestions` | GET/POST/DELETE | Connection suggestions |
| `MY_NETWORK_CONNECTION_SUGGESTIONS?action=markAllItemsAsSeen` | | POST | | Mark suggestions seen |
| `RELATIONSHIPS_COHORTS` | `voyagerRelationshipsCohorts` | GET | Relationship cohorts |
| `PROPS` | `relationships/propsV2` | GET | Props (kudos) |

---

## 9. Search

| Route Constant | Path | Method | Query Params | Pegasus Model |
|---|---|---|---|---|
| `SEARCH` | `search/hits` | GET | `q=guided`, `searchId`, `origin`, `timestamp`, `guides=List(v->{type},...)`, `keywords`, `blendedSrpEnabled`, `spellCorrectionEnabled`, `relatedSearchesEnabled` | `SearchHit` |
| `SEARCH_BLENDED` | `search/blended` | GET | | Blended search |
| `SEARCH_HISTORY` | `search/history` | GET/POST | `action=dismiss` (clear), `action=update` (update) | `SearchHistory` |
| `SEARCH_FACET` | `search/facets` | GET | `q=guided`, `guides=List(...)`, `requestedFacets=List(...)`, `keywords` | Search facets |
| `SEARCH_TOPICS` | `search/topics` | GET | `q=blendedStorylines`, `count` | Search topics |
| `SEARCH_QUERIES` | `search/queries` | GET | | Search queries |
| `SEARCH_DEGREES` | `search/degrees` | GET | | Search degrees |
| `SEARCH_ADS` | `search/ads` | GET | | Search ads |
| `SEARCH_FILTERS` | `search/filters` | GET | `q`, `filters=List(...)`, `keywords` | Search filters |
| `SEARCH_JYMBII_ADS` | `search/wwuAds` | GET | | JYMBII ads |
| `SEARCH_SAVED_SEARCH` | `search/savedSearches` | GET/POST/DELETE | | Saved searches |
| `SEARCH_QUERY_SUGGESTION` | `voyagerSearchQueries` | GET | `q=quelp` | Query suggestions |
| `GUIDED_SEARCH_CLUSTER` | `search/cluster` | GET | `q=guided`, `searchId`, `origin`, `timestamp`, `guides=List(...)`, `keywords` | Cluster search |
| `CAMPAIGN_STORY` | `search/campaignStory` | GET | | Campaign story |

---

## 10. Typeahead

| Route Constant | Path | Method | Query Params | Pegasus Model |
|---|---|---|---|---|
| `TYPEAHEAD` | `typeahead/hits` | GET | `q=federated`, `query`, `origin`, `shouldUseSchoolParams`, `types=List(...)` | `TypeaheadHit` |
| `TYPEAHEAD` | `typeahead/hits` | GET | `q=blended`, `query`, `id` | Blended typeahead |
| `TYPEAHEAD` | `typeahead/hits` | GET | `q=blendedJobs`, `query`, `id` | Job-specific typeahead |
| `TYPEAHEAD` | `typeahead/hits` | GET | `q=mentions`, `query`, `origin` | Mentions typeahead |
| `TYPEAHEAD` | `typeahead/hits` | GET | `q=groupMembers`, `query`, `groupId` | Group members typeahead |
| `TYPEAHEAD` | `typeahead/hits` | GET | `q=recentlyMentioned`, `types=List(...)` | Recent mentions |
| `TYPEAHEAD` | `typeahead/hits` | GET | `q=jobs`, `query`, `origin` | Jobs typeahead |
| `TYPEAHEADV2` | `typeahead/hitsV2` | GET | `q=blended`, `keywords`, `id` | Blended typeahead v2 |
| `TYPEAHEADV2` | `typeahead/hitsV2` | GET | `q=blendedJobs`, `keywords` | Jobs typeahead v2 |
| `TYPEAHEADV2` | `typeahead/hitsV2` | GET | `q=hashtags`, `prefix`, `commentary`, `urns=List(...)` | Hashtag typeahead |
| `TYPEAHEADV2` | `typeahead/hitsV2` | GET | `q=bingGeo`, `keywords`, `id`, `types=ADDRESS` | Bing geo typeahead |
| `TYPEAHEAD_HITS` | `voyagerTypeaheadHits` | GET | | Typeahead hits v3 |

---

## 11. Notifications

| Route Constant | Path | Method | Query Params | Pegasus Model |
|---|---|---|---|---|
| `NOTIFICATION_CARDS` | `identity/notificationCards` | GET | `start`, `count` | `NotificationCard` |
| `NOTIFICATION_CARDS?action=add` | `identity/notificationCards?action=add` | POST | | Add notification action |
| `NOTIFICATION_CARDS?action=remove` | `identity/notificationCards?action=remove` | POST | | Remove notification action |
| `NOTIFICATION_SEGMENTS` | `identity/notificationSegments` | GET | | Notification segments |
| `NOTIFICATION_SETTINGS` | `identity/notificationSettings` | GET | | `NotificationSetting` |
| `NOTIFICATION_SETTINGS/{type}` | `identity/notificationSettings/{type}` | POST | | Update notification setting (PATCH) |
| `NOTIFICATION_SETTINGS?action={action}` | `identity/notificationSettings?action={action}` | POST | | Notification setting action |
| `NOTIFICATION_GROUPS` | `identity/notificationSettingGroups` | GET | | Notification setting groups |
| `NOTIFICATION_APPRECIATION_TEMPLATE` | `identity/appreciationTemplate` | GET | | Appreciation template |
| `NOTIFICATION_APPRECIATION_CREATE` | `identity/appreciation` | POST | | Create appreciation |

---

## 12. Organizations / Companies

| Route Constant | Path | Method | Query Params | Recipe / Decoration | Pegasus Model |
|---|---|---|---|---|---|
| `COMPANY` | `entities/companies` | GET | | | Legacy company |
| `COMPANY_VIEW` | `entities/companiesView` | GET | | | Company view |
| `COMPANY_DECO` | `organization/companies` | GET | | | Company (decorated) |
| `SCHOOL_DECO` | `organization/schools` | GET | | | School (decorated) |
| `COMPANY_LANDING_PAGE` | `organization/landingPageContents` | GET | | | Landing page |
| `COMPANY_CAREER_PAGE_SETTINGS` | `organization/careerPageSettings` | GET | | | Career page settings |
| `COMPANY_TARGETED_CONTENT` | `organization/targetedContents` | GET | | `com.linkedin.voyager.deco.organization.shared.FullTargetedContent-16` | Targeted content |
| `COMPANY_CULTURAL_INSIGHTS` | `organization/culturalInsights` | GET | | `com.linkedin.voyager.deco.organization.shared.EmployeeCulturalInsights-18`, `com.linkedin.voyager.deco.organization.shared.EmployeeCareerInsights-5` | Cultural/career insights |
| `COMPANY_PREMIUM_INSIGHTS` | `organization/premiumInsights` | GET | | | Premium company insights |
| `COMPANY_RECOMMENDATIONS` | `organization/companyRecommendations` | GET | | | Company recommendations |
| `COMPANY_NOTIFICATION_CARDS` | `organization/notificationCards` | GET | | | Company notifications |
| `ORGANIZATION_ADMIN_UPDATES` | `organization/adminUpdates` | GET | `q=adminFeed`, `organization` | `com.linkedin.voyager.deco.organization.shared.OrganizationAdminUpdateCard-10` | Admin feed updates |
| `ORGANIZATION_COMPANIES` | `voyagerOrganizationCompanies` | GET | `q=similarCompanies`, `company` | `com.linkedin.voyager.deco.organization.shared.CompactCompany-5` | Similar companies |
| `ORGANIZATION_SCHOOL_V2` | `voyagerOrganizationSchoolsV2` | GET | | `com.linkedin.voyager.deco.organization.shared.FullSchoolV2-19` | School v2 |
| `COMPANY_BASIC_INFO` | `voyagerEntitiesCompanies` | GET | | | Basic company info |

### Company Reviews (Zephyr)

| Route Constant | Path | Method |
|---|---|---|
| `COMPANY_REVIEW` | `companyReviews` | GET/POST/DELETE |
| `MINI_COMPANY_REVIEW` | `miniCompanyReviews` | GET |
| `COMPANY_REVIEW_REVIEWED_COMPANIES` | `zephyrReviewedCompanies` | GET |
| `COMPANY_REVIEW_TAGS` | `zephyrReviewedTags` | GET |
| `COMPANY_REFLECTION` | `zephyrQuestions` | GET |
| `COMPANY_REFLECTION_ALL_ANSWER` | `zephyrCompanyReviews` | GET |

---

## 13. Premium

| Route Constant | Path | Method | Pegasus Model |
|---|---|---|---|
| `PREMIUM_PRODUCTS` | `premium/products` | GET | Premium products |
| `SUBSCRIPTION_CART` | `premium/cart` | GET | Subscription cart |
| `PREMIUM_ONBOARDING` | `premium/onboarding` | GET | Onboarding flow |
| `PREMIUM_FEATURE_ACCESS` | `premium/featureAccess` | GET | Feature access levels |
| `PREMIUM_MY_PREMIUM` | `premium/myPremium` | GET | My premium info |
| `PREMIUM_EXPLORE_PREMIUM` | `premium/explorePremium` | GET | Explore premium |
| `PREMIUM_INSIGHTS` | `premium/premiumInsights` | GET | Premium insights |
| `PREMIUM_FREEMIUM_ELIGIBILITY` | `premium/timeBasedFreeFeatureAccessEligibility` | GET | Free trial eligibility |
| `PREMIUM_LEARNING` | `learning/recommendations` | GET | Learning recommendations |
| `PREMIUM_WELCOME_FLOW_CARDS` | `premium/welcomeFlowCards` | GET | Welcome flow |
| `PREMIUM_PROFINDER_PROMO` | `premium/profinderPromo` | GET | ProFinder promo |
| `PREMIUM_PROFINDER_QUESTIONNAIRE` | `premium/profinderQuestionnaires` | GET | ProFinder questionnaires |
| `PREMIUM_PROFINDER_REQUEST_FOR_PROPOSALS` | `premium/requestsForProposals` | GET | Request for proposals |
| `PREMIUM_PROFINDER_RELATEDSERVICES` | `premium/relatedProfinderServices` | GET | Related services |
| `BOOST` | `premium/boost` | GET | Boost |

---

## 14. Growth / Onboarding

| Route Constant | Path | Method | Notes |
|---|---|---|---|
| `ABI_FLOW` | `growth/pageContent/voyager_abi_flow` | GET | Address book import flow |
| `ABI_PAST_IMPORTED_CONTACTS_FLOW` | `growth/pageContent/voyager_abi_past_imported_contacts_flow` | GET | Past imported contacts |
| `ABI_AUTOSYNC_TOAST` | `growth/pageContent/voyager_abi_autosync_toast` | GET | Auto-sync toast |
| `ABI_CONTACTS_FILTERING` | `growth/contactsFiltering` | GET | Contacts filtering |
| `ONBOARDING_FLOW` | `growth/pageContent/onboarding-voyager` | GET | Onboarding flow |
| `ONBOARDING_STEP_FLOW` | `voyagerOnboardingOnboardingStep` | GET | Onboarding steps |
| `ONBOARDING_LAUNCH` | `growth/onboardingLaunch` | GET | Onboarding launch |
| `NEW_TO_VOYAGER` | `growth/pageContent/new-to-voyager` | GET | New to voyager |
| `BABY_CARROT` | `growth/pageContent/baby-carrot` | GET | Baby carrot onboarding |
| `UNDERAGE_CHECK` | `growth/underage` | GET | Underage check |
| `CHANNELS` | `growth/channels` | GET | Growth channels |
| `EMAIL` | `growth/emailConfirmationTask` | GET | Email confirmation |
| `GOAL_TYPE_RECOMMENDATIONS` | `growth/goalTypeRecommendations` | GET | Goal type recs |
| `GOALS` | `growth/goals` | GET | Goals |
| `GROWTH_PYMK` | `growth/pymk` | GET | Growth PYMK |
| `CONTACTS` | `growth/contacts` | POST | Contacts upload |
| `SUGGESTED_ROUTES` | `growth/suggestedRoutes` | GET | Suggested routes |
| `GROWTH_SUGGESTED_CONTACTS_GROUP` | `growth/suggestedContactsGroups` | GET | Suggested contacts groups |
| `INLAY` | `growth/inLay` | GET | InLay |
| `GROWTH_PAGE_CONTENT` | `voyagerGrowthPageContent` | GET | Page content (Lego) |
| `GROWTH_ONBOARDING_HANDLES` | `voyagerOnboardingMemberHandles` | GET/POST | Onboarding member handles |
| `LAUNCHPAD_CARD` | `voyagerOnboardingLaunchpadCard` | GET | Launchpad card |
| `INTERACTIVE_CAMPAIGN_HIGHLIGHTS` | `voyagerOnboardingInteractiveCampaignHighlights` | GET | Campaign highlights |
| `SYNC_CALENDAR` | `voyagerGrowthCalendar` | POST | Calendar sync (`CalendarUploadRequest`) |
| `SAME_NAME_DIRECTORY` | `voyagerGrowthSameNames` | GET | Same name directory |
| `APP_LAUNCHER` | `appUniverse` | GET | App launcher |
| `TAKEOVERS` | `takeovers` | GET | Takeovers |

---

## 15. Groups

| Route Constant | Path | Method | Pegasus Model |
|---|---|---|---|
| `GROUP` | `entities/groups` | GET | Legacy group |
| `GROUPS` | `groups/groups` | GET | Groups |
| `GROUPS_UPDATES` | `groups/updates` | GET | Group updates |
| `GROUP_MEMBERSHIPS` | `groups/groupMemberships` | GET | Group memberships |

---

## 16. Real-Time

These use the base URL directly with `/realtime/` prefix (NOT the `/voyager/api/` prefix):

| Path | Method | Query Params | Notes |
|---|---|---|---|
| `/realtime/connect` | GET (long-poll) | | Establish real-time connection |
| `/realtime/realtimeFrontendSubscriptions` | GET | `ids=List(...)` | Manage subscriptions |
| `/realtime/realtimeFrontendTimestamp` | GET | | Server timestamp |
| `/realtime/realtimeFrontendClientConnectivityTracking?action=sendHeartbeat` | POST | | Send heartbeat |

Topic format: `(topic:{urn},clientConnectionId:{connectionId})`

---

## 17. Media Upload

| Route Constant | Path | Method | Notes |
|---|---|---|---|
| `FILE_UPLOAD_TOKEN` | `fileUploadToken` | GET | File upload token |
| `AMBRY_UPLOAD_URLS` | `voyagerAmbryUploadUrls` | GET | Ambry upload URLs |
| `MUPLD` | `mupld/upload` | POST | Media upload |
| `MUPLD_ATTACHMENT` | `mupld/attachment` | POST | Attachment upload |
| `MUPLD_JOB_APPLICATION_RESUME` | `mupld/cappts` | POST | Resume upload (job app) |
| `MEDIA_UPLOAD_METADATA` | `voyagerMediaUploadMetadata` | POST | Upload metadata (init + complete) |

The vector media uploader uses two action routes:
- `voyagerMediaUploadMetadata?action=upload` -- initiate upload
- `voyagerMediaUploadMetadata?action=completeUpload` -- complete upload

---

## 18. Tracking / Analytics

| Route Constant | Path | Method | Notes |
|---|---|---|---|
| `LEGO_PAGE_IMPRESSION` | `legoPageImpressionEvents` | POST | Page impression tracking |
| `LEGO_WIDGET_IMPRESSION` | `legoWidgetImpressionEvents` | POST | Widget impression tracking |
| `LEGO_WIDGET_ACTION` | `legoWidgetActionEvents` | POST | Widget action tracking |
| `PUSH_REGISTRATION` | `pushRegistration` | POST | Push notification registration |

### Me Tab Analytics

| Route Constant | Path | Method |
|---|---|---|
| `ME_FEED_BADGING` | `identity/badge` | GET |
| `ME_FEED_CARDS` | `identity/cards` | GET |
| `ME_FEED_PANELS` | `identity/panels` | GET |
| `ME_FEED_HEADER` | `identity/header` | GET |
| `ME_WVMP_CARDS` | `identity/wvmpCards` | GET |
| `ME_CONTENT_ANALYTICS_HEADER` | `identity/socialUpdateAnalyticsHeader` | GET |
| `ME_CONTENT_ANALYTICS_HIGHLIGHTS` | `identity/socialUpdateAnalytics` | GET |
| `COMMUNICATIONS_TAB_BADGES` | `voyagerCommunicationsTabBadges` | GET |

---

## 19. Geographic / Location

| Route Constant | Path | Method | Query Params |
|---|---|---|---|
| `STATE` | `states` | GET | `q=findStates`, `countryCode` |
| `CITY` | `cities` | GET | `q=findCitiesByStateCode`, `countryCode`, `stateCode` |
| `CITY` | `cities` | GET | `q=findCityByPostalCode`, `countryCode`, `postalCode` |
| `REGION` | `regions` | GET | `q=findRegionByPostalCode`, `countryCode`, `postalCode` |
| `COUNTRY` | `countries` | GET | |
| `INDUSTRY` | `industries` | GET | |
| `INDUSTRY_CATEGORY` | `industryCategory` | GET | |
| `COMPANY_SIZE` | `companySizes` | GET | |
| `SENIORITY` | `seniorityLevels` | GET | |
| `VOYAGERGEO` | `voyagerGeo` | GET | `q=findLocations`, `countryGeoUrn`, `postalCode` |
| `BASIC_LOCATION` | `voyagerGrowthBasicLocations` | GET | |
| `SIMPLIFIED_TOPICS` | `simplifiedTopics` | GET | |

---

## 20. Zephyr-Specific (China)

These endpoints are specific to the Chinese (Zephyr) variant. Many are also accessible via the international variant.

### Q&A / Knowledge

| Route Constant | Path | Method |
|---|---|---|
| `ZEPHYR_QUESTION` | `normQuestion` | POST |
| `ZEPHYR_ANSWER` | `normAnswer` | POST |
| `QUESTION_DETAIL_PATH` | `zephyrRichQuestion` | GET |
| `ANSWER_DETAIL_PATH` | `zephyrRichAnswer` | GET |
| `ANSWER_LIST_PATH` | `zephyrSimplifiedAnswers` | GET |
| `MY_ANSWERS_PATH` | `zephyrMyAnswers` | GET |
| `MY_QUESTIONS_PATH` | `zephyrMyQuestions` | GET |
| `PREVIEW_QUESTION_ANSWER` | `previewQuestionAndAnswer` | GET |
| `ZEPHYR_FEED_TOP_ANSWERERS` | `zephyrFeedTopAnswerers` | GET |

### Career / Insights

| Route Constant | Path | Method |
|---|---|---|
| `ZEPHYR_CAREER_INSIGHT` | `careerInsights` | GET |
| `ZEPHYR_CAREER_MINI_INSIGHT` | `miniCareerInsights` | GET |
| `ZEPHYR_CAREER_INSIGHT_COMPANY` | `zephyrCareerInsightCompanies` | GET |
| `ZEPHYR_VOTES` | `zephyrVotes` | POST (with `action=voteForOption`) |
| `ZEPHYR_VOTES_COMMENTS` | `zephyrVoteComments` | GET |
| `ZEPHYR_VOTES_ALLOWLIST` | `careerInsightWhitelists` | GET |
| `CAREER_PATH_OCCUPATIONS` | `zephyrCareerPaths` | GET |
| `CAREER_PATH_VIEW` | `zephyrCareerPathViews` | GET |
| `ZEPHYR_STABLENESS_INSIGHTS` | `zephyrStablenessInsights` | GET |
| `ZEPHYR_CAREER_HOME_RECOMMEND` | `zephyrCareerHomeRecommend` | GET |
| `ZEPHYR_CAREER_DAILY_QUESTIONS` | `zephyrCareerDailyQuestions` | GET |

### Zephyr Jobs

| Route Constant | Path | Method |
|---|---|---|
| `ZEPHYR_JOB_POSTINGS` | `zephyrJobPostings` | GET/POST |
| `ZEPHYR_MINI_JOB_POSTINGS` | `zephyrMiniJobPostings` | GET |
| `ZEPHYR_MINI_COMPANIES` | `zephyrMiniCompanies` | GET |
| `ZEPHYR_MINI_JOBS` | `zephyrMiniJobs` | GET |
| `ZEPHYR_JOB_CANDIDATE_RECOMMENDATIONS` | `zephyrJobCandidateRecommendations` | GET |
| `ZEPHYR_JOB_COHORTS` | `zephyrJobCohorts` | GET |
| `ZEPHYR_JOB_REFERRER_ENTITIES` | `zephyrJobReferrerEntities` | GET |
| `ZEPHYR_JOB_CANDIDATE_REFERRALS` | `zephyrJobCandidateReferrals` | GET |
| `ZEPHYR_JOBS_JOB_POSTER_RESPONSIVENESS` | `jobPosterResponsivenessBadge` | GET |
| `ZEPHYR_JOBS_JOB_POSTER_SETTING` | `jobPosterBadgeSetting` | GET/POST |
| `ZEPHYR_JOB_POSTER_COMPLIANCE` | `zephyrJobPosterCompliance` | GET |
| `ZEPHYR_JOB_NOTIFICATION` | `zephyrJobNotifications` | GET/POST |
| `ZEPHYR_JOB_TAB_BADGE` | `zephyrJobTabBadges` | GET/POST |
| `ZEPHYR_JOB_DESCRIPTION_RECOMMENDATIONS` | `zephyrJobDescriptionRecommendations` | GET |
| `ZEPHYR_JOB_ALERT_CREATE_RECOMMENDATIONS` | `zephyrJobAlertCreateRecommendations` | GET |
| `ZEPHYR_SALARY_INSIGHTS_JOB_POSTING` | `zephyrSalaryInsightJobPosting` | GET |
| `ZEPHYR_JOB_SOCIAL_HIRING_JOB_SEEKER_CARDS` | `zephyrJobSocialHiringJobSeekerCards` | GET |
| `ZEPHYR_JOB_SOCIAL_HIRING_REFERRERS` | `zephyrJobSocialHiringReferrers` | GET |
| `ZEPHYR_JOB_SOCIAL_HIRING_JOB_SHARE_HISTORIES` | `zephyrJobSocialHiringJobShareHistories` | GET/POST |
| `ZEPHYR_JOB_SOCIAL_HIRING_JOB_SEEKER_REQUEST_CARDS` | `zephyrJobSocialHiringJobSeekerRequestCards` | GET |
| `ZEPHYR_JOB_SOCIAL_HIRING_JOB_SEEKER_AUTO_PILOT_PREFERENCES` | `zephyrJobSocialHiringJobSeekerAutoPilotPreferences` | GET/POST |

### Zephyr Misc

| Route Constant | Path | Method |
|---|---|---|
| `ZEPHYR_HASHTAGS` | `zephyrHashtags` | GET (`q=allTopic`, `q=prefixMatch`, `q=segment`, `q=search`) |
| `ZEPHYR_HASHTAG_BOX` | `zephyrHashtagBox` | GET |
| `ZEPHYR_NEARBY_PEOPLE` | `relationships/nearbyPeople` | GET |
| `NEARBY_PEOPLE_V2` | `zephyrNearbyV2` | GET |
| `ZEPHYR_NEARBY_PEOPLE_ENTRY_STATUS` | `zephyrNearbyPeopleEntryStatus` | GET |
| `ZEPHYR_BIZ_CARDS` | `zephyrGrowthBizCard` | GET/POST/DELETE |
| `ZEPHYR_VIDEO_ASSETS` | `zephyrVideoAssets` | GET |
| `ZEPHYR_HOVR_TOKEN` | `zephyrHovrToken` | GET |
| `NYMK` | `zephyrNymk` | GET |
| `ZEPHYR_ONBOARDING_TOPIC_LIST` | `zephyrFollow` | GET |
| `ZEPHYR_PROFILE_COMPLETENESS` | `zephyrProfileCompletenessResource` | GET |
| `ZEPHYR_ANDROID_HOTFIX_PATCH` | `zephyrDefaultAndroidHotFixPatch` | GET |
| `ZEPHYR_FEED_LINKEDIN_SELECTED_SLOTS` | `zephyrFeedLinkedInSelectedSlots` | GET |
| `FEED_CAMPAIGN_SEARCH` | `zephyrSearchHotWords` | GET |
| `SESAME_CREDIT` | `zephyrSesameCredit` | GET |
| `INTERNAL_PROMOTION` | `internalPromotions` | GET |
| `CAREER_CONVERSATION_PREFERENCE` | `zephyrCareerConversationPreference` | GET |
| `REGISTRATION_SOURCE` | `zephyrRegistrationSource` | GET |
| `POSITION_PRIVACY` | `zephyrBarcodePrivacySettings` | GET/POST |
| `MARS_V2` | `zephyrMarsCampaign` | GET |
| `CHINA_BLOCKED_URLS` | `chinaBlockedUrls` | GET |
| `GROWTH_WECHAT_INVITE_INFO` | `zephyrGrowthWeChatInviteInfo` | GET |
| `GROWTH_TENCENT_MEETING` | `zephyrGrowthTencentMeetings` | GET/POST |
| `WECHAT_BIND_NOTIFICATION_ROUTE` | `zephyrGrowthWechatAccountInfo/LINKEDIN_OFFICIAL_ACCOUNT` | GET |
| `GROWTH_BENEFITS` | `zephyrGrowthLightPremiumEntitlements` | GET |

### Zephyr Coach / Mentorship

| Route Constant | Path |
|---|---|
| `ZEPHYR_COACH_CAMPAIGN` | `zephyrCoachCampaign` |
| `ZEPHYR_COACH_CAMPAIGN_CONNECTED_MENTEES` | `zephyrCoachCampaignConnectedMentees` |
| `ZEPHYR_COACH_CAMPAIGN_MENTEES` | `zephyrCoachCampaignMentees` |
| `ZEPHYR_COACH_CAMPAIGN_CONNECTED_MENTORS` | `zephyrCoachCampaignConnectedMentors` |
| `ZEPHYR_COACH_CAMPAIGN_RECOMMENDED_MENTORS` | `zephyrCoachCampaignRecommendedMentors` |
| `ZEPHYR_COACH_CAMPAIGN_MENTOR_BADGE` | `zephyrCoachCampaignMentorBadge` |
| `ZEPHYR_CAREER_COACH_CAMPAIGN_CONSULTED_MENTEE` | `zephyrCareerCoachCampaignConsultedMentee` |
| `ZEPHYR_CAREER_COACH_CAMPAIGN_CONSULTED_MENTOR` | `zephyrCareerCoachCampaignConsultedMentor` |

### Zephyr Learning

| Route Constant | Path |
|---|---|
| `LEARNING_FEATURED_COURSES` | `zephyrLearningFeaturedCourses` |
| `LEARNING_CAREER_PATH_COLLECTION` | `zephyrLearningCareerPathCollection` |
| `LEARNING_AGGREGATED_MINI_COURSES` | `zephyrLearningAggregatedMiniCourses` |
| `LEARNING_CAREER_PATH_COURSE_COLLECTION` | `zephyrLearningCareerPathCourseCollection` |
| `LEARNING_MINI_COURSES` | `zephyrLearningMiniCourses` |
| `LEARNING_COURSE_DETAIL` | `zephyrLearningCourses` |

### Zephyr Scholarship

| Route Constant | Path |
|---|---|
| `SCHOLARSHIP_SIGNUP` | `zephyrMiniProgramScholarshipCampaignSignup` |
| `SCHOLARSHIP_REWARD` | `zephyrMiniProgramScholarshipCampaignReferralReward` |
| `SCHOLARSHIP_CAMPAIGN_PROFILE` | `zephyrMiniProgramScholarshipCampaignProfile` |
| `SCHOLARSHIP_CAMPAIGN_EDITOR_PICK_PROFILES` | `zephyrMiniProgramScholarshipCampaignEditorPickProfiles` |
| `SCHOLARSHIP_CAMPAIGN_TASK_PROGRESS` | `zephyrMiniProgramScholarshipCampaignTaskProgresses` |
| `SCHOLARSHIP_RANK` | `zephyrMiniProgramScholarshipCampaignRanking` |
| `SCHOLARSHIP_DAILY_RANK` | `zephyrMiniProgramScholarshipCampaignApplicantCompetitivenessDailyReport` |
| `SCHOLARSHIP_EXAM_RESULT` | `zephyrMiniProgramScholarshipCampaignApplicantCompetitivenessReport` |

### Zephyr Rewards / Coupons

| Route Constant | Path |
|---|---|
| `REWARDS` | `entities/rewards` |
| `JOB_SEEKING_GIFT` | `zephyrGrowthRegistrationRewardInfo` |
| `ZEOHYR_LINKEDIN_REWARD_MISSIONS` | `zephyrMissions` |
| `ZEPHYR_COUPONS_LIST` | `zephyrGrowthMiniCoupons` |
| `ZEPHYR_COUPONS_DETAIL` | `zephyrGrowthCoupon` |
| `ZEPHYR_GEO_GROUPS` | `zephyrGeoGroups` |
| `ZEPHYR_PROFILE_CHANGE_LOCATION_PROMPT` | `zephyrGrowthChangeLocations` |
| `ZEPHYR_MINI_PROGRAM_QUESTION_INVITEES` | `zephyrMiniProgramQuestionInvitees` |
| `CAMPUS_RECRUITING_PROMO` | `zephyrCampusRecruits` |
| `PROFILE_RECOMMEND_SKILL` | `identity/zephyrRecommendedSkills` |
| `PROFILE_MISSING_QP_COMPONENTS` | `identity/zephyrProfileReward` |

---

## Appendix A: Non-API-Prefix Endpoints

Some endpoints don't use the `/voyager/api/` prefix:

| Path | Method | Notes |
|---|---|---|
| `/uas/authenticate` | GET/POST | Auth |
| `/uas/issueLoginCookie` | POST | Session cookies |
| `/uas/directLogout` | POST | Logout |
| `/oauth/mobilesdk/authorization` | GET | OAuth2+PKCE |
| `/uas/mobilesdk/authorize` | POST | OAuth grant |
| `/checkpoint/login/fastrackProfileV2` | POST | Fast-track login |
| `/realtime/connect` | GET (long-poll) | Real-time |
| `/realtime/realtimeFrontendSubscriptions` | GET | Subscriptions |
| `/realtime/realtimeFrontendTimestamp` | GET | Timestamp |
| `/realtime/realtimeFrontendClientConnectivityTracking` | POST | Heartbeat |
| `/jobposting/api/jobPostingActivityLogs` | POST | Job posting activity (different prefix) |
| `/psettings/group` | GET | Privacy settings |

## Appendix B: Key Decoration/Recipe IDs

These are field projection identifiers passed via the `n` query parameter:

| Recipe ID | Domain | Notes |
|---|---|---|
| `com.linkedin.voyager.deco.jobs.shared.FullJobPosting-71` | Jobs | Full job posting detail |
| `com.linkedin.voyager.deco.jobs.shared.ListedJobPosting-43` | Jobs | Listed job (compact) |
| `com.linkedin.voyager.deco.jobs.search.ListedJobSearchHit-50` | Jobs | Job search result |
| `com.linkedin.voyager.deco.jobs.ListedJobPostingRecommendation-45` | Jobs | Job recommendation |
| `com.linkedin.voyager.deco.jobs.LCPListedJobPostingRecommendation-13` | Jobs | LCP job recommendation |
| `com.linkedin.voyager.deco.jobs.premiuminsights.ListedTopApplicantJobs-39` | Jobs | Top applicant jobs |
| `com.linkedin.voyager.deco.jobs.premiuminsights.ApplicantRankInsights-5` | Jobs | Applicant rank |
| `com.linkedin.voyager.deco.jobs.premiuminsights.FullApplicantInsights-6` | Jobs | Full applicant insights |
| `com.linkedin.voyager.deco.jobs.premiuminsights.ApplicantTopSkills-5` | Jobs | Top skills insights |
| `com.linkedin.voyager.deco.jobs.premiuminsights.FullCompanyInsights-11` | Jobs | Company insights |
| `com.linkedin.voyager.deco.jobs.FullJobSeekerPreferences-32` | Jobs | Job seeker preferences |
| `com.linkedin.voyager.deco.jobs.JobPostingReferralWithDecoratedEmployee-9` | Jobs | Referral (employee view) |
| `com.linkedin.voyager.deco.jobs.JobPostingReferralWithDecoratedCandidate-10` | Jobs | Referral (candidate view) |
| `com.linkedin.voyager.deco.jobs.JobPosterComplianceCompactOrganization-5` | Jobs | Poster compliance |
| `com.linkedin.voyager.deco.identity.normalizedprofile.shared.ApplicantProfile-13` | Identity | Applicant profile |
| `com.linkedin.voyager.deco.identity.normalizedprofile.shared.ListedProfile-6` | Identity | Listed profile |
| `com.linkedin.voyager.deco.organization.shared.FullCompany-40` | Organization | Full company |
| `com.linkedin.voyager.deco.organization.shared.CompactCompany-5` | Organization | Compact company |
| `com.linkedin.voyager.deco.organization.shared.FullSchoolV2-19` | Organization | Full school |
| `com.linkedin.voyager.deco.organization.shared.EmployeeCulturalInsights-18` | Organization | Cultural insights |
| `com.linkedin.voyager.deco.organization.shared.EmployeeCareerInsights-5` | Organization | Career insights |
| `com.linkedin.voyager.deco.organization.shared.FullTargetedContent-16` | Organization | Targeted content |
| `com.linkedin.voyager.deco.organization.shared.OrganizationAdminUpdateCard-10` | Organization | Admin update card |

## Appendix C: Pagination Convention

All paginated endpoints use Rest.li standard pagination:

```
?start={offset}&count={pageSize}&paginationToken={token}
```

- `start`: 0-based offset
- `count`: page size
- `paginationToken`: optional continuation token (for cursor-based pagination)

## Appendix D: Rest.li Action Pattern

Many endpoints support the `?action={name}` query parameter pattern. This is Rest.li's "action" resource method, which maps to a POST request performing a named operation on a resource. Common actions:

- `create` -- create new entity
- `batchCreate` -- batch create
- `accept`, `ignore`, `withdraw`, `reportSpam` -- invitation actions
- `feedback`, `undoFeedback`, `negativeFeedback` -- feed actions
- `save`, `unsave` -- save/unsave actions
- `follow`, `unfollow`, `followByEntityUrn`, `unfollowByEntityUrn` -- follow actions
- `dismiss` -- dismiss/clear
- `consent` -- GDPR consent
- `submit` -- form submission
- `scan` -- virus scan
- `markAllItemsAsSeen`, `markAllItemsAsSeenByTypesAndTimestamp`, `clearUnseenCount` -- notification management
