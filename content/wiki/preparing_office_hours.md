+++
title = "Preparing Office Hours"
+++

If you've volunteered to host Office Hours, the first step will be to create the agenda document.

Create a copy of [this template file](https://docs.google.com/document/d/10wUKjCAEIvYCQLnp_QpdFrAXkO0sBnarG2abbI7mduQ/edit?tab=t.0) in the [Office Hours Google drive folder](https://drive.google.com/drive/folders/1mmrRnlpYb3j5P0ewAOcY_zj2tTH5u5aO) (you may need additional permissions).

You then need to fill out the sections about the different projects, listing the important changes.

We've written a Deno script to automate the process, but you could also collect it manually using [Github's "pulse" pages](https://github.com/linebender/vello/pulse) or write your own script.
This programmatically fetches a list of recently-changed issues and PRs:

```js
// collect-issues.mjs

// Copyright 2025 the Linebender Authors
// SPDX-License-Identifier: Apache-2.0

// Create new token at https://github.com/settings/tokens/new
// The token is necessary because otherwise you're likely to hit rate limits
const token = "<your-github-token>";

const repos = [
  "linebender/xilem",
  "linebender/vello",
  "linebender/parley",
  "rust-mobile/android-view",
  "AccessKit/accesskit",
  "linebender/interpoli",
  "linebender/peniko",
  "linebender/kurbo",
  "linebender/color",
  "linebender/kompari",
  "linebender/tiny-skia",
  "linebender/simplecss",
  "linebender/svgtypes",
  "linebender/resvg",
  "linebender/bevy_vello",
  "linebender/velato",
  "linebender/vello_svg",
  "linebender/linebender.github.io",
];

const since = new Date(Date.now() - 7 * 24 * 60 * 60 * 1000).toISOString();

const headers = {
  "User-Agent": "linebender-scrapping-script",
  "Authorization": `token ${token}`,
  "X-GitHub-Api-Version": "2022-11-28",
  "Accept": "application/vnd.github+json",
};

for (const repo of repos) {
  let issues = [];
  let page = 1;

  while (true) {
    // See https://docs.github.com/en/rest/issues/issues?apiVersion=2022-11-28#list-repository-issues
    const url =
    `https://api.github.com/repos/${repo}/issues?since=${since}&state=all&per_page=100&page=${page}`;

    const res = await fetch(url, { headers });

    if (!res.ok) {
      throw new Error(`Error: ${res.status} ${res.statusText}`);
    }

    let pageItems = await res.json();
    issues.push(...pageItems);

    if (pageItems.length < 100)
      break;
  }

  const repoName = repo.split("/").at(-1);

  const results = await Promise.all(issues.map(async (issue) => {
    const isPR = issue.pull_request != null;
    const isOpen = issue.state == "open";
    const createdRecently = new Date(issue.created_at) >= new Date(since);

    let label;

    // ISSUE
    if (!isPR) {
      label = "[ISSUE ACTIVITY]";
      if (createdRecently) {
        label = "(open issue)";
      } else if (!isOpen) {
        label = "(issue closed)";
      }
    }

    // PR
    else {
      label = "[PR ACTIVITY]";

      // Fetch PR details to determine draft/merged status
      const prRes = await fetch(issue.pull_request.url, { headers });

      if (!prRes.ok) {
        throw new Error(`Error fetching PR #${issue.number}: ${prRes.status} ${prRes.statusText}`);
      }

      const pr = await prRes.json();
      if (pr.merged_at && new Date(pr.merged_at) >= new Date(since)) {
        label = "🎉";
      } else if (!isOpen && !createdRecently) {
        label = "(pr closed)";
      } else if (pr.draft && isOpen && createdRecently) {
        label = "(draft)";
      } else if (!pr.draft && isOpen && createdRecently) {
        label = "(awaiting review)";
      }
    }

    // The logic above should leave a lot of items with a label
    // of [ISSUE/PR ACTIVITY]. These should be investigated manually.

    return `[${repoName}#${issue.number}](${issue.html_url}) ${label} - ${issue.title}`;
  }));

  console.log(`=== REPO: ${repo} ===`);
  for (const line of results.reverse()) {
    console.log(line);
  }
  console.log("");
}
```

Note that the script shows you virtually all issues/PR which saw activity in the last seven days.
Some of them may not be worth mentioning, or have already been mentioned in previous meetings, etc.
You should still do some manual sorting with the results.

Once this is done, follow the checklist at the bottom of the template.
It includes tasks like changing the links, making the document public, linking to it on Zulip, etc.

The last step is to connect to the designated Google Meet room (possibly one you've created yourself) and start hosting the discussions.

Have fun!
