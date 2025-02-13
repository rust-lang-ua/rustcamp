# Chapter 6: Capstone project

Congratulation! You've almost done it!

Create a portfolio project using Rust within a limited timeframe.

---

## Task 6.1: Agree on your capstone project

Create a subject for your project.

> Note:
> it is better to start creating a _project idea_ as soon as possible. And share in on the 3 stage of the bootcamp.

Good project, it is project, that:

- You can add to your portfolio / resume;
  - So, it is implemented good enough;
  - Shows your skills;
- Useful for someone;
  - So, you spent your time usefully.

Discuss your project idea with mentor/mentors/group in the bootcamp chat.

You can write down your project idea in some **public** document. For example, in someone from these:

- `README.md`-like file in [Gist](https://gist.github.com/);
  - For example, name it `rust_bootcamp_capestone_project.md`.
- In [Google Docs](https://docs.google.com/).

Don't use repository at beginning. Because you don't know yet what you will implement.

Send it to the bootcamp chat. (See details in the delivery section)

Members of the chat can review your project idea and give you feedback.

Your _project idea_ must be approved by mentor from your subgroup. (Details in the delivery section.)

You will interact with this mentor during the project.

### Details

#### What to use in the project?

Consider this project as an opportunity to learn and grow your skills in Rust.

Challenge yourself by exploring new Rust features, libraries, or patterns, and incorporate them into your project where
appropriate.

It is recommended to use existing libraries and frameworks to speed up development. Don't reinvent the wheel, if it's
possible.

But don't use something "just because". Use it, if it is really needed for project.

#### Domain

It is recommended to choose a project idea that you are passionate about or interested in. This will help you stay
motivated and engaged throughout the project.

If it's a domain you're already familiar with, you'll be able to focus on learning Rust rather than learning the domain.
So, it will be helpful for you.

#### Limited scope and timeframe

Choose a project idea that is suitable for implementation within a **3-day** timeframe. Consider the complexity and
feasibility of the project.

It is better to have a small project that is well implemented than a large project that is half-implemented. You can
improve the project later.

So, implement the most important features first. If you have ideas about extra features, just write it down as TODOs.

It can also be helpful to be explicit about what your project does and what it doesn't.

#### Open-source

This project should be open-source.

It is recommended to choose a project idea that can be shared with the programming community.

This will allow you to:

- receive feedback from other developers;
- improve ecosystem of Rust.

#### Finding a subject for your project

If you're having trouble finding a suitable subject for your project, don't hesitate to ask your mentors for
suggestions.

#### Investigation & Proof-of-concept

It is recommended to make a some research and/or create a proof of concept for questionable parts of the project.

### Delivery

#### What to deliver?

Public document with project idea, accessible by the link.

Document is sent to the bootcamp chat with the message of this format (with tags):

```markdown
#capstone_project_idea #capstone_unapproved
**Title**: <title> (short, up to 100 characters)
**Description**: <description> (relatively short, up to 1000 characters. Remember, that details can be in the document)
**Details url**: <url> (url to your document with details)
```

Tags is used for filtering messages in the chat.

#### Delivery criteria

Project idea is approved by mentor from your subgroup.

Approval is done, at least, by reply to your message in the chat with "+" message. Also, mentor must write a direct
message to you. You will interact with this mentor during the project.

#### Actions after delivery

After approval, you remove `#capstone_unapproved` tag from your message.

---

## Task 6.2: Get approval for the basic implementation from the first mentor

Start to work on your project.

For this, create a public repository at [GitHub](https://github.com/) in the personal namespace (not in the bootcamp
organization).

This repository in the main/master branch should contain, at least:

- `README.md` file with the project description;

Make development in the separate branch.

Open a Pull Request to the main/master branch.

Do not merge it for now.

Focus on designing and developing a minimum viable version of your project, rather than a fully finished product, given
the limited time frame. Don't worry about achieving perfection in your implementation at this stage. Just ensure you get
your mentor's approval on your work.

Send a link to your repository to your project mentor.

Mentor should review your project. Details in the delivery section.

```markdown
#capstone_project #mvp_unapproved
```

Any mentor from the bootcamp should review your project and approve it. (Details in the delivery section.)

### Delivery

#### What to deliver?

Url to the Pull Request.

Url is sent to direct message to the mentor.

#### Delivery criteria

Mentor writes a comment in the Pull Request with text:

```plaintext
MVP approved
```

and left reaction 👀 (eyes) on the top message in the Pull Request.

> Note: Do not merge the Pull Request, for now.

---

## Task 6.3: Get 4 reviews and approval

Teamwork is crucial in our program, as is respecting senior team members and acknowledging the strengths of your peers.

You are required to obtain four reviews of your work. These reviews should incorporate diverse perspectives.

You are required to obtain approval from (at least 1 per category):

- Rust forum
  - https://users.rust-lang.org/c/code-review/11
  - Create a new topic in the "Code Review" category. And ask for a review of your project.
- Mentor from the current bootcamp (outside your immediate subgroup)
- Mentor from the current bootcamp (inside your immediate subgroup)
- Student from the current bootcamp (from any subgroup)

Collect reviews with approvals in comments of the Pull Request. (Details in the delivery section.)

You need to collect proof of these reviews. And sent it as Pull Request in your private repository in the bootcamp
organization. (Details in the delivery section.)

### Delivery

#### What to deliver?

Reviews with approvals in comments of the Pull Request.

> Note: use comments, because comments can be left by any user, without explicit access to the repository. So, it is
> easier to get reviews from the Rust forum.

Comment must be with any "relatively" approved text. Better, with extra data. For example:

- \+
- Ok
- Good to me
- I like it
- etc.

Also, must be a reaction 👍 (like / thumb up) on the comment.

Amount of reviews with approvals must be at least 4.

Collect proofs of these reviews as a screenshots + links to the comments.

Come back your private repository in the bootcamp organization (where you did previous tasks).

Send proofs same way as in the previous tasks. With usage of the current task as a name.

Then, process is the same as in the previous tasks.

#### Delivery criteria

Same as in the previous tasks in organization repository (approval of Pull Request in reviewers section).

---

## Task 6.4: Present your project

In a successful team, members feel comfortable being vulnerable and open to feedback. As such, you are expected to
present your project to your peers and mentors. This will allow you to receive more feedback, further facilitating your
growth and learning.

So, present your project in the bootcamp chat. (Details in the delivery section.)

You can collect feedback from the chat.

> Note: You also can leave a feedback to other projects. It can be helpful for you and for the author of the project.
> Comments, proposals for improvements, etc.

Congrats! At this stage, you already have a minimum portfolio.

### Delivery

#### What to deliver?

Message in the bootcamp chat. Message must contain:

- tag `#capstone_project_demo`;
- Short description of the project;
- Url to the repository;

It also can contain:

- Video with the demo of the project;
  - You can use [OBS Studio](https://obsproject.com/) for recording;
  - You can publish it on [YouTube](https://www.youtube.com/) with access by the link. Or just send it as a file.

#### Delivery criteria

At least 1 any reaction on your message in the chat.
