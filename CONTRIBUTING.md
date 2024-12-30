# Contributing to enygmah

Thanks for your interest! :heart: We would love
for you to contribute to enygmah and help make it even better than it is today!

As a contributor, here is an overview of things to learn about and ways to get involved:

- [Code of Conduct](#code-of-conduct)
- [How can I help?](#how-can-i-help)
   - [Code Contributions](#code-contributions)
- [Question or Problem?](#got-a-question-or-a-problem)
- [Issues and Bugs](#found-a-bug)
- [Feature Requests](#missing-a-feature)
- [Submit an Issue](#submit-an-issue)
- [Submit a Pull Request](#submit-a-pull-request-pr)

## Code of Conduct

Help us keep enygmah open and inclusive.
Please read and follow our [Code of Conduct][coc].

## How can I help?

There are many ways you can help. Here are some ways to help without coding:

- You can be help others on our [Github Discussions Page][discussions].
- You can [contribute to the official docs](https://github.com/hotaydev/enygmah-docs).
- You can confirm bugs on the [issue tracker][issue-tracker] and mention reproducible steps. It helps the core team to get more reports so we can fix the highest priority bugs.
<!-- - You can contribute to [translations][translations] with a [pull request](#submit-a-pull-request-pr). -->

For ways to help with coding, read the next section.

### Code Contributions

For contributors who want to help with coding, we have a list of [good first issues](https://github.com/hotaydev/enygmah/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) to help you get started.
These are issues that are beginner-friendly and do not require advanced knowledge of the codebase. We encourage new contributors to start with these issues and gradually work their way up to more challenging tasks.

## Got a Question or a Problem?

Please do not open issues for general support questions as we want to keep GitHub issues for bug reports and feature requests.
Instead, we recommend using our [Github Discussions][discussions] tab to ask support-related questions.

These channels are a much better place to ask questions since:

- there are more people willing to help there
- questions and answers stay available for public viewing so your question/answer might help someone else
- The channels' voting system assures that the best answers are prominently visible.

To save your and our time, we will systematically close all issues that are requests for general support and redirect people to the forum.

If you would like to chat about the question in real-time, you can reach out talking with [@TaylorHo][https://github.com/TaylorHo].

## Found a Bug?

If you find a bug, you can help us by [submitting an issue](#submit-an-issue) to our [GitHub Repository][github].
Even better, you can [submit a Pull Request](#submit-a-pull-request-pr) with a fix.

## Missing a Feature?

You can *request* a new feature by [submitting an issue](#submit-an-issue) using our "Feature Request" template.
If you would like to *implement* a new feature, please open an issue and outline your proposal so that it can be discussed. 
It's always beneficial to engage in discussions about a feature before commencing its development. This proactive approach helps identify whether the feature may be controversial or lacks widespread utility.

## Submit an Issue

Before you submit an issue, please search the [issue tracker][issue-tracker]. An issue for your problem might already exist and the discussion might inform you of workarounds readily available.

To submit an issue, [fill out the issue using a template][new-issue]. Please file a single issue per problem and do not enumerate multiple bugs in the same issue.

The Issues page provides various templates, covering topics like installation problems, bugs, security flaws, performance issues, and feature requests. Select a template and describe your issue, using the included specific questions to guide you in detailing the problem.

## Submit a Pull Request (PR)

Your PR should be open from **your branch directly into the main branch**. We manage the versions by publishing a release and a tag here, on GithUb.

Before working on your pull request, please check the following:

1. Search [GitHub][search-pr] for related PRs that may effect your submission.

2. Be sure that there is an issue that describes the problem you're fixing or the feature behavior and design you'd like to add.

After doing the above, you are ready to work on your PR! To create a PR, fork this repository and then create a branch for the fix. Once you push your code to your fork, you'll be able to open a PR to the enygmah repository.

For more info you can follow this [GitHub guide](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request-from-a-fork).
For more github PR guides, see [these guides](https://docs.github.com/en/pull-requests).

### PR Guidelines

When submitting a Pull Request (PR) or expecting a subsequent review, please follow these guidelines:

1. The PR is ready for review. If you you have work you know how to do, then please keep your changes locally or in your own fork until they are ready. If you need help with your PR, feel free to submit with questions as a [draft PR](https://github.blog/2019-02-14-introducing-draft-pull-requests/).

2. The PR checks which include tests and lint checks are passing.

3. The PR has no merge conflicts.

4. Commits and Pull Request titles should use the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/#summary) prefixes/types. The commit message and the PR title should have the same format, e.g. `<prefix>: description ...`. The most used prefixes are:
   * `fix` or `bugfix` - Bug fixes.
   * `feat` or `feature` - New features.
   * `chore` - Misc changes that aren't feat or fix. We usually avoid using this category.
   * `enhance` - Enhancements to existing features.
   * `test` - Test only changes.
   * `ci` - Changes to the CI/CD system of flow.
   * `build` - Changes to the build system (usually related to [cargo](https://doc.rust-lang.org/cargo/) logic).
   * `docs` - Changes to the documentation. Synce we have a dedicated repo to this, you can use this category to add in code docs to functions and methods.
   * `style` - Changes to the code styles and patterns.
   * `refactor` - Related to some code refactoring, aiming some readability or simplicity improvements.
   * `perf` or `performance` - Specific to the code performance or application performance.

> If your PR introduces a breaking change, add a `!` at the final of the commit, e.g. `<prefix>!: description ...`

5.  The PR having "allow edits from maintainers" enabled would be appreciated. Helps us help your contribution.

6. The PR avoids the following changes that are not helpful to the core team:
   * Unrelated refactoring or heavy refactoring
   * Dependency updates e.g. in package.json
   * Changes that contain multiple unverified resources. This is risky for our users and is a lot of work to verify. A change with one resource that can be verified is acceptable.

### PR Additional Links

* To run enygmah locally, just clone this repo, run `cargo build` and test analyzing it's own source code with `cargo run -- scan .`
* For security vulnerabilities reports, see our [Securitity Guide](https://github.com/hotaydev/enygmah/blob/main/.github/SECURITY.md).
* If you need some other support, [he have a doc for that](https://github.com/hotaydev/enygmah/blob/main/.github/SUPPORT.md).

## Thank You

Your contributions to open source, large or small, make great projects like this possible. Thank you for taking the time to contribute.

[coc]: https://github.com/hotaydev/enygmah/blob/main/.github/CODE_OF_CONDUCT.md "enygmah Code Of Conduct"
[discussions]: https://github.com/hotaydev/enygmah/discussions/categories/q-a "Github Discussions"
[github]: https://github.com/hotaydev/enygmah "enygmah Repo"
[issue-tracker]: https://github.com/hotaydev/enygmah/issues "enygmah Issue Tracker"
[new-issue]: https://github.com/hotaydev/enygmah/issues/new "Submit a New issue"
[search-pr]: https://github.com/hotaydev/enygmah/pulls "Search open PRs"
