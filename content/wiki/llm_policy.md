+++
title = "LLM contribution policy for Linebender projects"
+++

Open-source projects are facing an increasing amount of submissions generated in whole or in part by LLMs.

Maintainers tend to strongly dislike them: they require very little effort to create (since a machine is doing all the work) but a lot of effort to review (since LLMs make mistakes that are hard to track down).
Because LLMs are good at mimicking high-effort contributors, maintainers often feel pressured to give the benefit of the doubt to the submitter and give a level of feedback that isn't warranted by the effort the submitter made.

Still, LLM agents can produce high-quality code, so we don't want to ban them altogether.

This document outlines Linebender's official policy for LLM contributions, inspired by [this zulip discussion](https://xi.zulipchat.com/#narrow/channel/419691-linebender/topic/AI.20slop.20policy/near/575407715).


## Disclosure

Contributors should disclose their LLM usage ahead of time.
Reviewers will *not* be happy if a contributor only admits to using LLMs after being prodded, and will tend to assume the contributor is downplaying their level of AI use.

This above applies even if the reviewer double-checked everything the LLM wrote.
In our experience, people overestimate their level of understanding of agent-produced code when the agent had a high level of initiative.
"Code you wrote yourself" and "Code you read after the LLM wrote it" do not warrant the same level of scrutiny.

Contributors should also disclose content that wasn't written by an agent, but was a direct result of LLM outputs, notably:

- Code or documentation that was created following an outline, plan or architecture proposed by an LLM.
- Code that was copy-pasted by an AI chat tool (e.g. ChatGPT), even if the code was double-checked or adjusted.

This disclosure should be included in the PR description, so that it appears in the final commit message.

### Copilot-like tools

Tools like Github Copilot or Cursor Tab that do small tab-completion of code you're writing fall in a grey area.

Completing single lines of code or small blocks of boilerplate is fine and doesn't require disclosure.

Completing large blocks of code or entire functions, however, is similar to asking an agent to generate it for you.
Writing a comment that says "The following does X" and then hitting tab a few times is almost identical to using an agent.

As a rule of thumb, in Rust, by the time Copilot produces completions with multiple `;` characters, you should treat it as LLM code that requires disclosure.

### Non-generation use of LLM

Using LLMs for debugging, exploration, testing (*not* generating test files), spell-checking and grammar-checking is accepted without disclosure.

### Anti-disclosure

Given the above, declaring that a PR *doesn't* include AI-generated content is redundant and unverifiable.
Please don't include such messages unless asked.


## Level of effort

The rule of thumb for all LLM content is "You should not ask someone to read text if reading it would take longer than it took you to create it".

As such, we do not allow LLM bots, PRs that were generated end-to-end by LLMs, or AI-generated PR descriptions (translations are fine with disclosure).

If a PR includes AI-generated content, we fully expect the submitter to review their own PR before asking anyone else to look at it.
They should spend as much effort on this self-review as they would on a human-authored PR.

In discussion spaces like Github comments and the Zulip server, please avoid posting AI-generated analyses, even if you vetted them.


## Documentation

In general, avoid generating documentation with AI.

Documentation is most valuable when it is intentional and concise.
Our users will read our documentation much more often than our code, which means that verbose documentation is much more damaging than verbose code.

Because generating documentation doesn't cost the contributor anything, generated doc mechanically leans towards verbosity, which translates to a worse reading experience.

For these reasons, Linebender projects hold documentation to a higher standard than code.

Two rules of thumb:

- Avoid generating documentation longer than one line with AI.
- Strongly avoid generating documentation longer than two paragraphs with AI.

If a submitted change requires a large amount of documentation, and you are uncomfortable with your writing skills, we prefer that you reach out to us and ask for advice rather than submit large amounts of LLM-generated docs.

In general, we value the process of communicating about documentation and expressing your intent more than we value the final artifact.
If you feel apprehensive about the quality of your docs, we'd encourage you to lean towards "ask us about it" rather than "use more AI".

If you still want to generate large amounts of documentation and are satisfied with the output, refer to the above sections about disclosure and effort.


## LLMs and non-English speakers

Most Linebender contributors don't speak English as their first language, and we understand that contributors might struggle with English and feel anxious about it.

As mentioned above, using AI to check your grammar or find ways to reword a sentence is fine and doesn't require disclosure.

Using AI to translate something you wrote in your native language usually doesn't require disclosure.

We consider these cases meaningfully different from asking AI to generate text for you from a prompt or having AI auto-generate doc for a code item, both of which require disclosure.


## Agent files

We will not merge agentic markdown files.

Some projects may include common agent files in their `.gitignore`.
