+++
title = "Preparing Office Hours"
+++

If you've volunteered to host Office Hours, the first step will be to create the agenda document.

Create a copy of [this template file](https://docs.google.com/document/d/10wUKjCAEIvYCQLnp_QpdFrAXkO0sBnarG2abbI7mduQ/edit?tab=t.0) in the [Office Hours Google drive folder](https://drive.google.com/drive/folders/1mmrRnlpYb3j5P0ewAOcY_zj2tTH5u5aO) (you may need additional permissions).

You then need to fill out the sections about the different projects, listing the important changes.

You may want to use a script like the following to programmatically get a list of recently-changed issues and PRs:

```js
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

for (const repo of repos) {
  const url =
    `https://api.github.com/repos/${repo}/issues?since=${since}&state=all&per_page=100`;

  const res = await fetch(url, {
    headers: {
      "Authorization": `token ${token}`,
      "Accept": "application/vnd.github.v3+json",
    },
  });

  if (!res.ok) {
    console.error(`Error: ${res.status} ${res.statusText}`);
    Deno.exit(1);
  }

  const issues = await res.json();

  const results = await Promise.all(issues.map(async (issue) => {
    const isPR = !!issue.pull_request;
    const createdRecently = new Date(issue.created_at) >= new Date(since);

    let label = "ACTIVITY";

    if (!isPR && createdRecently) {
      label = "open issue";
    } else if (isPR) {
      // Fetch PR details to determine draft/merged status
      const prRes = await fetch(issue.pull_request.url, {
        headers: {
          "Authorization": `token ${token}`,
          "Accept": "application/vnd.github.v3+json",
        },
      });

      if (prRes.ok) {
        const pr = await prRes.json();
        if (pr.merged_at && new Date(pr.merged_at) >= new Date(since)) {
          label = "merged";
        } else if (pr.draft && createdRecently) {
          label = "draft";
        } else if (!pr.draft && createdRecently) {
          label = "awaiting review";
        }
      }
    }

    return `[#${issue.number}](${issue.html_url}) (${label}) - ${issue.title}`;
  }));

  console.log(`=== REPO: ${repo} ===`);
  for (const line of results.reverse()) {
    console.log(line);
  }
  console.log("");
}
```

Once this is done, follow the checklist at the bottom of the template.
It includes tasks like changing the links, making the document public, linking to it on Zulip, etc.

The last step is to connect to the designated Google Meet room (possibly one you've created yourself) and start hosting the discussions.

Have fun!
