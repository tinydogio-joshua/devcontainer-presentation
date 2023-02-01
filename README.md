# devcontainer-presentation

> 🚨 This project is used for giving _devcontainer presentations/demos_. It utilizes Rust and Postgres, but is not meant to show "good" or production ready Rust or Postgres code. Their usage is more to show how a stack can be spun up in isolation from the host machine, while still interacting with each other quickly and reliably in a fully configured dev environment.

## What is a devcontainer

_Devcontainers are a feature currently shipping in VSCode (as an extension) utilizing containers to create fully configured and version controlled development environments._

### Great what does that give me?

* Team members are always working from the same foundation on a project:
  * Base Operating System
  * Network Stack
  * Required Extensions and Configuration
  * CLI Tools
  * Host OS Agnostic (Only Requires something like Docker to run the containers)
* The local development environment can be version controlled and changes distributed to the team.
* Development Environments can be hosted in the cloud (Examlpe: GitHub Codespaces)

### Downsides

* Can be a little slower than running the stack on the Host OS.
* If running on Apple Silicon, some containers are still a little flakey as some software is still transitioning.
  * 🚨 I picked this stack for the devcontainer as it currently is one of those flakey containers and I want to show what to do when things go wrong.
