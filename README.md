# Dev Container Presentation / Demo

> 🚨 This project is used for giving _dev container presentations/demos_. It is not meant to show "good" or production ready code. The purpose is to show how a fully configured development environment can be spun up in isolation from the host machine in minutes.


## 🚚 What is a `dev container`?

_`Dev Containers` are a feature currently shipping for [Visual Studio Code](https://code.visualstudio.com/) (as an extension) utilizing [containers](https://www.docker.com/resources/what-container/) to create fully configured and version controlled development environments._

### ✅ Great what does that give me?

* Team members are always working from the same:
  * Base Operating System
  * Network Stack
  * Required Extensions and Configuration
  * CLI Tools
* Host OS agnostic.
* Full project setup in minutes.
* Version controlled Development experience.
* Can be hosted in the cloud (Example: [GitHub Codespaces](https://github.com/features/codespaces)).
* **⭐️ NEW**: [Multiple Configurations Per Project](https://github.com/microsoft/vscode-docs/blob/main/remote-release-notes/v1_75.md#folders-with-multiple-devcontainerjson-files) allowing for different setups for different types of work.

### 👎 Downsides

* Slower than running on the Host OS.
* _Currently_ a [Visual Studio Code](https://code.visualstudio.com/) and [GitHub Codespaces](https://github.com/features/codespaces) only feature.
  * [Currently being standardized](https://containers.dev/).
  * [Other vendors](https://youtrack.jetbrains.com/issue/IDEA-292050/Support-for-.devcontainer#focus=Comments-27-6005224.0-0) are looking to adopt.
* Flakey containers if running on arch64 (Example: Apple Silicon).
  * Software is still transitioning.
  * This is more of a [Docker](https://www.docker.com/) issue than a `dev container` issue.
  * > 🚨 _I picked this stack for this project as it currently is one of those flakey containers and I want to show what to do when things go wrong._


## 🛠️ Requirements

* [Docker](https://www.docker.com/)
* [Visual Studio Code](https://code.visualstudio.com/)
  * [Dev Containers Extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)


# 💬 Presenter Notes

## 🛠️ Pre-Presentation Setup

* Be sure above software is downloaded and configured.
* Delete any containers, images, and volumes associated with this repo prior to presentation.


## 🧐 Things To Show

### ☝️ This Project

* Starting out, no images, container, or volumes exist for this project.
* Cloning this project with Dev Container.
  * Show `.devcontainer` directory.
* "Open in Container" icon and prompt.
* Startup Log
* Terminal (In Container vs Host OS)
* Configuration
  * `.devcontainer.json`
  * `Dockerfile`
* Extensions:
  * SQL (Configured)
    * Show Postgres connection, but no db.
  * Adding new extensions to `.devcontainer.json` via extension panel.
    * Search for `vscode-pets`.
* Migrations
  * Run via Terminal
    * `cd backend`
    * `sqlx migrate run`
  * Show in Extension
* Mapped Ports
  * Start project:
    * `cd backend`
    * `cargo build`
    * `cargo run`
      * > 🚨 I know `cargo run` does a build, but this is working around one of the flakey issues in this container.
  * Show access via host:
    * [http://localhost:3000](http://localhost:3000)
    * > 🚨 Notice database connectivity.
      * Data will persist via container volume, even if container is destroyed.

### 🎉 Creating a New Project

* Show how to make a [node.js](https://nodejs.org/en/) dev container.
* Use newer version than local.
  * Show differences in terminal.
* Example code:
```javascript
console.log("Hello, Dev Container");
```

### 🎨 Show Production Level Project

* Quick Overview of Project
* Mapped Environment Variables
