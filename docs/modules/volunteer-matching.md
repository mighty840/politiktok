# F01 -- Volunteer Matching & Retention Engine

Matches volunteers to campaign tasks based on skills, availability, and location. Tracks engagement metrics to identify at-risk volunteers and improve retention over time.

## Key Features

- **Volunteer management**: Full CRUD for volunteer profiles including skills, availability (JSON), location (GeoJSON), tags, and bio.
- **Task management**: Create and manage campaign tasks with required skills, date ranges, location, and volunteer capacity limits.
- **Skill-based matching**: Composite scoring algorithm that ranks volunteers for a given task based on skill overlap and availability.
- **Churn detection**: Identifies volunteers with high churn scores (>0.7) who may be disengaging.
- **Assignment tracking**: Assigns volunteers to tasks with duplicate and capacity checks.
- **LLM-powered messaging**: Drafts personalized outreach, retention, and thank-you messages for volunteers using context from their profile and task details.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `create_volunteer` | `volunteer-matching/create-volunteer` | Create a new volunteer profile |
| `update_volunteer` | `volunteer-matching/update-volunteer` | Update an existing volunteer |
| `list_volunteers` | `volunteer-matching/list-volunteers` | List with search, status, and skill filters |
| `get_volunteer` | `volunteer-matching/get-volunteer` | Fetch a single volunteer by ID |
| `get_at_risk_volunteers` | `volunteer-matching/at-risk-volunteers` | List volunteers with churn_score > 0.7 |
| `create_task` | `volunteer-matching/create-task` | Create a new campaign task |
| `list_tasks` | `volunteer-matching/list-tasks` | List tasks with status and search filters |
| `get_task` | `volunteer-matching/get-task` | Fetch a single task by ID |
| `match_task` | `volunteer-matching/match-task` | Find best-matching volunteers for a task |
| `assign_volunteer` | `volunteer-matching/assign-volunteer` | Assign a volunteer to a task |
| `draft_message` | `volunteer-matching/draft-message` | Generate a personalized volunteer message |

## Matching Algorithm

The volunteer-task matching uses a composite score:

```
score = 0.6 * skills_overlap + 0.4 * availability_score
```

- **Skills overlap**: Count of matching skills divided by the number of required skills
- **Availability score**: 1.0 if the volunteer status is `active`, 0.5 otherwise

Results are sorted by composite score descending. Already-assigned volunteers are excluded.

## UI Components

- **Volunteers list page** (`/volunteers`): Searchable, filterable table of all volunteers with status badges and churn indicators.
- **Volunteer detail page** (`/volunteers/:id`): Full profile view with skills, availability, assignment history, and message drafting.
- **Tasks list page** (`/tasks`): Task listing with status filters and assigned volunteer counts.
- **Task detail page** (`/tasks/:id`): Task details with matching volunteer suggestions and one-click assignment.

## Database Tables

- `volunteers` -- volunteer profiles with skills (text[]), availability (jsonb), churn_score
- `tasks` -- campaign tasks with required_skills (text[]), date ranges, capacity
- `assignments` -- volunteer-to-task assignments with status tracking
