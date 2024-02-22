# Namada REST API

Welcome, Earthling! 🌍👽 You've just docked at the most stellar REST API in the Milky Way: The **Namada REST API**. Serving data at the speed of light across the cosmos, our API is the go-to source for all your interstellar governance and epoch inquiries. Strap in as we guide you through setting up your own galactic data station.

## Prerequisites

Before launching, make sure you have the following onboard:

- Rust: The metal of the cosmos. Ensure you're equipped with the latest stable version to avoid any unexpected asteroid fields.
- Cargo: Your cargo hold for Rust crates. It comes bundled with Rust, so no need for extra spacewalks.

## Setup

Clone this repository to your local star system:

\`\`\`bash
git clone https://github.com/kintsugi-tech/namada-rest.git
cd namada-rest-api
\`\`\`

Configure your spacecraft with the necessary settings by editing `config/Settings.toml`. Don't worry; it's not rocket science! Just specify your `rpc_url`, `bind_ip`, and `port`.

## Launching

To ignite the engines and start serving requests across the galaxy, run:

\`\`\`bash
cargo run
\`\`\`

Your console will light up with the message: "Server listening [bind_ip]:[port]", indicating that you're now broadcasting to the universe.

## Making Contact

Communicate with the API using your preferred space communication tools (like `curl` or Postman). Here are some examples:

- **GET /epoch**: Retrieves the current epoch data.
- **GET /proposal_result/{id}**: Dives into the proposal results for a given ID.

Remember, with great power comes great responsibility. Use this API wisely to maintain peace and prosperity across the galaxies.

## Contributing

Found a wormhole to a new feature or spotted an asteroid of a bug? Open a pull request or issue. Contributions are more welcome than a water planet in a desert solar system!

## License

This project is under the galaxy's most powerful and freedom-respecting license: the GNU General Public License (GPL). For more details, warp to the `LICENSE` file.

## Acknowledgments

- The Galactic Council of Rustaceans for their guidance and wisdom.
- Earthlings and extraterrestrial beings who contributed to this project.

## Farewell

May your queries be swift and your data vast. Happy exploring, space coder! 🚀✨
