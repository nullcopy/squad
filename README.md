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

Clone the repository and build:

```bash
git clone https://github.com/cargodog/squad.git
cd squad
./build.sh
```

The binary will be available at `target/release/squad`.

### Installation

You can install squad using cargo:

```bash
cargo install --path .
```

Or run directly:
```bash
cargo run -- --help
```

### Creating Your First Agent

1. **Create an agent**:
   ```bash
   squad create my-agent
   ```

2. **List your agents**:
   ```bash
   squad list
   ```

3. **Check agent status**:
   ```bash
   squad ps
   ```

4. **Remove an agent**:
   ```bash
   squad rm my-agent
   ```

## 📖 Usage

Squad provides several commands to manage your AI agents:

### Commands

- `squad create <name>` - Create a new agent
- `squad list` - List all agents  
- `squad ps` - Show running agent status
- `squad rm <name>` - Remove an agent

### Examples

```bash
# Create multiple agents
squad create researcher
squad create writer
squad create reviewer

# Check what's running
squad ps

# Clean up
squad rm researcher writer reviewer
```

## 🏗️ Architecture

Squad uses container technology (Podman/Docker) to isolate and manage AI agents. Each agent runs in its own container environment, providing:

- **Isolation**: Agents run independently 
- **Resource Management**: Container-level resource controls
- **Security**: Sandboxed execution environment
- **Portability**: Consistent runtime across platforms

## 🧪 Development

See [CONTRIBUTING.md](CONTRIBUTING.md) for complete development setup, testing guidelines, code standards, and contribution workflow.

## 🔧 Configuration

Squad automatically detects your container runtime:

1. **Podman** (preferred) - Rootless and daemonless
2. **Docker** - Fallback option

No additional configuration required for basic usage.

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, coding standards, and submission guidelines.

## 📄 License

TODO

## 🔗 Links

- [Repository](https://github.com/cargodog/squad)
- [Issues](https://github.com/cargodog/squad/issues)
- [Releases](https://github.com/cargodog/squad/releases)



## License

This project is licensed under the [GNU Affero General Public License v3.0 or later](LICENSE) (`AGPL-3.0-or-later`).
