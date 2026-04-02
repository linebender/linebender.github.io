# LLM contribution policy for Linebender projects

Open-source projects are facing an increasing amount of submissions generated in whole or in part by LLMs.

Maintainers tend to strongly dislike them: they require very little effort to create (since a machine is doing all the work) but a lot of effort to review (since LLMs make mistakes that are hard to track down). Because LLMs are good at mimmicking high-effort contributors, maintainers often feel pressured to give the benefit of the doubt to the submitter and give a level of feedback that isn't warranted by the effort the submitter made.

Still, LLM agents can produce high-quality code, so we don't want to band them altogether.

This document outlines Linebender's official policy for LLM contributions, inspired by [this zulip discussion](https://xi.zulipchat.com/#narrow/channel/419691-linebender/topic/AI.20slop.20policy/near/575407715).


## Disclosure

Contributors should disclose their LLM usage ahead of time. Reviewers will *not* be happy if a contributor only admits to using LLMs after being prodded, and will tend to assume the reviewer is downplaying their level of AI use.

This above applies even if the reviewer double-checked everything the LLM wrote. In our experience, people overestimate their level of understanding of agent-produced code when the agent had a high level of initiative. "Code you wrote yourself" and "Code you read after the LLM wrote it" do not warrant the same level of scrutiny.

Contributors should also disclose content that wasn't written by an agent, but was a direct result of LLM outputs, notably:

- Code or documentation that was created following an outline, plan or architecture proposed by an LLM.
- Code that was copy-pasted by an AI chat tool (e.g. ChatGPT), even if the code was double-checked or adjusted.

This disclosure should be included in the PR description, so that it appears in the final commit message.

### Copilot-like tools

Tools like Github Copilot or Cursor Tab that do small tab-completion of code you're writing fall in a grey area.

Completing single lines of code or small blocks of boilerplate is fine and doesn't require disclosure.

Completing large blocks of code or entire functions, however, is similar to asking an agent to generate it for you. Writing a comment that says "The following does X" and then hitting tab a few times is almost identical to using an agent.

As a rule of thumb, in Rust, by the time Copilot produces completions with multiple `;` characters, you should treat it as LLM code that requires disclosure.

### Non-generation use of LLM

Using LLMs for debugging, exploration, testing (*not* generating test files) is accepted without disclosure.

### Anti-disclosure

Given the above, declaring that a PR *doesn't* include AI-generated content is redundant and unverifiable. Please don't include such messages unless asked.


## Level of effort

The rule of thumb for all LLM content is "You should not ask someone to read text if reading it would take longer than it took you to create it".

As such, we do not allow LLM bots, PRs that were generated end-to-end by LLMs, or AI-generated PR descriptions (translations are fine with disclosure).

If a PR includes AI-generated content, we fully expect the submitter to review their own PR before asking anyone else to look at it. They should spend as much effort on this self-review as they would on a human-authored PR.

In discussion spaces like Github comments and the Zulip server, please avoid posting AI-generated analyses, even if you vetted them.


## Documentation

In general, avoid generating *any* documentation longer than one line with AI. *Never* generate documentation longer than two paragraphs with AI.

AI-generated documentation tends to be very verbose and redundant. Because generating it doesn't cost the contributor anything, they lean towards "more is better", which translates to a worse reading experience.

If you're not confident in your documentation skills, writing stub documentation with TODO comments is better than verbose-but-meaningless LLM documentation.


## Agent files

We will not merge agentic markdown files.

Some projects may include common agent files in their `.gitignore`.
