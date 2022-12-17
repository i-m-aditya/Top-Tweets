# Top Tweets

Are you curious about what tweets made a particular Twitter user famous or helped them gain a large number of followers? Look no further! With our command line tool, you can easily find the top tweets of any user.

## Usage

In order to use this tool, you will need to set your Twitter BEARER_TOKEN in your environment for authentication. You can obtain your BEARER_TOKEN from the [Twitter developer page](https://developer.twitter.com/en/portal/dashboard) and set it using the following command:
```bash
export OPENAI_API_KEY='sk-XXXXXXXX'
```

Once you have configured your environment, run `cargo run` followed by the username whose top tweets you wish to find(`cargo run JeffBezos`).

![Terminal Image](https://cdn.discordapp.com/attachments/1037841498449907773/1053686284624875550/Screenshot_2022-12-17_at_8.17.07_PM.png)

## Develop

Make sure you have the latest version of rust installed (use [rustup](https://rustup.rs/)). Then, you can build the project by running `cargo build`, and run it with `cargo run`.

## License

This project is open-sourced under the MIT license. See [the License file](LICENSE) for more information.
