## How to Fork

To get started, you'll need to create a **private** fork of the [Rust Incubator](https://github.com/rust-lang-ua/rust_incubator). Each exercise should be completed via separate pull requests. Once a reviewer approves your pull request, merge it into the master/main branch. The exercise is considered completed at that point.

#### 1. [Generate](https://github.com/rust-lang-ua/rust_incubator/generate) a new [GitHub repository](https://help.github.com/articles/github-glossary/#repository) from this one [using it as a template ❗](https://help.github.com/en/articles/creating-a-repository-from-a-template). Ensure your repository is private.

 - Navigate to the [Rust Incubator](https://github.com/rust-lang-ua/rust_incubator).
 - Click the **"Use this template"** button and select **"create a new repository"**.
 <p align="center">
 <img src="/asset/img/fork/1.png"  width="600" align="center" />
 </p>

 - On the creation page, name your repository. Set visibility to **Private** and include only **master** branch (not include all branches). Click **"Create repository from template"**.
 <p align="center">
 <img src="/asset/img/fork/2.png"  width="600" align="center" />
 </p>

 - Your private fork is now ready. You can find it in your repositories list.

#### 2. [Invite our reviewers](https://help.github.com/en/articles/inviting-collaborators-to-a-personal-repository) to your repository as collaborators.
 - From your repository's main page, go to the "Settings" section.
 <p align="center">
 <img src="/asset/img/fork/3.png"  width="600" align="center" />
 </p>

 - On the left side, find the **"Manage access"** section and click **"Invite a collaborator"**.
 <p align="center">
 <img src="/asset/img/fork/4.png"  width="600" align="center" />
 </p>

 - Add [our bot](https://github.com/1tbot) as a collaborator.
 <p align="center">
 <img src="/asset/img/fork/5.png"  width="600" align="center" />
 </p>

 - Await approval from the invited collaborators.

#### 3. Stay Up-to-Date.

This course is continually evolving, and it's important to stay updated with the latest changes.

To keep your fork up-to-date, follow these commands:

```bash
git remote add upstream https://github.com/rust-lang-ua/rust_incubator.git
git fetch upstream master
git merge upstream/master --allow-unrelated-histories
```

To fetch the latest changes, use:
```bash
git fetch upstream master
git merge upstream/master
```

To stay aware of any new changes, consider [watch this repository on GitHub](https://github.com/rust-lang-ua/rust_incubator/subscription) or tracking it via [RSS subscription](https://github.com/rust-lang-ua/rust_incubator/commits/master.atom).

If you get the error `fatal: refusing to merge unrelated histories` add option `--allow-unrelated-histories`  flag to the last command.
