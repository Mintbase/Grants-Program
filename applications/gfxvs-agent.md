# Mintbase Grant Proposal

> This document will be part of the terms and conditions of your agreement and therefore needs to contain all the required information about the project. Don't remove any of the mandatory parts presented in bold letters or as headlines. Blockquote sections starting with a `>` (such as this one) can be removed.
>
> See the [Grants Program Process](https://github.com/Mintbase/Grants-Program/#pencil-process) on how to submit a proposal.

- **Project Name:** GFXvs AI Agent
- **Team Name:** Teckas Technologies Private Limited
- **Payment Address:** monish016.near
- **[Level](../README.md#level_slider-levels):** 1

> ⚠️ *The combination of your GitHub account submitting the application and the payment address above will be your unique identifier during the program. Please keep them safe.*

## Project Overview :page_facing_up:

### Overview

- Tagline: "Empowering creativity with AI-generated art in your wallet."
- Description: The GFXvs AI Agent enables users to generate images directly within the Bitte wallet and submit them to the GFXvs upcoming battle arts queue. This tool enhances the NFT experience by simplifying the art creation and submission process.
- Integration with Mintbase / NEAR ecosystem: Our project leverages the NEAR blockchain to securely generate and manage AI art submissions, fostering community engagement in the NFT space.
- Team's interest: We are passionate about blending AI technology with blockchain to democratize art creation and enhance user accessibility.
- **Grant Amount Requested:** 3,000 USD

### Project Details

We expect the teams to already have a solid idea about your project's expected final state. Therefore, we ask the teams to submit (where relevant):

- **Mockups/designs of any UI components:** [GFXvs AI Agent UI Mockups](https://tinyurl.com/gfxvs-art-ai-agent)
- **Data models / API specifications of the core functionality:** The GFXvs AI Agent utilizes APIs from the Bitte wallet and includes a REST API endpoint for uploading art:
  - **Endpoint:** `/api/art`
  - **Method:** POST
  - **Summary:** Upload image data
  - **Description:** Uploads image data including `colouredArt`, `arttitle`, and `artistId`.
  - **Request Body:**
    - **colouredArt:** The URL of the image (string).
    - **arttitle:** The title of the image (string).
    - **artistId:** User's account ID (string).
  - **Responses:**
    - **200:** Image data uploaded successfully with details about the art, including `artistId`, `arttitle`, and `colouredArt`.

- **An overview of the technology stack to be used:** The technology stack includes OpenAI and Next.js.
- **Documentation of core components, protocols, architecture, etc. to be deployed:** No additional documentation will be provided.
- **PoC/MVP or other relevant prior work or research on the topic:** The GFXvs AI Agent is fully developed and operational, demonstrating successful integration with the Bitte wallet.
- **What your project is not or will not provide or implement:** The agent currently supports image generation and submission only through the Bitte wallet and does not integrate with other wallet types or platforms.

### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- **Where and how does your project fit into the ecosystem?** The GFXvs AI Agent enhances the NFT marketplace by providing a unique tool for generating and submitting AI-generated art directly within the Bitte wallet. This integration fosters community engagement and streamlines the process of art creation and submission.

- **Who is your target audience?** NFT creators, collectors, and users of the Bitte wallet who are interested in innovative tools for art generation and submission.

- **What need(s) does your project meet?** The agent addresses the need for user-friendly solutions for creating and managing AI-generated art, making it accessible for both experienced artists and newcomers to the NFT space.

- **Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?** While other projects may focus on either AI art generation or wallet integration separately, our solution uniquely combines both functionalities, providing a seamless experience for users.

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

Our team has a strong background in blockchain development and AI technology. We have successfully integrated the GFXvs AI Agent with the Bitte wallet and previously contributed to various projects in the NFT space.

### Team Code Repos

- [Teckas Technologies GitHub](https://github.com/Teckas-Technologies)

### Team LinkedIn Profiles

- [Varatharaj Kanniyappan](https://www.linkedin.com/in/varatharaj-k-680505141/)
- [Immanuel John](https://www.linkedin.com/in/immanueljohnprofile/)
- [Monish](https://www.linkedin.com/in/monish016/)
- [Sathish](https://www.linkedin.com/in/sathish-ms/)

## Development Status :open_book:

The GFXvs AI Agent is fully developed and operational. It successfully integrates with the Bitte wallet, allowing users to generate AI art and submit it to the GFXvs upcoming battle arts queue.

### Key Features:
- Users can create unique AI-generated art using customizable prompts.
- The agent seamlessly uploads generated images to the GFXvs battle arts queue through the `/api/art` endpoint.

The development is complete, and we are open to feedback and any future modifications to enhance the user experience and functionality of the GFXvs AI Agent.

## Future Plans

In the short term, we intend to gather user feedback to identify areas for improvement and additional features for the GFXvs AI Agent. We are committed to enhancing the user experience based on community input.

### Long-term Plans:
- **Feature Expansion:** Explore potential new functionalities, such as advanced customization options for image generation and integration with additional wallet platforms.
- **Community Engagement:** Organize events and contests to promote the use of the GFXvs AI Agent and to foster a vibrant community around AI-generated art.
- **Continuous Improvement:** Regularly update the agent based on user feedback and emerging trends in the NFT and AI art spaces.

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Mintbase Website

We have previously developed projects related to AI and blockchain and are excited to continue contributing to the Mintbase ecosystem. We believe the GFXvs AI Agent will provide significant value to users and enhance the NFT experience.

If there are any additional contributions or support from other teams, we will mention them here.
