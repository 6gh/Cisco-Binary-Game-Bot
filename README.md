# Cisco Binary Game Bot

A bot for the [Binary Game by Cisco](https://learningcontent.cisco.com/games/binary/index.html)

## Usage

1. Clone this repo
2. [Download the chromedriver](https://googlechromelabs.github.io/chrome-for-testing/#stable) and run in a background process
   - When starting chromedriver, you should see something along the lines of `Starting ChromeDriver... on port 9515`. Port 9515 is the port I have put in the code. It is currently hardcoded. **If it is wrong please change it in the main.rs file, line 9.**
   - You will also need Google Chrome installed.
3. Open the cloned repo, and run `cargo run` to start working
4. Enjoy

## Known Issues

These are known issues I encountered when running this bot. I may or may not fix them in a future date.

- After a while of running, the bot will get slower and slower over time. As of right now, I am not sure why this is.
- At around level 46, the bot will encounter a `StaleElementReference`, and will panic quit. I suspect this is due to the above issue.

## License

The source code is available under an [MIT License](/LICENSE).
