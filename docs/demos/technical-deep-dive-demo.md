# Technical deep-dive demo script

## Goal

This is the 5-7 minute demo for developers, security students, and bug bounty beginners who want to see how Uxarion actually works.

The 90-second demo sells attention. This deep dive converts attention into trust.

## Main promise

```text
Uxarion is not just a chat wrapper.
It runs a scoped security workflow, uses security tools, records evidence, stores findings, and writes a report.
```

## Demo structure

### 1. Start with the problem

Say:

```text
Normal AI coding agents are dangerous for security work because they blur scope, mix coding and testing, and often leave the result as chat text. Uxarion is built around scoped, operator-driven security assessment.
```

Show the README install command:

```bash
npm install -g uxarion
```

Then launch:

```bash
uxarion
```

### 2. Show target scope

Use only a local vulnerable app:

```text
Target: http://127.0.0.1:8888
Scope: OWASP crAPI local lab only
```

Ask:

```text
Set the scope to http://127.0.0.1:8888 only. Assess this authorized local crAPI lab for authorization and state-management issues. Prefer passive inspection first, then bounded verification. Save evidence for confirmed findings and generate a final report.
```

Explain:

```text
The exact target matters. A security agent should not wander from a local lab to the internet or a different host.
```

### 3. Show the workflow loop

Show Uxarion performing these steps:

1. inspect pages and API behavior,
2. identify account-specific endpoints,
3. compare behavior across users,
4. replay a bounded request,
5. mark the result as confirmed, inconclusive, or not reproduced,
6. save evidence,
7. record a finding.

Say:

```text
The workflow is not magic. It is careful comparison, request inspection, evidence, and reporting.
```

### 4. Show evidence, not just claims

Open the saved evidence artifact or show the terminal output pointing to it.

Say:

```text
A finding without evidence is just a guess. Uxarion saves artifacts so the report can be checked later.
```

Show:

```text
Evidence saved
Finding recorded
Report generated
```

### 5. Show the report

Open the generated Markdown report.

Highlight sections:

- summary,
- finding title,
- severity,
- confidence,
- reproduction status,
- impact,
- evidence references,
- limitations.

Say:

```text
This is the part that makes the tool useful. The result is not only a chat answer. It is a security artifact you can send, review, or improve.
```

### 6. Explain safety and limits

Say this directly:

```text
Uxarion is for authorized testing. It is local-first and operator-driven. It is not a tool for attacking random public targets. It is useful when you own the target, have permission, or are working inside a legal lab or bug bounty scope.
```

This line protects the project and makes serious users trust you more.

### 7. End with what is next

Say:

```text
Next I am improving sample reports, Burp/ZAP workflows, and bug-bounty style exports. If you test web apps or are learning security, try the crAPI demo and open an issue with where the setup breaks.
```

## Exact video chapters

```text
00:00 Why security agents need scope
00:35 Install and launch
01:05 Target setup: crAPI local lab
01:40 Scoped assessment prompt
02:20 Discovery and account comparison
03:20 Confirming an authorization issue
04:20 Evidence capture
05:00 Generated report
06:00 Safety model and next steps
```

## Recording checklist

Before recording:

- update Uxarion to latest release,
- use a fresh terminal profile,
- increase terminal font size,
- disable notifications,
- prepare two crAPI accounts,
- prepare MailHog if the demo needs email flows,
- delete or hide old artifact paths,
- do one full dry run without recording.

During recording:

- narrate the purpose of each step,
- do not show API keys,
- do not show real personal accounts,
- do not test real public targets,
- do not include long pauses.

After recording:

- cut dead time,
- add chapter labels,
- add captions,
- export a 90-second cut and a 5-7 minute cut,
- upload the long demo to YouTube or Loom,
- use the short cut for X, LinkedIn, Reddit, and Hacker News.

## Deep-dive post caption

```text
I made a longer technical walkthrough of Uxarion.

The key idea: security agents should not just chat. They should run inside explicit target scope, preserve evidence, record findings, and generate a report from artifacts.

Demo target: OWASP crAPI local lab.
Repo: github.com/rachidlaad/uxarion
```
