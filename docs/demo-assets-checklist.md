# Demo assets checklist

This checklist exists because a good product demo is not just a screen recording. It is a sales asset.

## Minimum assets before posting publicly

### 1. Short video

- Length: 60-90 seconds.
- Format: vertical and horizontal if possible.
- Hook in first 3 seconds.
- Show Uxarion, target scope, evidence, and report.
- Hide all keys, cookies, tokens, personal paths, and private accounts.

### 2. Long video

- Length: 5-7 minutes.
- Format: horizontal.
- Includes setup, scope, workflow, evidence, report, limitations.
- Has chapters.
- Uploaded somewhere linkable.

### 3. Screenshots

Create these exact screenshots:

1. Uxarion terminal after launch.
2. Scope prompt with `http://127.0.0.1:8888`.
3. Confirmed finding summary.
4. Evidence artifact saved.
5. Generated report.
6. README top section with install command.

### 4. Sample report

Add a sanitized sample report folder.

Recommended path:

```text
examples/crapi-sample-report/report.md
```

The report should contain:

- assessment scope,
- target,
- methodology,
- findings table,
- one detailed finding,
- evidence references,
- reproduction status,
- limitations,
- remediation guidance.

### 5. README proof section

Add this near the top of the README:

```md
## Demo result

| Target | Result | Output |
| --- | --- | --- |
| OWASP crAPI local lab | 4 confirmed authorization/state bugs | Saved evidence + generated Markdown report |
```

### 6. Repro tutorial

Add a tutorial called:

```text
docs/demos/crapi-repro-tutorial.md
```

It should walk a user from local lab startup to generated report.

### 7. Feedback issue template

Create a GitHub issue template for demo feedback:

```text
What OS are you using?
How did you install Uxarion?
What target/lab did you test?
Where did setup break?
Did evidence/report generation work?
What was unclear?
```

## Quality bar

Do not post until these are true:

- A stranger can tell what Uxarion does in 10 seconds.
- A stranger can install it without asking you.
- A stranger can reproduce at least one local-lab result.
- A stranger can see a real generated report.
- The safety boundary is obvious.

## Do not publish with these problems

- README only says abstract AI/security words.
- Demo is longer than 2 minutes with no clear hook.
- No sample output.
- No report screenshot.
- No clear target scope.
- No install status for Windows/macOS/Linux.
- No explanation of authorized use.

## The best first public demo package

```text
README proof table
+ 90-second video
+ 5-minute technical video
+ sample report
+ crAPI tutorial
+ social launch pack
```

Anything less is weaker than it should be.
