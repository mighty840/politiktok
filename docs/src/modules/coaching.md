# F14 -- Coaching & Debate Rehearsal

Provides AI-driven coaching for candidates including debate rehearsal, simulated press interviews, and town hall practice sessions.

## Key Features

- **Multiple coaching modes**: Journalist interview simulation, debate rehearsal, and town hall Q&A practice.
- **Configurable difficulty**: Easy, medium, and hard settings control how aggressive the interviewer is.
- **Topic focus**: Specify topics for the session to practice specific policy areas.
- **Conversation continuity**: Multi-turn sessions where the AI adapts its follow-up questions based on the candidate's responses.
- **In-memory sessions**: Session state is maintained client-side (no database persistence required), keeping conversations private.
- **Performance feedback**: Post-session analysis of the candidate's performance.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `start_session` | `coaching/start` | Initialize a new coaching session |
| `send_response` | `coaching/respond` | Send a candidate response and get the next question |
| `end_session` | `coaching/end` | End session and get performance feedback |

## Coaching Modes

| Mode | Simulation |
|------|-----------|
| `journalist` | Press conference or one-on-one interview with a journalist |
| `debate` | Debate stage with an opposing candidate |
| `townhall` | Voter questions at a town hall event |

## Difficulty Levels

| Level | Behavior |
|-------|----------|
| `easy` | Straightforward questions, no follow-ups, friendly tone |
| `medium` | Probing questions, occasional follow-ups, neutral tone |
| `hard` | Aggressive questioning, persistent follow-ups, adversarial tone |

## Session Structure

```rust
pub struct CoachingSession {
    pub id: String,
    pub mode: String,          // "journalist", "debate", "townhall"
    pub topics: Vec<String>,
    pub difficulty: String,    // "easy", "medium", "hard"
    pub messages: Vec<CoachingMessage>,
    pub created_at: Option<String>,
}
```

Messages alternate between `interviewer` and `candidate` roles, building a natural conversation flow.

## UI Components

- **Coaching page** (`/coaching`): Session configuration panel with mode, difficulty, and topic selectors.
- **Chat interface**: Real-time conversation view with the AI interviewer.
- **Feedback panel**: Post-session analysis with strengths, weaknesses, and improvement suggestions.

## Database Tables

None -- sessions are maintained in-memory on the client for privacy.
