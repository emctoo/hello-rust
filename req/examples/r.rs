use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    code: String,
    description: String,

    #[serde(rename = "type")]
    question_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Resp {
    _status: u16,
    _timestamp: String,
    questions: Vec<Product>,
}

#[derive(Parser)]
#[command(author = "xG", version = "1.0", about = "subscription questions", long_about=None)]
struct Cli {
    // name: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Digital,
    Combo,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let cli = Cli::parse();
    println!("{:?}", cli.command);
    match cli.command {
        Some(cmd) => {
            run(cmd).await?;
        }
        None => {
            run(Commands::Digital).await?;
            run(Commands::Combo).await?;
        }
    };
    Ok(())
}

async fn run(cmd: Commands) -> Result<(), reqwest::Error> {
    println!("\n- execut {:?}", cmd);
    let api = "https://ibdservices.stage.investors.com/im/api/product/subscribe/questions";
    // let basic_url = "https://ibdservices.stage.investors.com/im/api/product/subscribe/questions?productGroup=ICA";
    // let combo_url = format!("{}{}", basic_url, "&productCategory=34");
    // let url = match cli.command {
    //     Commands::Digital => basic_url,
    //     Commands::Combo => &combo_url,
    // };
    // println!("url: {}", url);

    // let mut resp = reqwest::get(url).await?.json::<Resp>().await?;
    // println!("status: {}, timesamp: {}", resp._status, resp._timestamp);

    let client = reqwest::Client::new();
    let query = match cmd {
        Commands::Combo => vec![("productGroup", "ICA"), ("productCategory", "34")],
        Commands::Digital => vec![("productGroup", "ICA")],
    };
    let mut resp = client
        .get(api)
        .query(&query)
        .send()
        .await?
        .json::<Resp>()
        .await?;

    resp.questions
        .as_mut_slice()
        .sort_by(|&Product { code: ref a, .. }, &Product { code: ref b, .. }| a.cmp(b));

    for p in &resp.questions {
        // for p in questions {
        println!("{} - {} - \"{}\"", p.question_type, p.code, p.description);
    }
    println!("total of questions: {}", resp.questions.len());
    Ok(())
}
