# Squad - Assemble your squad of AI agents

Squad is a tool for managing a virtual team of AI agents. With squad, you can:
* Create new agents by name
* Manage agents and set them to work
* Stop or kill agents

## 🌟 Quick Start

### Prerequisites

1. **Container Runtime** - Either Podman (recommended) or Docker
   
   **Option A: Podman (Recommended - Rootless & Daemonless)**
    Ensure Podman is installed ([podman docs](https://podman.io/docs/installation))

    Start podman:
    ```bash
    # Start user podman socket
    systemctl --user start podman.socket
    systemctl --user enable podman.socket
    ```
    
    **Option B: Docker**
    Ensure Docker is installed ([docker docs](https://docs.docker.com/engine/install/))

    Add your user to the docker group
    ```bash
    # Add your user to docker group
    sudo usermod -aG docker $USER
    # Then logout and login again
    ```

    Start docker:
    ```bash
    # Start Docker daemon
    sudo systemctl start docker
    sudo systemctl enable docker
    ```
    
2. **Rust** toolchain (1.75 or later)

### Building

TBD...

### Creating Your First Agent

TBD...

## Command Reference

### 'squad'

'''
Usage: squad <COMMAND>

Commands:
  create  Create and start new agent containers
  list    List all active agents
  rm      Stop and remove agent containers
  ps      Show detailed status information for all agents
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
'''

### `squad create'

'''
Create and start new agent containers

Usage: squad create [NAMES]...

Arguments:
  [NAMES]...  Names of the agents to create

Options:
  -h, --help  Print help
'''

#### Examples

* 'squad create Alice - Create a single agent named Alice'
* 'squad create Alice Bob Carol - Create multiple agents at once'

### `squad list`

'''
List all active agents

Usage: squad list

Options:
  -h, --help  Print help
'''

### `squad rm`

'''
Stop and remove agent containers

Usage: squad rm [OPTIONS] [NAMES]...

Arguments:
  [NAMES]...  Names of the agents to remove

Options:
  -a, --all   Remove all agents
  -h, --help  Print help
'''

#### Examples

* 'squad rm Alice - Remove agent named Alice'
* 'squad rm Alice Bob - Remove multiple agents'
* 'squad rm -a - Remove all agents'
* 'squad rm --all - Remove all agents (alternative syntax)'

### `squad ps`

'''
Show detailed status information for all agents

Usage: squad ps [OPTIONS]

Options:
  -h, --help           Print help
''' 

