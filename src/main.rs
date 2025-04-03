use anyhow::Result;
use html_escape::decode_html_entities;
use rayon::{ThreadPoolBuilder, prelude::*};
use std::{
    io::Read,
    sync::{Arc, Mutex},
};

mod args;

const URL: &str = "https://www.whitehouse.gov/presidential-actions/";

fn main() -> Result<()> {
    let args = init();
    let pages = args.pages.unwrap_or(1);

    let contents = curl(URL)?;
    let max_pages = get_total_pages(&contents)?;
    let pages = pages.min(max_pages);

    let mut actions = get_num_actions(pages)?;
    actions.reverse();

    for action in actions.clone().into_iter().enumerate() {
        let (i, action) = action;
        let i = actions.len() - i;
        action.display(i)
    }

    Ok(())
}

fn init() -> args::Args {
    args::Args::init()
}

fn curl(url: &str) -> Result<String> {
    let r = ureq::get(url).call()?;

    let mut reader = r.into_body().into_reader();
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    Ok(String::from_utf8_lossy(&buf).to_string())
}

#[derive(Debug, Clone)]
struct Action {
    title: String,
    link: String,
}

impl Action {
    pub fn new(title: &str, link: &str) -> Self {
        Self {
            title: title.to_string(),
            link: link.to_string(),
        }
    }

    pub fn display(&self, i: usize) {
        println!(
            "
\x1b[34;1m-{i} Presidential Action\x1b[0m
\x1b[37mTitle: {}
Link: {}\x1b[0m",
            self.title, self.link
        );
    }
}

fn parse_contents(contents: &str) -> Vec<Action> {
    let raw_actions: Vec<&str> = contents
        .lines()
        .filter(|l| l.contains("class=\"wp-block-post-title\"><a href="))
        .collect();

    let mut actions: Vec<Action> = Vec::new();

    raw_actions.iter().for_each(|a| {
        let title_parts = a.rsplitn(4, '>').collect::<Vec<&str>>();
        let (title, _) = title_parts.get(2).unwrap().split_once('<').unwrap();
        let title = decode_html_entities(title);

        let iso1 = &a[a.find("href=\"").unwrap() + 6..];
        dbg!(&a, &iso1);
        let link = &iso1[..iso1.find("\"").unwrap()];
        let link = decode_html_entities(link);

        actions.push(Action::new(&title, &link))
    });

    actions
}

fn get_total_pages(contents: &str) -> Result<u32> {
    let line = contents.lines().filter(|l| {
        l.contains(r#"<a data-wp-key="index-4" data-wp-on--click="core/query::actions.navigate" class="page-numbers" href="https://www.whitehouse.gov/presidential-actions/page/"#)
    }).collect::<Vec<_>>();
    let line = line.first().unwrap();

    let pages = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<Vec<_>>()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap();
    Ok(pages)
}

// this means the first page is downloaded twice
// yes this is suboptimal
// yes i dont care
fn get_num_actions(n: u32) -> Result<Vec<Action>> {
    if n == 0 {
        panic!("Invalid value")
    }

    let actions: Arc<Mutex<Vec<Action>>> = Arc::new(Mutex::new(vec![
        Action {
            title: String::new(),
            link: String::new()
        };
        (n * 10) as usize
    ]));

    let pool = ThreadPoolBuilder::new().num_threads(n as usize).build()?;

    pool.install(|| {
        (1..=n).into_par_iter().for_each(|i| {
            let url = format!("{URL}page/{i}");
            let contents = curl(&url).expect("Failed to fetch url");

            let acts = parse_contents(&contents);
            let mut actions_lock = actions.lock().unwrap();

            let start_index = (i - 1) as usize * 10;
            for (j, act) in acts.into_iter().enumerate() {
                actions_lock[start_index + j] = act;
            }
        });
    });

    Ok(actions.lock().unwrap().clone())
}
