# Rustic-Chat 🌐

**Rustic Chat** is a CLI application for peer-to-peer chatting using the `iroh-gossip` protocol.

🇷🇺 : [Документация](docs/RU_DOCS.md)

---


### Overview 🌟
Rustic Chat enables users to open or join chat rooms using tickets. It leverages the `iroh-gossip` protocol for seamless P2P communication.
It also features a CLI made using CLAP (Command line argument parser) in Rust.

---

### Features ✨
- **Open a Chat Room**: Create a new chat topic and generate a ticket for others to join.
- **Join a Chat Room**: Use a ticket to connect to an existing chat topic.
- **Custom Nicknames**: Set a nickname for the chat session.
- **Relay Mode Options**: Enable or disable relay functionality.

---

### Usage 🚀

#### Command-Line Arguments:
```bash
USAGE:
    rustic_chat [OPTIONS] <COMMAND>

OPTIONS:
    --no-relay            Disable relay completely
    -n, --name <NAME>     Set your nickname

COMMANDS:
    open                  Open a chat room and generate a ticket
    join <TICKET>         Join a chat room using a ticket
```

#### Examples:
1. **Open a Chat Room**:
   ```bash
   rustic_chat open
   ```
   Output:
   ```
   > creating a chat room for topic <TOPIC_ID>
   > ticket to join us: <TICKET>
   ```

2. **Join a Chat Room**:
   ```bash
   rustic_chat join <TICKET>
   ```
   Output:
   ```
   > joining the following chat room: <TOPIC_ID>
   > connected
   ```

---

### Code Structure 📂

#### File Organization:
```plaintext
src/
├── main.rs          // Entry point
├── args.rs          // Command-line arguments
├── message.rs       // Message-related functionality
├── ticket.rs        // Ticket creation and parsing
├── utils.rs         // Utility functions
```

#### High-Level Workflow:
1. Parse CLI arguments.
2. Open or join a chat room based on the command.
3. Set up gossip and endpoint communication.
4. Broadcast and receive messages using the `iroh-gossip` protocol.

---

### Contributing 🤝
We welcome contributions! Feel free to fork the repository and submit pull requests.

---

### License 📜
This project is licensed under the MIT License.

---


