use anyhow::Result;
use html_escape::decode_html_entities;

const URL: &str = "https://www.whitehouse.gov/presidential-actions/";

fn main() -> Result<()> {
    let contents = curl(URL)?;
    let actions = parse_contents(&contents);

    for action in &actions {
        action.display();
    }

    Ok(())
}

fn curl(url: &str) -> Result<String> {
    let r = ureq::get(url).call()?;
    Ok(r.into_string()?)
}

#[derive(Debug)]
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

    pub fn display(&self) {
        println!("
\x1b[34;1mPresidential Action\x1b[0m
\x1b[37mTitle: {}
Link: {}\x1b[0m", self.title, self.link);
    }
}

fn parse_contents(contents: &str) -> Vec<Action> {
    let raw_actions: Vec<&str> = contents.lines().filter(|l| {
        l.contains("wp-block-post-title has-heading-4-font-size")
    }).collect(); 

    let mut actions: Vec<Action> = Vec::new();

    raw_actions.iter().for_each(|a| {
        let title_parts = a.rsplitn(4, '>').collect::<Vec<&str>>();
        let (title, _) = title_parts.get(2).unwrap().split_once('<').unwrap();
        let title = decode_html_entities(title);

        let link_parts = a.splitn(5, '"').collect::<Vec<&str>>();
        let link = link_parts.get(3).unwrap();
        let link = decode_html_entities(link);

        actions.push(Action::new(&title, &link))
    });

    actions.reverse();
    actions
}
