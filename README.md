# Mintbase Grants Program<!-- omit in toc -->

<p align="center">
  <img src="src/Grants2-template-1600x900.png" style="width:1300px";>
</p>


- [:wave: Introduction](#wave-introduction)
  - [Guidelines](#guidelines)
  - [Project Ideas](#project-ideas)
  - [Support](#support)
  - [Team](#team)
- [:level_slider: Levels](#level_slider-levels)
- [:pencil: Process](#pencil-process)
  - [1. Application](#1-application)
  - [2. Application Review](#2-application-review)
  - [3. Milestone Delivery and Payment](#3-milestone-delivery-and-payment)
  - [Changes to a Grant after Approval](#changes-to-a-grant-after-approval)
- [:mailbox_with_mail: Suggest a Project](#mailbox_with_mail-suggest-a-project)
- [:bulb: Help](#bulb-help)
  - [Additional information](#additional-information)
  - [Real-time conversation](#real-time-conversation)
- [:rocket: Alternative Funding Sources](#rocket-alternative-funding-sources)
  - [Treasury](#treasury)
  - [Hackathons](#hackathons)

## :wave: Introduction

Cohort 2 Applications are now being reviewed!
  
As part of our commitment to always improving the Mintbase + NEAR ecosystem, we offer a comprehensive grants program focused on funding software development and research efforts related to NEAR protocol NFT infrastructure, **Mintbase, [MintbaseJS](https://github.com/Mintbase/mintbase-js) and our [GraphQL Indexer API](https://docs.mintbase.io/dev/getting-started)**.

 Thanks to the [Web3 Foundation's Grants Program](https://github.com/w3f/Grants-Program) for providing the structural outline for this program.


### Guidelines

Anyone is welcome to apply for a grant. Projects funded through our programs are broad in scope, but our focus lies on strong **technical** projects that add value to the ecosystem.

Projects are more likely to be accepted if:

- Your application is **rich in technical details** and well-defined.
- It presents a **well-researched** or tested concept, for which you are able to show some prior work.
- You can demonstrate that the project will be **maintained** after completion of the grant, be it through an obvious commitment to the technology from your side, additional funding sources or an existing business model.
- Your team has **proven experience** with the relevant technologies and/or a strong technical background. You will be asked to provide the GitHub profiles of your team members as part of your application, which we will examine these for past activity and code quality. You can also link to supporting examples on other platforms.
- You can clearly present how your project stands out among competitors or implements technology that doesn't exist in the ecosystem yet.

Additionally, projects **must** fulfill the following requirements:

- All code produced as part of a grant must be **open-sourced**, and it must also not rely on closed-source software for full functionality. We prefer Apache 2.0 but GPLv3, MIT or Unlicense are also acceptable.
- Projects may not encourage gambling or illicit activity in any shape or form. 
- Projects cannot have been part of a successful token sale.
- Teams must finish projects that have been awarded a grant before being considered for another.


In addition to the information provided on your application, note that your project will need to comply with our [Guidelines for Milestone Deliverables](docs/milestone-deliverables-guidelines.md). In particular, we require all projects to create documentation that explains how their project works. At a minimum, _written_ documentation is required for funding. Tutorials and videos are also helpful to understand your project goals and use cases. 

Please also observe our [Announcement Guidelines](docs/announcement-guidelines.md) for grant-related communications.

Finally, we take licensing and the right of all teams in and outside the ecosystem to be recognized for their work very seriously. Using others' work with no attribution or indication that this was not your own work as part of a milestone delivery **will lead to immediate termination**. Please reach out to us before submitting if you have any doubts on how to comply with a specific license and we'll be happy to help.

We also try to enforce our [code of conduct](CODE_OF_CONDUCT.md) and reserve the right to [block users](https://github.blog/2016-04-04-organizations-can-now-block-abusive-users/) if necessary.

### Project Ideas

Not sure where to start? Take a look at [this list of ideas](https://docs.mintbase.io/dev/build-ideas). 

[Requests For Proposals](rfps) (RFPs) represent concrete ideas for projects that we would like to see implemented. Several teams may apply for the same RFP, so even if another team has already applied to implement a certain RFP, we invite you to do the same if you're interested.

Finally, you don't need to start your own project in order to be eligible for a grant. Instead, some teams choose to port existing work to Mintbase, where the pertinent licenses allow, or even to contribute to an existing open-source project. In the latter case, you should check in advance that the project's maintainers are interested in your contribution, and the acceptance of the milestones will generally be tied to the inclusion of your work in said project.

If you would like feedback on your project idea, feel free to contact us on [telegram](https://t.me/mintgrants)

### Support

The scope of our Grants Program consists of funding and feedback on delivered milestones. This means that we do not provide hands-on support as part of a grant, but if you face specific issues during development, we will do our best and try to direct you to the correct resources. You can find general documentation and more information on Mintbase on the [Mintbase Developer Hub](https://docs.mintbase.io/getting-started), and we encourage you to join the [community](https://t.me/mintdev) in order to get help with specific issues and stay up to date with the most recent developments.

For questions about the grants program itself, see our [FAQ](docs/faq.md#frequently-asked-questions).

### Team

#### Mintbase Grants Committee<!-- omit in toc -->

The committee consists of individuals who know the funding priorities of the Mintbase ecosystem, and is responsible for evaluating grant applications and providing feedback on these.

In cases where a niche expert opinion is desirable, one of the committee members may request such a review.

- [Till](https://github.com/tifrel)
- [Rui Santiago](https://github.com/sainthiago)
- [Luís Freitas](https://github.com/microchipgnu)
- [Nate Geier](https://github.com/nategeier)
- [Paul Kuveke](https://github.com/paul-kuveke)
- Luis Infante
- [Carolin](https://github.com/caromintbase)
- [Marcelo](https://github.com/marcelomintbase)


## :level_slider: Levels

The Mintbase Grants Program offers different grant levels to help you best depending on your current stage.

USD equivalent payments are made in NEAR.
### :hatching_chick: Level 1 (= InstaGrants)<!-- omit in toc -->

- **Target:** Individuals & small teams
- **Amount:** Up to $10,000
- **Requirements:** 2 approvals

### :baby_chick: Level 2<!-- omit in toc -->

- **Target:** Small teams/start-ups
- **Amount:** Up to $50,000
- **Requirements:** 3 approvals and one pitch to Mintbase Council

### :rooster: Level 3<!-- omit in toc -->

- **Target:** Companies/foundations with a proven track record
- **Amount:** Unlimited
- **Requirements:** Pitch call + 5 approvals (for >$100k: Mintbase Council approval)

## :pencil: Process

> **:loudspeaker:** The application process is the same regardless of the [level](#level_slider-levels) being applied for. For ideas please refer to approved submissions from the previous year. 

### 1. Application

   1. [Fork](https://github.com/Mintbase/Grants-Program/fork) this repository.
   2. In the newly created fork, create a copy of the application template ([`applications/application-template.md`](applications/application-template.md)). If you're using the GitHub web interface, you will need to create a new file and copy the [contents](https://raw.githubusercontent.com/mintbase/Grants-Program/main/applications/application-template.md) of the template inside the new one. Make sure you **do not modify the template file directly**. In the case of a maintenance application, use the maintenance template ([`maintenance template`](maintenance/maintenance-template.md)) instead. 
   3. Name the new file after your project: `project_name.md`.
   4. Fill out the template with the details of your project. The more information you provide, the faster the review. Please refer to our [Grant guidelines for most popular grant categories](docs/grant_guidelines_per_category.md) and make sure your deliverables present a similar same level of detail. If you're only applying for a smaller grant that only consists of, say, UI work, you don't need to provide as much detail.
   5. Once you're done, create a pull request. The pull request should only contain _one new file_—the Markdown file you created from the template.
   6. You will see a comment template that contains a checklist. You can leave it as is and tick the checkboxes once the pull request has been created. Please read through these items and check all of them.
   7. Sign off on the [terms and conditions](docs/T&Cs.md) presented by the [CLA assistant](https://github.com/claassistantio) bot as a Contributor License Agreement. You might need to reload the pull request to see its comment.

### 2. Application Review

   1. The team can (and usually does) issue comments and request changes on the pull request.
   2. Clarifications and amendments made in the comments _need to be included in the application_. You may address feedback by directly modifying your application and leaving a comment once you're done. Generally, if you don't reply within 2 weeks, the application will be closed due to inactivity, but you're always free to reopen it as long as it hasn't been rejected.
   3. When all requested changes are addressed and the terms and conditions have been signed, someone will mark your application as `ready for review` and share it internally with the rest of the committee.
   4. The application will be accepted and merged as soon as it receives the required approvals (see [levels](#level_slider-levels)), or closed after a rejection vote. Unless specified otherwise, the day on which it is accepted will be considered the starting date of the project and will be used to estimate delivery dates.

### 3. Milestone Delivery and Payment
   1. The mintbase team will create a telegram group with approved teams.
   2. When you are ready to submit completion for a milestone, please open a PR on that adds submissions/your_project_milestone_YYYY-MM-DD.md. Use the submissions/delivery guidelines.md content as the template for your submission documentation.
   3. Share a link to the PR in your private telegram group for review and further instructions on invoicing and receiving payment.


### Changes to a Grant after Approval

- Accepted grant applications can be amended at any time. However, this _necessitates a reevaluation by the committee_ and the same number of approvals as an application (according to the [levels](#level_slider-levels)). If your application has been accepted and, during development, you find that your project significantly deviates from the original specification, please open a new pull request that modifies the existing application. This also applies in case of significant delays.
- If your _delivery schedule_ significantly changes, please also open a pull request with an updated timeline.
- If your deliveries are significantly delayed and we cannot get a hold of you, we will terminate the grant (3 approvals required, regardless of level. If a member of the committee creates the termination PR, only 2 more approvals are required).

## :mailbox_with_mail: Suggest a Project

If you think that we should support the development of certain tools or projects that aren't in the Mintbase [tech stack](https://docs.mintbase.io/dev/getting-started/our-stack), feel free to submit a suggestion using the process described below. We are particularly interested in supporting projects that could be leveraged by other builders in our ecosystem.

**Submit an idea:**

If you have an idea for a project or would like to highlight an area in which you'd like to see teams build, but lack the technical background to create a detailed outline, you're welcome to open an [issue](https://github.com/Mintbase/Grants-Program/issues/new). We will review your suggestion and, if necessary, will create an RFP based on it and reach out to teams able to build it.

**Submit an RFP (Request for Proposals):**

Ideas generally have better chances of being implemented if they're presented in a project outline format that can be picked up straight away by a team, so if you have a good concept of the milestones required to bring your project to life, you can follow the process below and directly submit an RFP:

1. [Fork](https://github.com/Mintbase/Grants-Program/fork) this repository.
2. In the newly created fork, create a copy of the suggestion template ([`rfps/suggestion-template.md`](rfps/suggestion-template.md)) inside the [`rfps`](rfps) folder. Make sure you create a new file and copy the [contents](https://raw.githubusercontent.com/mintbase/Grants-Program/master/rfps/suggestion-template.md) of the template into the new one, and _do not modify the template file directly._
3. Name the file after your idea: `project_name.md`.
4. Fill out the template with the project details. Please include as many details as possible.
5. Once you're done, create a pull request. The pull request should only contain _one new file_—the Markdown file you created from the template.
6. You will see the same template as for creating an application. Please replace it with [this one](.github/PULL_REQUEST_TEMPLATE/rfp_pr_template.md).

## :bulb: Help

### Additional information

| <img src="src/web.png?s=50" width="50"> | <img src="src/twitter.png?s=50" width="50"> | <img src="src/medium.png?s=50" width="50"> | <img src="src/like.png?s=50" width="50"> | <img src="src/reddit.png?s=50" width="50"> |  
| :-: | :-: | :-: | :-: | :-: |
| [Mintbase Website](https://mintbase.io) | [Mintbase Twitter](https://twitter.com/mintbase) | [Mintbase Medium](https://medium.com/mintbase) | [Docs](https://docs.mintbase.io) | [Mintbase Reddit](https://www.reddit.com/r/mintbase) 

### Real-time conversation

We have Telegram channels for real-time discussions on Mintbase. Join the conversation.

- [Mintbase Grants Chat](https://t.me/mintgrants)
- [Mintbase Open](https://t.me/mintbase)


## :rocket: Alternative Funding Sources

Some funding sources might be more and some less suitable for your project throughout its various stages. We encourage you to explore alternative funding options listed below. Please note, however, that you should not seek to fund the **same scope of work** from multiple sources and that any team found doing so will have its Mintbase support terminated.

### Treasury

The treasury is a pot of on-chain funds collected through transaction fees, slashing, staking inefficiencies, etc. The funds held in the treasury can be spent on spending proposals. [Mintbase](https://mintbase.io) offer everyone the opportunity to apply for funding via the treasury. See:

- [Mintbase Treasury Account](https://nearblocks.io/address/mintbasegrants.near#transaction)
`mintbasegrants.near`

### Hackathons

From time to time, Mintbase and/or NEAR organize hackathons to promote quick prototyping of Mintbase related ideas. We highly encourage you to participate in these hackathons. Bear in mind, however, that you cannot submit the **same work** for a hackathon and the Grants Program. If you have worked or are planning to work on a project for a hackathon, your grant application should either propose a different set of features or otherwise build on top of your hackathon work. The same applies in reverse, although that will likely be less common.

The best way to learn about upcoming hackathons is by following Mintbase on Telegram and Twitter.


## :information_source: License<!-- omit in toc -->

[Apache License 2.0](LICENSE) © Mintbase


