## FAQ

#### - What is PR?

PR is an abbreviation for [Pull Request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests).

#### - How should I name PR?

Each task should be submitted as a separate PR on its own branch. Use the following template for the PR name: `task_task_number`. Here are some examples:

- "task_0" for the task in the zero section,
- "task_1_3" for the third task in the first section,
- "task_5_3_11" for the 11th subtask of the third task in the fifth section,

#### - What is the final task?
This is the last task of the section, namely:
- Task_1_10 : 1_concepts/README.md
- Task_2_7 : 2_idioms/README.md 
- Task_3_12 : 3_ecosystem/README.md 
- Task_4_4 : 4_backend/README.md
Questions should be answered in writing only in them.

#### - Should I answer theoretical questions in PR?
Only in the [final tasks](https://github.com/rust-lang-ua/rust_incubator/edit/master/faq.md#--what-is-the-final-task).
In tasks that are not final, you do not need to answer theoretical questions. For example, in Problems 1_1, 1_2, you do not need to answer the theoretical questions in writing because they are not final.
In the final tasks, please write your answers in the same PR as the solution to the practical problem. Do not create a separate PR to answer the questions; it will not be accepted by mentors.

#### - When and how are the meetings conducted?

Meetings are held four times a week on weekdays (except Tuesday) according to your group's schedule. The meeting lasts about 1 hour. All appointments will be in your calendar. Attendance at the first meeting is mandatory.

#### - Will the meetings be recorded?

No, we usually don't record meetings. So, ensure you don't miss them.

#### - How do I start completing tasks?

[Our bot](https://github.com/1tbot) will create a private fork of the main [Rust Incubator repository](https://github.com/rust-lang-ua/rust_incubator) in [our organization](https://github.com/rust-lang-ua) and invite you. For more information on how to configure the received repository, see the [instructions](https://github.com/rust-lang-ua/rust_incubator/blob/master/orientation.md#getting-started).

#### - How and where do I send Pull Requests of completed tasks?

Create a separate branch for each task in your private fork. Complete the task in the corresponding branch and create a PR to your main master branch. Remember to tag all your mentors in the comments section of the PR.

#### - When is a task considered completed?

A task is marked as completed when you've made a Pull Request, and it's been reviewed and approved by one of the reviewers. After you complete your Pull Request, remove the "NOT READY / DRAFT / WIP" marker from the title. Don't forget to specify one of your mentors as a reviewer and tag all of your mentors in the comments section.

#### - How do mentors distinguish between finished Pool Requesters and unfinished Pool Requesters?

A task is marked as completed when you've made a Pull Request, and it's been reviewed and approved by one of the reviewers. After you complete your Pull Request, remove the "NOT READY / DRAFT / WIP" marker from the title. Don't forget to specify one of your mentors as a reviewer and tag all of your mentors in the comments section.

#### - Should I open an unfinished PR?

Yes, you should. Open a PR as soon as you can, marking it with "NOT READY / DRAFT / WIP" in the title. Your mentors will review it once you remove this marker and mention them in the PR's description.

#### - How many mentors need to approve before I can merge?

You need approval from at least one mentor before merging the Pull Request into the main branch. However, if more than one mentor has reviewed your Pull Request and requested changes, you must receive approval from all of them.

#### - Can I only complete the course assignments on weekends?

To achieve quality results, you should dedicate more than 20 hours a week to your studies. Three months of intensive study can equate to two years of independent practice. If you only study on weekends, you'd need to dedicate 10 hours each day. We recommend distributing your study time evenly throughout the week.

#### - Where should I answer the questions in the first chapter?

You can provide the answers under each question by editing your own md-file.

#### - Where can I ask for help?

Firstly, ask your peers in the Bootcamp's chat. You can also post your questions in the Telegram [chat of our community](https://t.me/rustlang_ua), the official [forum](https://users.rust-lang.org/). [ChatGPT](https://openai.com/blog/chatgpt) works well for simpler queries. However, remember that your solutions must be original, and anyone found sharing or copying solutions will be expelled.

#### - What is the best way to ask questions?

It's preferable to have your code in the [playground](https://play.rust-lang.org/). This way, mentors and peers don't have to spend extra time recreating the problem.

![playground_ask](https://github.com/rust-lang-ua/rust_incubator/assets/98274821/2351bddd-455f-4078-a7cb-328a7bb08ac9)

#### - Is the Telegram chat moderated?

Initially, we won't restrict communication in the chat or divide it into sub-chats. We encourage participants to get to know each other. However, we may implement stricter moderation rules later if needed. For now, respect all chat participants and refrain from offensive behavior.

#### - What is the motivation of mentors to mentor?

All our mentors are professionals, many of whom own their companies and are in a position to hire talented participants. Mentoring not only improves their knowledge but also satisfies their innate desire to help others.

#### - Do we cooperate with IT companies in any way?

We are making a lot of efforts to involve companies in interviews with bootcamp graduates and hopefully we will get there. We currently have 4 partner companies: TacansLab, AI Edge Lab, Intellias, and Near.

#### - What should I do if I find mistakes in the repository?

We encourage you to open an __issue__ if you see any problems.

#### - How can I incorporate changes from the main repository?

Add the main repository as a remote.

```bash
# Add the remote
git remote add upstream git@github.com:rust-lang-ua/rust_incubator.git
# Fetch the changes from the repository
git fetch --all
# Merge the changes
git merge upstream/master
```

*If you get the error `fatal: refusing to merge unrelated histories` add option `--allow-unrelated-histories`  flag to the last command.*

> NOTE: it is possible that the mentioned commands may result in numerous merge conflicts that would be tedious to resolve by hand.
> This might happen if the repository is modified in some way before it is synced with the template.
> If the changes are not significant, it is probably easier to try again from the start. Otherwise, here is one way to fix such an issue:

#### - What to do if you merged with `--allow-unrelated-histories`, but there are too many merge conflicts?

1. You need to restore the state of your repository before the unsuccessful merge.

Beacuse of how `--allow-unrelated-histories` merges commits, the only actual way to do so is to make a brand new clone of your repository from the state it was saved on your GitHub.
If you didn't publish your progress before attempting to merge, unfortunately, you are going to lose that progress.

```bash
git clone https://github.com/YourUsername/your_repo.git
cd your_repo
```

Since this is a new clone, we are going to need to setup a new remote for the template repository.

```bash
git remote add upstream https://github.com/rust-lang-ua/rust_incubator.git
```

2. Create a new branch and reset it to the state of the `upstream/master`

```bash
# fetch the data from the template repository
git fetch upstream master
# create a new brench and switch into it
git checkout -b template
# reset the branch to the state of upstream/master
git reset upstream/master
```

Now, instead of a bunch of conflicts, you have unstaged changes, which are much easier to work with.
In my case, I just had to discard all of the changes(correct state of the repository took priority). Your situation might be different.

3. Run `git status` and choose what to keep.

```bash
# this will show you all the unstaged changes
git status
# if you wish to keep some of the changes, you may add them
git add file1 file2 ...
# or, to keep all of the changes,
git add *
# if you wish to discard a particular change, run
git restore file1 file2 ...
# or you may discard all of the changes
git restore *
```

Once you're done with staging the changes, commit them. You may do so in multiple steps, if you wish.

```bash
git commit
```

4. Finally, you may merge the branches

```bash
git checkout master
git merge template -Xtheirs --allow-unrelated-histories
```
Done! At this point, all of your commits have common history with the template repository, and therefore ordinary

```bash
git fetch upstream master
git merge upstream/master
```

should suffice when it comes to updating your repository, and it should work without any unexpected problems or '--allow-unrelated-histories'.

If you are satisfied with the result, you can remove the `template` branch and publish your changes to GitHub.

```bash
git branch -D template
git push
```

#### - I can't choose my capstone project. Can you suggest some?

<details>
<summary>Gamepad Tester and Mapper</summary>
<br>
A program that displays button presses on a gamepad with cross-platform support. Implement creating an SDL mapping string for the button mapping.
</details>

<details>
<summary>Small TTV (Twitch TV) client</summary>
<br>
TTV client will use ttv.lol to remove ads and prevent the purple screen of death. You can use GStreamer bindings, but I recommend using libmpv as the player to play around with C-Rust interop. Code from Streamlink, Xtra, and others will be helpful.
</details>

<details>
<summary>Easy to use VPN through NAT traversal</summary>
<br>
I'm not sure if such a thing exists or if it makes sense, but a VPN through STUN or some kind of NAT traversal. You can use a part of WebRTC that generates SDP offers and establish a connection through a punched tunnel. The user only needs to insert a base64 string from another client, and the connection is established. Of course, this won't work with 50% of connections.
</details>

<details>
<summary>Decoder for an audio format</summary>
<br>
For those who enjoy reading 200-page RFCs, a decoder for a simple (most likely lossless and outdated, like .ape) audio format. However, it should be implemented as an addition to the Symphonia crate.
</details>

<details>
<summary>Reddit to Lemmy Proxy Bot</summary>
<br>
Make a bot that reposts subreddits to Lemmy.    
However, website parsing will be required. You can also earn stars on GitHub for this project.
</details>

<details>
<summary>Razer Chroma SDK REST API reimplementation with OpenRGB</summary>
<br>
For those who want to contribute to Linux gaming, a Razer Chroma SDK REST API reimplementation with OpenRGB. It's for those who want to work with backend frameworks. The server will map and redirect Chroma API calls to the OpenRGB API. A shim DLL can also be created to support games that use DLL. With this, even those who play on Linux under Wine will be able to experience RGB effects and it also will provide support for a greater number of devices. Of course, it won't be possible to cover the entire API in a week, but basic functionalities can be implemented.
</details>

<details>
<summary>Wireless Audio Transmitter</summary>
<br>
Wireless audio transmitter on an Arduino (NOT Bluetooth). Everyone is familiar with full-size wireless headphones that use a radio channel to transmit audio without delay and with higher quality than Bluetooth. The goal is to create an adapter with a jack connector that will transmit data to a USB receiver on a PC. The NRF24L01 radio module can be used for the wireless module.
</details>
