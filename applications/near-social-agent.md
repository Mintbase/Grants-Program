# Near Social AI Agent Grant Proposal

- **Project Name:** NEAR Social AI Agent
- **Team Name:** Teckas Technologies Private Limited
- **Payment Address:** monish016.near
- **[Level](../README.md#level_slider-levels):** 1


## Project Overview :page_facing_up:

### Overview

- Tagline: "Seamlessly connect to the NEAR Social ecosystem from your wallet."
- Description: The NEAR Social AI Agent enables users to create profiles on near.social, make posts, and read profiles directly from the Bitte wallet. This integration simplifies social interactions within the NEAR ecosystem.
- Integration with Mintbase / NEAR ecosystem: Our project utilizes the NEAR blockchain to facilitate social interactions, enhancing user engagement and community building within the NEAR ecosystem.
- Team's interest: We are committed to enriching the NEAR community experience by leveraging AI technology to make social interactions more accessible and engaging.

### Project Details

- **AI Agent link:** [NEAR Social AI Agent](https://tinyurl.com/near-social-ai-agent)
- **Data models / API specifications of the core functionality:** The NEAR Social AI Agent will utilize the following APIs:

  - **Endpoint:** `/api/profile`
    - **Method:** POST
    - **Summary:** Upload profile details
    - **Description:** Uploads collected profile details along with user's accountId.
    - **Request Body:**
      - **accountId:** User's account Id (string).
      - **name:** Name of the user (string).
      - **about:** Brief description about the user (string).
      - **twitter:** Twitter Id of the user (string).
      - **github:** Github Id of the user (string).
      - **telegram:** Telegram Id of the user (string).
      - **website:** Personal website link of the user (string).
      - **tags:** Tags relevant to the user (array of strings).
      - **profileImage:** The URL of the profile image (string).
      - **bannerImage:** The URL of the banner image (string).
    - **Responses:**
      - **200:** Profile data uploaded successfully, returning a profileUrl.

  - **Endpoint:** `/api/post`
    - **Method:** POST
    - **Summary:** Create a post
    - **Description:** Allows users to post content with their accountId.
    - **Request Body:**
      - **accountId:** User's account Id (string).
      - **content:** Content of the post (string).
      - **imageUrl:** The URL of the post image (string).
    - **Responses:**
      - **200:** Post created successfully, returning a postUrl.

- **An overview of the technology stack to be used:** The technology stack includes OpenAI and Next.js.
- **What your project is not or will not provide or implement:** The agent will focus solely on interactions with near.social and will not cover other social platforms.

### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- **Where and how does your project fit into the ecosystem?** The NEAR Social AI Agent enhances social engagement by providing a direct link between users' wallets and the NEAR social platform, simplifying profile management and content creation.
- **Who is your target audience?** Users of the NEAR ecosystem who wish to engage socially while managing their profiles and posts from their wallet.
- **What need(s) does your project meet?** The agent addresses the need for streamlined social interactions in the NEAR ecosystem, allowing users to manage their social presence conveniently.
- **Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?** While there may be social interaction tools, this project uniquely integrates wallet functionality with social features, enhancing user convenience.

## Team :busts_in_silhouette:

### Team members

- **Team leader:** Varatharaj Kanniyappan
- **Team members:** Sathish, Monish, Immanuel John

### Contact

- **Contact Name:** Varatharaj Kanniyappan
- **Contact Email:** info@teckastechnologies.com
- **Website:** [Teckas Technologies](https://teckastechnologies.com)

### Legal Structure

- **Registered Address:** 2/115.2, Kelaputhukudi Kaspa, Ammanpuram, Tuticorin District, Tamilnadu - 628201, India
- **Registered Legal Entity:** Teckas Technologies Private Limited

### Team's experience

Our team has a strong background in blockchain development and AI technology. We are experienced in integrating applications with blockchain platforms, including previous successful projects in the NFT space.

### Team Code Repos

- [Teckas Technologies GitHub](https://github.com/Teckas-Technologies)

### Team LinkedIn Profiles

- [Varatharaj Kanniyappan](https://www.linkedin.com/in/varatharaj-k-680505141/)
- [Immanuel John](https://www.linkedin.com/in/immanueljohnprofile/)
- [Monish](https://www.linkedin.com/in/monish016/)
- [Sathish](https://www.linkedin.com/in/sathish-ms/)

## Development Status :open_book:

The NEAR Social AI Agent is fully developed and operational. It allows users to create profiles on near.social, make posts, and read profiles directly from the Bitte wallet.

### Key Features:
- Users can create and manage their profiles on near.social.
- Users can create posts and interact with content directly from the wallet.

The development is complete, and we welcome feedback and suggestions for future enhancements.

### Milestones and Grant Amount:
- **Milestone 1:** Completed functionalities include creating profiles, reading profiles, and creating posts.
- **Milestone 2:** To be developed after payment for release 1, which will include getting the last uploaded post, putting comments, reposting, following users, and getting notifications.

**Total Grant Requested:** 5,000 USD  
- **Milestone 1:** 3,000 USD  
- **Milestone 2:** 2,000 USD  

## Future Plans

In the short term, we aim to gather user feedback to refine the agent’s functionalities and ensure a user-friendly experience. We are committed to enhancing the user experience based on community input.

### Long-term Plans:
- **Feature Expansion:** Implement the functionalities outlined in Milestone 2, enhancing social interactions within the NEAR ecosystem.
- **Community Engagement:** Organize events and initiatives to promote the NEAR Social AI Agent and encourage user participation.
- **Continuous Improvement:** Regularly update the agent based on user feedback and emerging trends in the NEAR ecosystem.

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Mintbase Website

We have previously developed projects related to AI and blockchain and are excited to continue contributing to the NEAR ecosystem through the NEAR Social AI Agent. We believe this project will significantly enhance social interactions within the community.

**App Link:** [NEAR Social AI Agent](https://tinyurl.com/near-social-ai-agent)
