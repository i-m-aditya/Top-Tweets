use bat::PrettyPrinter;
use clap::Parser;
use colored::Colorize;
use dotenv::dotenv;
use question::Answer;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::env;

use prettytable::{row, table};

/*
Remove dead code warning
*/
#[allow(dead_code)]
#[derive(Debug)]
struct Response {
    tweet_props: Vec<TweetProps>,
    // nextToken variable which could be null
    next_token: Option<String>,
}

#[derive(Parser, Debug)]
struct Cli {
    /// Description of the command to execute
    username: String,
}

#[derive(Debug)]
struct TweetProps {
    likes: i64,
    tweet_text: String,
}

fn make_api_call(
    client: &Client,
    next_token: &Option<String>,
    user_id: &i64,
) -> Result<Response, reqwest::Error> {
    let bearer_token = env::var("BEARER_TOKEN").unwrap();

    let mut query_params = HashMap::new();
    query_params.insert("tweet.fields", "public_metrics");
    query_params.insert("max_results", "100");

    if next_token.is_some() {
        query_params.insert("pagination_token", next_token.as_ref().unwrap().trim_matches('"'));
        
    }

    let response = client
        .get(format!(
            "https://api.twitter.com/2/users/{}/tweets",
            user_id
        ))
        .header("Authorization", format!("Bearer {}", bearer_token))
        .query(&query_params)
        .send()
        .unwrap()
        .json::<serde_json::Value>()
        .unwrap();



    let data = response.get("data").unwrap().as_array().unwrap();

    // println!("Hello");
    let mut tweets = Vec::new();
    for tweet in data {
        let tweet_text = tweet.get("text").unwrap().as_str().unwrap();
        let likes = tweet
            .get("public_metrics")
            .unwrap()
            .get("like_count")
            .unwrap()
            .as_i64()
            .unwrap();

        tweets.push(TweetProps {
            likes: likes,
            tweet_text: tweet_text.to_string(),
        });
    }

    if response.get("meta").unwrap().get("next_token").is_some() {
        let next_token = response.get("meta").unwrap().get("next_token").unwrap();
        return Ok(Response {
            tweet_props: tweets,
            next_token: Some(next_token.to_string()),
        });
    }
    Ok(Response {
        tweet_props: tweets,
        next_token: None,
    })
}
fn main() {
    dotenv().ok();

    let cli = Cli::parse();
    let username = cli.username;

    let bearer_token = env::var("BEARER_TOKEN").unwrap();

    let client = Client::new();
    let response = client
        .get(format!(
            "https://api.twitter.com/2/users/by?usernames={}",
            username
        ))
        .header("Authorization", format!("Bearer {}", bearer_token))
        .send()
        .unwrap();
    let text = response.json::<serde_json::Value>().unwrap();

    let user_id: i64 = text.get("data").unwrap()[0]
        .get("id")
        .unwrap()
        .as_str()
        .unwrap()
        .parse()
        .unwrap();

    let mut next_token: Option<String> = Option::None;
    
    let mut tweet_props_list: Vec<TweetProps> = Vec::new();
    loop {
        let response = make_api_call(&client, &next_token, &user_id).unwrap();
        tweet_props_list.extend(response.tweet_props);
        
        if response.next_token.is_some() {
            next_token = response.next_token;
        } else {
            break;
        }
    }
    // println!("Total tweets: {}", tweet_props_list.len());


    // sort tweetpropslist by likes in descending order

    tweet_props_list.sort_by(|tweet_a, tweet_b| tweet_b.likes.cmp(&tweet_a.likes));

    let mut table  = table!([bFg => "Index", "Likes", "Tweet"]);

    table.add_row(row!["Index", "Likes", "Tweet"]);

    // print the top 10
    for (index, tweet) in tweet_props_list.iter().enumerate().take(10) {
        if tweet.tweet_text.len() < 50 {

            table.add_row(row![index, tweet.likes, tweet.tweet_text.replace("\n", "").bright_green()]);

        } else {
            table.add_row(row![index, tweet.likes, tweet.tweet_text.get(0..50).unwrap().replace("\n", "").bright_blue()]);

        }
    }
    table.printstd();

    let q = question::Question::new("Which tweet?").ask().unwrap();

    match q {
        Answer::RESPONSE(value) => {
            let index = value.parse::<usize>().unwrap();
            let tweet = &tweet_props_list[index];
            PrettyPrinter::new()
                .input_from_bytes(tweet.tweet_text.trim().as_bytes())
                .language("bash")
                .grid(true)
                .print()
                .unwrap();
        }
        _ => println!("Invalid answer"),
    }
}
