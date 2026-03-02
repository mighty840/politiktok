# F24 -- Meeting Summarizer

Summarizes political meetings, committee sessions, and strategy calls, extracting action items and tracking their completion.

## Key Features

- **Transcript summarization**: Submit meeting transcripts for automatic AI-powered summarization.
- **Key decision extraction**: Identify and list important decisions made during the meeting.
- **Action item extraction**: Pull out specific tasks with assignees, deadlines, and status.
- **Participant identification**: Detect who participated and their contributions.
- **Action tracking**: Monitor completion status of extracted action items over time.
- **Meeting history**: Searchable archive of past meeting summaries.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `summarize_meeting` | `meetings/summarize` | Summarize a meeting transcript |
| `list_meetings` | `meetings/list` | List past meeting summaries |
| `get_meeting` | `meetings/get` | Retrieve a specific meeting summary |
| `update_action_item` | `meetings/update-action` | Update action item status |

## Action Item Structure

Each extracted action item includes:

- **Description**: What needs to be done
- **Assignee**: Who is responsible
- **Deadline**: When it needs to be completed
- **Status**: pending, in_progress, completed, or overdue

## Meeting Summary Structure

A complete meeting summary includes:

- **Title**: Meeting title or generated name
- **Transcript**: The original transcript text
- **Summary**: Concise summary of the meeting
- **Key decisions**: List of decisions reached
- **Action items**: List of follow-up tasks with ownership
- **Participants**: People identified in the transcript

## UI Components

- **Meetings page** (`/meetings`): Transcript submission form with generated summary and action items.
- **Summary view**: Concise meeting overview with expandable sections.
- **Action item tracker**: Table of extracted actions with status toggles and deadline indicators.
- **Meeting history**: Searchable list of past meetings with action item completion rates.

## Database Tables

- `meeting_summaries` -- titles, transcripts, summaries, key decisions (text[]), action items (jsonb)
