# Dev Container Presentation / Demo

> 🚨 This project is used for giving _dev container presentations/demos_. It is not meant to show "good" or production ready code. The purpose is to show how a fully configured development environment can be spun up in isolation from the host machine in minutes.

## What is a `dev container`?

_`Dev Containers` are a feature currently shipping for [Visual Studio Code](https://code.visualstudio.com/) (as an extension) utilizing [containers](https://www.docker.com/resources/what-container/) to create fully configured and version controlled development environments._

### Great what does that give me?

* Team members are always working from the same:
  * Base Operating System
  * Network Stack
  * Required Extensions and Configuration
  * CLI Tools
* Host OS agnostic.
* Full project setup in minutes.
* Version controlled Development experience.
* Can be hosted in the cloud (Example: [GitHub Codespaces](https://github.com/features/codespaces)).

### Downsides

* Slower than running on the Host OS.
* Currently a [Visual Studio Code](https://code.visualstudio.com/) and [GitHub Codespaces](https://github.com/features/codespaces) _ONLY_ feature.
  * [Currently being standardized](https://containers.dev/).
  * [Other vendors](https://youtrack.jetbrains.com/issue/IDEA-292050/Support-for-.devcontainer#focus=Comments-27-6005224.0-0) are looking to adopt.
* Flakey containers if running on arch64 (Example: Apple Silicon).
  * Software is still transitioning.
  * This is more of a Docker issue than a `dev container` issue.
  * > 🚨 _I picked this stack for this project as it currently is one of those flakey containers and I want to show what to do when things go wrong._

## Requirements

* [Docker](https://www.docker.com/)
* [Visual Studio Code](https://code.visualstudio.com/)
  * [Dev Containers Extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
