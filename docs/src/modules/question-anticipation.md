# F16 -- Question Anticipation

Predicts likely voter questions based on current events, local issues, and trending topics, preparing candidates with ready responses and preparation notes.

## Key Features

- **Event-based prediction**: Generate anticipated questions tailored to specific event types (town hall, debate, press conference, rally).
- **Likelihood scoring**: Each question is rated by likelihood (high, medium, low).
- **Topic categorization**: Questions are grouped by policy topic.
- **Suggested answers**: AI-generated response drafts for each anticipated question.
- **Preparation notes**: Additional context and data points to strengthen the candidate's response.
- **Report generation**: Compile all anticipated questions into a printable preparation document.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `anticipate_questions` | `question-anticipation/anticipate` | Generate anticipated questions for an event |
| `list_reports` | `question-anticipation/list` | List past question reports |
| `get_report` | `question-anticipation/get` | Retrieve a specific question report |

## Question Structure

Each anticipated question includes:

- **Question**: The predicted voter question
- **Likelihood**: high, medium, or low probability of being asked
- **Topic**: Policy area (economy, healthcare, education, etc.)
- **Suggested answer**: A draft response the candidate can adapt
- **Preparation notes**: Background data, statistics, and talking points

## Report Structure

A complete question report includes:

- **Context**: Event description and current political landscape
- **Event type**: town_hall, debate, press_conference, or rally
- **Questions**: Ordered list of anticipated questions sorted by likelihood

## UI Components

- **Question anticipation page** (`/question-anticipation`): Event context form with generated question cards.
- **Question cards**: Expandable cards showing question, suggested answer, and prep notes with likelihood badges.
- **Topic filter**: Filter anticipated questions by policy topic.
- **Print view**: Formatted preparation document for offline review.

## Database Tables

- `question_reports` -- event context, event type, anticipated questions (jsonb)
