# F03 -- Sentiment Monitor

Monitors social media and public discourse to track sentiment trends around candidates, policies, and political topics in real time.

## Key Features

- **Post ingestion**: Bulk import social media posts from multiple platforms with automatic background classification.
- **LLM classification**: Each post is classified for sentiment (positive/negative/neutral), scored (-1.0 to 1.0), and tagged with topics.
- **Aggregated summaries**: Sentiment counts and trend scores grouped by topic and time window.
- **Spike detection**: Statistical anomaly detection that identifies when a topic's volume exceeds historical norms by a configurable threshold.
- **Topic discovery**: Automatic extraction of distinct topics from ingested posts.
- **Paginated feed**: Browse posts filtered by topic and sentiment with offset pagination.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `get_sentiment_summary` | `sentiment-summary` | Aggregate sentiment by topic and time window |
| `get_sentiment_feed` | `sentiment-feed` | Paginated post feed with filters |
| `get_topics` | `sentiment-topics` | List all distinct topics |
| `get_spike_log` | `sentiment-spikes` | Recent sentiment spike detections |
| `classify_post` | `sentiment-classify` | Classify a single post via LLM |
| `ingest_posts` | `sentiment-ingest` | Bulk insert posts with background classification |
| `detect_spikes` | `sentiment-detect-spikes` | Check for volume spikes on a topic |

## Classification Pipeline

1. Posts are inserted into `social_posts` via `ingest_posts`.
2. Each post is queued for background classification via `tokio::spawn`.
3. The LLM classifies sentiment and extracts topics as structured JSON.
4. The post record is updated with sentiment, score, and topics.

The classification prompt returns JSON:

```json
{
  "sentiment": "positive",
  "score": 0.75,
  "topics": ["healthcare", "insurance"]
}
```

## Spike Detection Algorithm

A spike is detected when:

```
magnitude = (current_count - historical_avg) / historical_stddev >= threshold
```

- Current count: posts matching the topic within the detection window
- Historical average and standard deviation: computed from 7-day bucketed history
- The dominant sentiment during the spike window is recorded

Detected spikes are persisted in the `sentiment_spikes` table with sample post IDs.

## Time Windows

Summaries support configurable time windows:

| Window | Interval |
|--------|----------|
| `1h` | 1 hour |
| `4h` | 4 hours |
| `24h` (default) | 24 hours |
| `7d` | 7 days |

## UI Components

- **Sentiment dashboard** (`/sentiment`): Overview with topic-level sentiment bars, trend indicators, and spike alerts.
- **Topic drill-down**: Click a topic to see its post feed filtered by sentiment.
- **Spike timeline**: Chronological list of detected spikes with magnitude and sample posts.

## Database Tables

- `social_posts` -- ingested posts with platform, text, sentiment, score, topics (text[]), coordination flags
- `sentiment_spikes` -- detected anomalies with topic, magnitude, and sample post IDs
