# PotLock AI Agent Grant Proposal


- **Project Name:** PotLock AI Agent
- **Team Name:** Teckas Technologies Private Limited
- **Payment Address:** monish016.near
- **[Level](../README.md#level_slider-levels):** 1


## Project Overview :page_facing_up:

### Overview

- Tagline: "Empowering users to easily donate and raise funds on PotLock with AI assistance."
- Description: The PotLock AI Agent streamlines the user experience on the PotLock platform by providing an intelligent interface to view all listed projects, donate to projects, and create new project profiles. Users can generate donation transactions, view all their past contributions, and track donations received for their own projects, all through the AI agent.
- Integration with Mintbase / NEAR ecosystem: This AI agent leverages the NEAR blockchain to securely facilitate donations, project management, and fund tracking for users on PotLock. The integration simplifies user interactions while supporting the NEAR ecosystem's growth.
- Team's interest: We are passionate about blending AI technology with blockchain to democratize art creation and enhance user accessibility.
- **Grant Amount Requested:** 3,000 USD

### Project Details
- **Mockups/designs of any UI components:** [PotLock AI Agent UI Mockups](https://tinyurl.com/potlock-ai-agent)
- **Data models / API specifications of the core functionality:** The PotLock AI Agent utilizes APIs from the Bitte wallet and includes a REST API endpoints:
  - **Endpoint:** `/api/project`
  - **Method:** GET
  - **Summary:** Fetch projects with pagination or view specific project details.
  - **Description:** Retrieves all projects or, if id is provided, fetches a specific project by `registrantId`. Displays all project details with the project name in bold.
  - **Parameters:**
    - **offset:** (integer, default: 0): Starting point for pagination.
    - **id:** Fetches project by the registrant ID (string).
  - **Responses:**
    - **200:** Projects retrieved successfully with a list of projects or details of a single project.

  - **Endpoint:** `/api/project`
  - **Method:** POST
  - **Summary:** Create a new project.
  - **Description:** Allows users to create a project with details like `accountId`, `name`, `categories`, `description`, `reason`, `profileImage` and `bannerImage`.
  - **Request Body:**
    - **accountId:** The account ID of the project creator (string).
    - **name:** The name of the project (string).
    - **categories:** List of project categories (array).
    - **description:**  Detailed description of the project (string).
    - **reason:** Reason for the project being a public good (string).
    - **profileImage:** URL for the project's profile image (string).
    - **bannerImage:** URL for the project's banner image (string).
  - **Responses:**
    - **200:** Project creation URL generated successfully.

  - **Endpoint:** `/api/donation`
  - **Method:** POST
  - **Summary:** Make a donation.
  - **Description:** Allows users to make a donation by specifying `recipientId`, `amount`, and an optional `message`.
  - **Request Body:**
    - **recipientId:** The account ID of the recipient (string).
    - **amount:** Donation amount in NEAR tokens (string).
    - **message:** Optional message for the donation (string).
  - **Responses:**
    - **200:** Donation transaction URL generated successfully.

  - **Endpoint:** `/api/donation/recipient`
  - **Method:** GET
  - **Summary:** Fetch donations received by the user.
  - **Description:** Retrieves donations received by the logged-in user, with pagination support.
  - **Parameters:**
    - **recipientId:** Logged-in user's account ID (string).
    - **offset:** Offset for pagination. (string).
  - **Responses:**
    - **200:** Donations retrieved successfully with a list of received donations.

  - **Endpoint:** `/api/donation/donor`
  - **Method:** GET
  - **Summary:** Fetch donations made by the user.
  - **Description:** Retrieves donations made by the logged-in user, with pagination support.
  - **Parameters:**
    - **donorId:** Logged-in user's account ID (string).
    - **offset:** Offset for pagination. (string).
  - **Responses:**
    - **200:** Donations retrieved successfully with a list of received donations.

- **An overview of the technology stack to be used:** The technology stack includes OpenAI and Next.js.
- **Documentation of core components, protocols, architecture, etc. to be deployed:** No additional documentation will be provided.
- **PoC/MVP or other relevant prior work or research on the topic:** The PotLock AI Agent is fully developed and operational, demonstrating successful integration with the Bitte wallet.
- **What your project is not or will not provide or implement:** The agent currently supports PotLock app functionalities like create new project, view all listed projects, view all donations made by the user and received by the project, make the donation for approved projects only through the Bitte wallet and does not integrate with other wallet types or platforms.

### Ecosystem Fit

Help us locate your project in the Mintbase landscape and what problems it tries to solve by answering each of these questions:

- **Where and how does your project fit into the ecosystem?** The PotLock AI Agent enhances the fundraising ecosystem on NEAR by providing users with a seamless way to interact with projects, donate, and manage their own fundraising efforts. It lowers the barrier for both donors and project creators, expanding NEAR's utility.

- **Who is your target audience?** PotLock app users, including project creators seeking donations, and donors looking to support blockchain-based initiatives.

- **What need(s) does your project meet?** The AI agent simplifies donation management, project creation, and tracking of received funds, making it easier for users to engage with the PotLock app.

- **Are there any other projects similar to yours in the Mintbase / NEAR ecosystem?** No other projects currently offer an AI-powered agent for donation management and project creation on NEAR, giving PotLock a unique edge in this space.

## Team :busts_in_silhouette:

### Team members

- **Team leader:** Varatharaj Kanniyappan
- **Team members:** Sathish, Monish, Immanuel John, Johnson

### Contact

- **Contact Name:** Varatharaj Kanniyappan
- **Contact Email:** info@teckastechnologies.com
- **Website:** [Teckas Technologies](https://teckastechnologies.com)

### Legal Structure

- **Registered Address:** 2/115.2, Kelaputhukudi Kaspa, Ammanpuram, Tuticorin District, Tamilnadu - 628201, India
- **Registered Legal Entity:** Teckas Technologies Private Limited

### Team's experience

Our team has extensive experience in developing blockchain solutions and integrating AI technologies. We have successfully delivered several AI-powered projects that enhance user experience by seamlessly combining AI with blockchain functionality. Our expertise includes developing smart contract-based applications, AI-driven user interfaces, and decentralized solutions that simplify complex workflows and promote user engagement. We are committed to building innovative, scalable solutions that align with the evolving needs of the blockchain ecosystem.

### Team Code Repos

- [Teckas Technologies GitHub](https://github.com/Teckas-Technologies)

### Team LinkedIn Profiles

- [Varatharaj Kanniyappan](https://www.linkedin.com/in/varatharaj-k-680505141/)
- [Immanuel John](https://www.linkedin.com/in/immanueljohnprofile/)
- [Monish](https://www.linkedin.com/in/monish016/)
- [Sathish](https://www.linkedin.com/in/sathish-ms/)
- [Johnson](https://www.linkedin.com/in/johnsonibl/)

## Development Status :open_book:

The PotLock AI Agent is fully developed and operational. It successfully integrates with the Bitte wallet, allowing users to view projects, make donations, view all the donations, and create new project profiles.

### Key Features:
- Users can view all listed projects on PotLock.
- View details about specific project.
- Donate NEAR to approved projects with a sign transaction URL.
- Create new project profiles for fundraising.
- Track donations made and received.

The development is complete, and we are open to feedback and any future modifications to enhance the user experience and functionality of the PotLock AI Agent.

## Future Plans

In the short term, we intend to gather user feedback to identify areas for improvement and additional features for the PotLock AI Agent. We are committed to enhancing the user experience based on community input.

### Long-term Plans:
- **Feature Expansion:** We plan to add further personalization options, such as recommended projects based on donation history.
- **Community Engagement:** Organize regular donation events and campaigns to increase platform usage.
- **Continuous Improvement:** Keep updating the AI agent to ensure it meets evolving user needs and keeps pace with blockchain and AI trends.

## Additional Information :heavy_plus_sign:

**How did you hear about the Grants Program?** Mintbase Website

We are excited about the potential of the PotLock AI Agent and its ability to simplify fundraising and donation management on NEAR. We look forward to contributing to the Mintbase ecosystem through this project.

If there are any additional contributions or support from other teams, we will mention them here.
