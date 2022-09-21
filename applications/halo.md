#Mintbase Grant Proposal

**Project Name:** Project Halo 

**Team Name:** Autonomic 

**Payment Address:** autonomic.near 

##Overview  
 

To bring holographic NFTs to the NEAR ecosystem in collaboration with the Looking Glass Factory and Mintbase. 
 

##Brief description: 
 

By utilising the open-source SDK of the Looking Glass Factory software we are going to add functionality to a cloned version of the Mintbase source-code. The end goal will be to allow the user to create and mint holographic NFTs and then buy and sell them on the NEAR blockchain using NEAR Protocol token. Said holographic NFTs will initially be only visible in true glasses-free holographic 3D using Looking Glass hardware.

 

**An indication of how your project relates to / integrates into the Mintbase / NEAR ecosystem.**
 

This project integrates directly with the Mintbase format as we are using a cloned version of its source-code. The project’s end goal is the full implementation of NEAR Protocol to allow users to buy and sell minted holographic NFTs on the marketplace using said token. This develops the NEAR ecosystem, adds value to the token itself as well as the NEAR Protocol blockchain.
 
**An indication of why your team is interested in creating this project.**
 
Our team are strong supporters of NEAR Protocol and its ecosystem. Our co-founder Michael King first discovered it in mid-2020, this was via researching NFT marketplaces in general, he then discovered PARAS based in Indonesia. Michael got to know the developers very well and started to volunteer his time to assist them with their site’s user journey, functionality, and general user interface.   

Around September 2021 he was asked by them to act as an official representative to attend NEARCON 2021. Sadly,due to other commitments and some global projects we had to finalise in the R&D sector he was unable to attend and represent the company.   

Around this time, he conducted the same exercise for Mintbase, interacting with the developers via their Telegram group. Michael engaged in conversations with ‘Chloethedev’ and helped test an early version of Mintbase and gave a great deal of feedback. This is also the time that he discovered Ref Finance and many other aspects of the NEAR ecosystem, from this point on the team was on a exploration journey and always had planned of developing a project with the NEAR Foundation in one way or another.   

Around April 2022, the team became very invested in the technology sector and saw a tech-demo by the Looking Glass Factory and sometime after this we had the idea to create a platform for holographic NFTs, ideally on the NEAR blockchain. We then began conversations with people from Looking Glass and NEAR to bring this idea to fruition.  

We recently attended NEARCON 2022 and further discussed part of the project with the developers at Mintbase and other influencers of the industry who described project Halo as being a very exciting innovation and a contribution to the current marketplace.  

We want to bring a revolutionary product to the NEAR blockchain. We now have very interested parties from Looking Glass and Mintbase to see this project come into fruition with some great future plans for its further development. The team have been keen followers and studied several aspects of cryptocurrency, blockchain and Web3 for years and we are now in the position where we have found a niche product that can support the community.  

Our technical lead Asim Maqsood Basraa has already conducted a deep-dive of the Looking Glass SDK and the open-source code of Mintbase to ensure the feasibility of making this project happen and we can confirm that the bridge and all of our technical abilities are at the highest capability to ensure the success of this project.

## Project Details 


## High Level requirements: 



1. Creators should be able to upload Holographic NFTs to Holographic NFT Marketplace(HNM) In the process the Creator  should be able to preview and validate the media as LG supported media format. 
2. HNM should be able to mint and burn NFTs using Near Protocol. Creator will be holding rights to the NFT it uploaded to HNM.
3. LG users should be able to purchase the NFT.
4. LG users should be able to load NFT directly to their LG hardware.
5. LG users should be able to download NFT on their system. 


## Technical Implementation details: 



1. Creators uploading Holographic NFT to HNM. 
    1. We integrate open source [StereoPhotoMaker](https://docs.lookingglassfactory.com/community/community-made-tools-and-projects#stereophotomaker-and-web-viewers-from-masuji-suto) into our marketplace which will support uploading various LG supported media formats including 180 degree photos, horizontal shift videos, 3D Models and more. It will require to build a web tool which is more user friendly than StereoPhotoMaker but will be reusing source code from StereoPhotoMaker. 
    2. Plugins for Unity, Unreal, Blender will be updated to upload NFTs directly to HNM.  LG plugins for these tools are open sourced. We will amend plugins so that creators should be able to load Holographic NFT on LG hardware but also can upload to HNM directly. 
2. HNM should be able to mint and burn NFTs using mintbase SDK.
    1. We will use Mintbase SDK and its following functionality to mint the NFT.
    2. For minting the NFT we will follow steps specified in Mintbase SDK and reuse the code provided in examples. 
3. LG users should be able to purchase the Holographic NFT. 
    1. We will use Mintbase SDK and its following functionality to facilitate the NFT purchase.
    2. For selling the NFT we will follow steps specified in Mintbase SDK and reuse the code provided in examples. 
4. LG users should be able to load NFT directly to their LG hardware.
    1. We will integrate [HoloPlayJS](https://docs.lookingglassfactory.com/developer-tools/three) and [HoloPlay CoreJS](https://docs.lookingglassfactory.com/core/corejs) libraries to provide this functionality on HNM. And steps to perform would be 
        1. Once User clicks the “load on my LG button”. 
        2. HNM will check if the LG bridge is installed on the system, if not it will prompt. 
        3. User installs LG bridge and continues the journey of loading the file on LG hardware.
        4. HNM loads the NFT Using HoloPlay CoreJS to LG hardware.
    2. We will also explore how we can secure the NFT on LG hardware by negotiating the identity of LG hardware. There could be two possible ways to achieve this,
        1. Possibly registering user’s LG devices with HNM, and only allow loading NFT to those LG devices.
        2. Seeking first class NFT, smart contract negotiation(like DRM) support from LG. 
5. LG users should be able to download NFT on their system.
    1. HNM will be able to provide copy of Holographic NFT to store on the disk and load it on LG hardware using other LG tools. But it will not be editable. We need to explore how to make it possible. 

## Ecosystem Fit 

**Where and how does your project fit into the ecosystem?**

There already exists a few NFT marketplaces built on NEAR, the most notable being PARAS and Mintbase. We will be adapting the open source-code of Mintbase to develop an evolutionary leap of the NFT marketplace within the NEAR ecosystem. Our final product will align itself alongside PARAS and Mintbase as a NFT marketplace but offer functionality and a format not seen before. Infact, nothing like this currently exists, especially not on NEAR. The fact we are using the Mintbase source-code allows us to effectively fast-track this project from a developmental point of view. We see this as effectively a platform for NEAR creatives, artists, art collectors and it will inevitably attract those who are not currently part of the NEAR ecosystem.

 

**Who is your target audience (parachain/dapp/wallet/UI developers, designers, your own user base, some dapp's userbase, yourself)?** 
 

Existing users of PARAS, Mintbase and anyone interested in NFTs, especially those minted on the NEAR blockchain. As said above, the final version of this platform will undoubtedly attract users from outside the NEAR ecosystem into it. We see the core audience as being young creatives and art collectors as well as those interested in cryptocurrency, technology, holographics, Web3 and the future of ecommerce.



**What need(s) does your project meet? ** \
 

Having monitored the NFT market for the last few years, combined with the recent crypto crash it is safe to say that several of these marketplaces have reached saturation point. The NFT itself was always meant to be a truly unique digital asset minted on a blockchain. With our project there will only ever be ONE of each piece available. Artists will be curated, initially by invite ONLY, their works will be vetted to not infringe upon copyright, this in itself brings a much-needed system of ‘auditing’ to the NFT landscape. Combined with the fact that these minted holographic NFTs will require Looking Glass hardware to be viewed in true glasses-free 3D this adds to the rarity of the product. This is bringing back a true sense of ‘uniqueness’ to the NFT landscape. This in itself will elevate not only NEAR Protocol and the NEAR blockchain but also the entire NFT marketplace across all blockchains.

 

**Are there any other projects similar to yours in the Mintbase / NEAR ecosystem? If so, how is your project different?** 
 

To answer both questions above, yes there are existing NFT platforms within the NEAR ecosystem, none of these implement holographic functionality. Our project represents an evolutionary leap utilising cutting-edge technology and re-establishing the ‘uniqueness’ of blockchain NFTs.

 

## Team

### Team members

**Name of team leader**: Michael King 

**Names of team members:** Joshua Taylor, Asim Maqsood Basraa, Jordan Lewis, Noor Zainab, Hira Saeed, Ammar Farooqi. 

**Additional assistance from:**

Luis Frietas (Mintbase) Developer

Looking Glass technical team

###Contact

**Contact Name:** Michael King 

**Contact Email:** mike@autonomic.ooo 

**Website:** [www.autonomic.ooo](http://www.autonomic.ooo)

### Legal Structure 

**Registered Address:** Address of your registered legal entity, if available:  

128 City Road, London, EC1V 2NX 

**Registered Legal Entity:** Name of your registered legal entity, if available: Autonomic Limited 

## Team's experience 

Please describe the team's relevant experience. If your project involves development work, we would appreciate it if you singled out a few interesting projects or contributions made by team members in the past. For research-related grants, references to past publications and projects in a related domain are helpful. 

**Michael King:** has helped test the NFT platforms Mintbase and Paras and will soon be assisting Ref Finance in a similar way. This project grew from his passion for the NEAR ecosystem, he is also the co-founder of Autonomic and has worked in high-end IT management, corporate banking and creative media. Michael was a data analyst for JPMorgan, Barclays and now bringing his wealth of experience in to Research and Development which brought this project idea into fruition. 

**Joshua Taylor:** Throughout his years, Joshua founded several start-up companies that were mainly in the technology and marketing sector. An experienced managing director, entrepreneur and innovator in various sectors globally with a big passion for technology. Joshua will be ensuring and shaping the project objectives and goals to meet the achieved plan with the timelines set. He will also be working on the future of the project once it has been developed as he wants this to be a functional asset for Mintbase and NEAR where it gives their community the option to own their holographic NFTs.  \
  \
**Jordan Lewis:** Jordan is our senior creative, with a breadth of experience in brand and marketing, working for the likes of Adidas, Universal, ITV, Ocado, TK-Maxx, New Automotive, Tommy Hilffiger, JustEat and many other leading brands. Jordan takes an integrated approach to deliver the right message to the right audiences. He will be developing the predesigned NFTs for testing in the necessary formats and shaping the brand identity of the project, during the process and after.  \
  \
**Asim Maqsood:**  Asim is seasoned technologist with 18 years of experience working with tech giants like Cisco, Mentor Graphics, Sky, Channel 4. He is also selected by TopTal which only lists the top 3% of engineers on their panel. During his lengthy career in software he has worked on all levels of software from writing firmwares to developing multiscreen apps. With his over a decade experience in integrating end-to-end software components in challenging projects like Sky Q and his wealth of knowledge in Image/Video processing SDKs, he is a great fit for leading the technical aspect of this project.  

His tech team includes **Noor Zainab**(Full stack developer with experience in in working on production systems like TelkomSel and few more), Hira Saeed (Frontend engineer with experience in leading team of frontend developers at MaxStream.tv) and **Ammar Farooqi**(Backend Engineer with experience in working on production systems like TelKomSel, StarHub and few more). 

Team LinkedIn Profiles (if available) 

[https://www.linkedin.com/in/mike-king-autonomic/](https://www.linkedin.com/in/mike-king-autonomic/)  (Michael King) 

[https://www.linkedin.com/in/joshuadelanotaylor/](https://www.linkedin.com/in/joshuadelanotaylor/) (Joshua Taylor) 

[https://www.linkedin.com/in/maqsood/](https://www.linkedin.com/in/maqsood/) (Asim Maqsood Basraa) 

[https://www.linkedin.com/in/jordanrhys/](https://www.linkedin.com/in/jordanrhys/) (Jordan Lewis) 

[https://www.linkedin.com/in/noor-zainab-b8760884/](https://www.linkedin.com/in/noor-zainab-b8760884/) (Noor Zainab)

[https://www.linkedin.com/in/hira-saeed-b27957a9/](https://www.linkedin.com/in/hira-saeed-b27957a9/) (Hira Saeed)

[https://www.linkedin.com/in/ammarfarooqi/](https://www.linkedin.com/in/ammarfarooqi/) (Ammar Farooqi)

## Development Status

### Development Roadmap

#### Phase 1:

**Duration:** 6 months

**Required Grant:** 50,000 USD


<table>
  <tr>
   <td>#
   </td>
   <td><strong>Milestones</strong>
   </td>
   <td><strong>Details</strong>
   </td>
  </tr>
  <tr>
   <td>1
   </td>
   <td>Web app
   </td>
   <td>Web app to upload, view, validate holographic NFT and load it in web viewer. Integrating into Mintbase Marketplace template
   </td>
  </tr>
  <tr>
   <td>2
   </td>
   <td>Web app
   </td>
   <td>Creator onboarding
   </td>
  </tr>
  <tr>
   <td>3
   </td>
   <td>Backend Docker
   </td>
   <td>Writing server to mint NFTs and assign ownership of the NFT to creator. 
   </td>
  </tr>
  <tr>
   <td>4
   </td>
   <td>Web app, Backend Docker
   </td>
   <td>Customer onboarding, Wallet creation etc
   </td>
  </tr>
  <tr>
   <td>5
   </td>
   <td>Web app, BackendDocker 
   </td>
   <td>Purchase journey for Holographic NFT
   </td>
  </tr>
</table>


**Deliverable for phase 1:** 

Creators can upload Holographic NFT and users can purchase it.  

### Phase 2:

**Duration:** 6 months

**Required Grant:** 50,000 USD


<table>
  <tr>
   <td>#
   </td>
   <td><strong>Milestones</strong>
   </td>
   <td><strong>Details</strong>
   </td>
  </tr>
  <tr>
   <td>1
   </td>
   <td>Web app, Installation scripts
   </td>
   <td>Loading NFT from web to LG glass
   </td>
  </tr>
  <tr>
   <td>2
   </td>
   <td>LG plugin
   </td>
   <td>LG plugins modifications to allow uploading NFTs straight from 3D rendering tools.  At least 2 tools to be supported
   </td>
  </tr>
  <tr>
   <td>3 
   </td>
   <td>Web tool
   </td>
   <td>Support in for existing NFT conversion to Holographic NFT. Also support for converting uploaded images to Holographic NFTs.
   </td>
  </tr>
  <tr>
   <td>4
   </td>
   <td>Final system
   </td>
   <td>Integrating/testing/fixing all and delivering final working system
   </td>
  </tr>
</table>


Deliverable for phase 2: 

Users can load NFTs on LG hardware. Creators can upload their creatives straight from their rendering tools. And creators can convert various media types to its 3D  holographic version.

Hardware Requirements

Hardware required:  



* 5 x LG $399.00 units = $1995 
* 1 x Holographic monitor = $3000 




### Future Plans
 

Short-term we will continually be testing and developing the platform, depending on the funding scale and success we may also seek additional investors or funding should we need it. With the full team of Autonomic behind the project, promoting our continued success will not pose any problems. Though we only intend to promote/market the project once it is complete.  
 

**The team's long-term plans and intentions in relation to it**.  
 

Long Term plans are once the development is complete, is to launch the marketplace at NEARCON 2023. Whereby users will be able to buy, sell and mint holographic NFTs on our new store front using NEAR Protocol anywhere in the world.

**Additional Information**

How did you hear about the Grants Program? 
Mintbase Website 

Here you can also add any additional information that you think is relevant to this application but isn't part of it already, such as: 

Work you have already done.

If there are any other teams who have already contributed (financially) to the project: N/A 

Previous grants you may have applied for: N/A 
