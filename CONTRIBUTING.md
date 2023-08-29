# Contributing to Oracle Engine

First, thank you for being interested in contributing to Oracle Engine! We aim to make it possible for anyone that wants to help contribute to this project, including code free solutions such as feedback and questions.

## Have a Question, Feedback, or Idea? 

Our discord server is a perfect place to ask questions or to provide us feedback! You can join our [Discord Server](https://discord.gg/dzW2DZBBQH) and search for your question in the support thread. If you do not see your question asked, please feel free to open a new thread and we will address it when possible. If you have any feedback or ideas, the text channel ideas-and-feedback is the home for that.

### Opening an issue

Before you open issue, please look in our discord support thread and see if your issue has been addressed there. 

### Writing Good Issues / Support Threads

Keep issues to a single problem or request. Do not put multiple bugs, problems, requests into the same issue. The more information you can provide, the more likely it is that we can address it faster and more accurately. If the issue is able to be reproduced, please list out the exact steps taken to cause the issue to happen.

Example of good information for an issue:

* Browser used
* Extensions installed
* Steps to reproduce the issue
* What did you expect to see versus what you see
* Screenshots / gifs
* Dev console output 

## Want to Contribute Via Code?

Please fork the [d2foundry/oracle_engine](https://github.com/d2foundry/oracle_engine) repo and create a branch from there to hold your changes. Once you have finalized your changes, push to your remote, open an issue and pull request on our repo, and link your pr to the issue.

### Code Checklist

* [ ] run the command ```cargo build``` and verify that your changes compile. If adjusting weapon formulas this will also update the timestamps for that given weapon
* [ ] make sure your code is linted via rust-analyzer
* [ ] use our [commit tags](#commit-tags) to format your commit message

#### Commit Tags

* chore - general maintence
* bungie - addresses a change due to new changes from bungie
* fix - fixes a bug
* add - new perks, functionality, etc
* data - we had wrong data
* feature - generally also dependent on the frontend/needs kat

# Thank You!

Contributions of all sizes are what make projects like this possible and able to thrive. We thank you for spending your time to contribute to our project. Your help means the world to us here at Foundry. 

- Eyes up, guardian