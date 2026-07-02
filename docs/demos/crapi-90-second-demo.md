# crAPI 90-second public demo script

## Goal

Make strangers understand Uxarion fast.

The demo is not about showing every command. The demo is about proving this:

> Uxarion can take an authorized local web target, test it like a security agent, save evidence, and produce a usable report.

Use OWASP crAPI only. Do not aim this demo at a real public target.

## Recording setup

Use a clean terminal with a large font. Hide API keys, tokens, local usernames, and private paths.

Recommended layout:

- left: terminal running Uxarion,
- right: browser showing crAPI,
- small bottom/right: generated report or evidence folder near the end.

Required local services:

```text
crAPI:   http://127.0.0.1:8888
MailHog: http://127.0.0.1:8025
ZAP:     optional, only if configured cleanly
```

## Demo title

Use this exact title on the thumbnail or first screen:

```text
I gave an AI security agent a vulnerable app.
It found 4 real bugs and wrote the report.
```

## Timeline

### 0-5s: hook

On screen:

```text
Most AI coding agents can poke at an app.
Uxarion is built to test authorized targets and save evidence.
```

Voiceover:

```text
I built Uxarion, a local-first AI security agent. In this demo, I point it at OWASP crAPI, a deliberately vulnerable app, and make it produce evidence and a report.
```

### 5-15s: scope

Show the target and say:

```text
Target: http://127.0.0.1:8888
Scope: local lab only
MailHog: http://127.0.0.1:8025
```

Voiceover:

```text
This is an authorized local lab. Uxarion binds the session to the target scope before testing.
```

### 15-25s: launch Uxarion

Show:

```bash
uxarion
```

Then prompt Uxarion:

```text
Assess http://127.0.0.1:8888 as an authorized local OWASP crAPI lab. Focus on authorization and state bugs. Save evidence for confirmed findings and write a final report.
```

Voiceover:

```text
The important part is not just scanning. I ask it to focus on authorization and state bugs, save evidence, and write a report.
```

### 25-50s: discovery and confirmation

Show fast cuts of Uxarion:

- inspecting routes,
- comparing behavior between two accounts,
- replaying a request,
- confirming a BOLA/IDOR-style issue,
- saving evidence.

On screen text:

```text
Confirmed: user can access or act on another user's data
Evidence: request + response saved
```

Voiceover:

```text
Here it compares accounts and confirms that one user can access or act on another user's data. The point is not a flashy exploit. The point is confirmed evidence.
```

### 50-70s: findings

Show a terminal/report view with a compact summary.

On screen:

```text
Findings found: 4
Type: authorization / state issues
Evidence: saved
Report: generated
```

Voiceover:

```text
In this run, Uxarion found four real issues in the vulnerable app, including BOLA-style authorization problems.
```

### 70-85s: report

Show `report.md` or the generated report preview.

On screen:

```text
Each finding includes:
- target
- severity
- confidence
- evidence
- reproduction status
- impact
- limitations
```

Voiceover:

```text
It does not just say 'vulnerable.' It writes a report from saved artifacts: target, severity, confidence, reproduction status, impact, limitations, and evidence.
```

### 85-90s: CTA

On screen:

```text
Open source: github.com/rachidlaad/uxarion
Install: npm install -g uxarion
```

Voiceover:

```text
The repo is open source. Try it on a local lab or your own authorized target.
```

## What to cut

Cut anything that does not prove the promise.

Remove:

- long installation time,
- model thinking pauses,
- irrelevant terminal scrolling,
- private paths,
- failed setup attempts,
- raw tokens or cookies,
- long exploit details that make the demo look irresponsible.

## What to show clearly

Keep these shots:

1. target URL and scope,
2. Uxarion prompt,
3. account comparison or request replay,
4. evidence being saved,
5. final report,
6. install command and GitHub link.

## Short caption

```text
I built Uxarion: a local-first AI security agent for authorized web testing.

In this demo it tests OWASP crAPI, confirms 4 authorization/state bugs, saves request/response evidence, and generates a pentest-style report.

Open source: github.com/rachidlaad/uxarion
```

## Brutal quality bar

If a stranger watches this and still asks "what does it do?", the demo failed.

If a stranger watches this and says "okay, I want to try it on crAPI," the demo worked.
