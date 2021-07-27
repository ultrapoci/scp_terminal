use anyhow::{anyhow, Result};
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode::*, KeyEvent},
    queue,
    style::{Attribute, Color::*},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use ego_tree::NodeRef;
use itertools::Itertools;
use scraper::Node;
use scraper::{node::Element, Html, Selector};
use std::io::{stdout, Write};
use structopt::StructOpt;
use termimad::{Area, MadSkin, MadView, Result as TermResult};

#[derive(Debug, StructOpt)]
#[structopt(name = "SCP Terminal")]
/// Use the Up or Down arrow keys to move through the page.
/// Use PgUp or PgDown to move an entire page up or down.
/// Press any other key to quit.
struct Opt {
    /// SCP code
    ///
    /// Show the page related to this SCP
    scp: String,
}

fn view_area() -> Area {
    let mut area = Area::full_screen();
    area.pad_for_max_width(area.width - 20); // we don't want a too wide text column
    area
}

fn run_app(skin: MadSkin, markdown: String) -> TermResult<()> {
    let lines_to_scroll = 5;
    let mut w = stdout(); // we could also have used stdout
    queue!(w, EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;
    queue!(w, Hide)?; // hiding the cursor
    let mut view = MadView::from(markdown, view_area(), skin);
    loop {
        view.write_on(&mut w)?;
        w.flush()?;
        match event::read() {
            Ok(Event::Key(KeyEvent { code, .. })) => match code {
                Up => view.try_scroll_lines(-lines_to_scroll),
                Down => view.try_scroll_lines(lines_to_scroll),
                PageUp => view.try_scroll_pages(-1),
                PageDown => view.try_scroll_pages(1),
                _ => break,
            },
            Ok(Event::Resize(..)) => {
                queue!(w, Clear(ClearType::All))?;
                view.resize(&view_area());
            }
            _ => {}
        }
    }
    terminal::disable_raw_mode()?;
    queue!(w, Show)?; // we must restore the cursor
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}

fn get_markdown(element: &Element) -> (String, String) {
    let (s1, s2) = match element.name() {
        "a" => ("", ""),
        "strong" => ("**", "**"),
        "em" => ("*", "*"),
        "li" => ("* ", "\n"),
        "p" => ("\n", "\n"),
        "sup" => ("```[", "]```"),
        "div" if element.attr("class") == Some("title") => ("\n## ", "\n"),
        "div" if element.attr("class") == Some("collapsible-block-unfolded-link") => ("`", "`"),
        "div" => ("\n", "\n"),
        "span" if element.attr("style") == Some("text-decoration: line-through;") => ("~~", "~~"),
        "tbody" => ("<tbody>", "</tbody>"),
        "td" => ("<td>", "</td>"),
        "th" => ("<th>", "</th>"),
        "tr" => ("<tr>", "</tr>"),
        _ => ("", ""),
    };
    (s1.to_string(), s2.to_string())
}

fn traverse(node: NodeRef<Node>) -> String {
    match node.value() {
        Node::Text(text) => text.text.to_string().replace("\n", ""),
        Node::Element(elem) if elem.name() == "br" => "\n".to_string(),
        Node::Element(elem) if elem.name() == "hr" => "---".to_string(), 
        Node::Element(elem) => {
            let not_allowed_divs = [
                Some("page-rate-widget-box"),
                Some("collapsible-block-folded"),
                Some("creditRate"),
                Some("footer-wikiwalk-nav"),
                Some("licensebox"),
                //Some("collapsible-block-unfolded-link"),
            ];

            match elem.name() {
                "div"
                    if not_allowed_divs.contains(&elem.attr("class"))
                        | (elem.attr("id") == Some("u-credit-view"))
                        | elem
                            .attr("class")
                            .map_or_else(|| false, |class| class.contains("scp-image-block")) =>
                {
                    "".to_string()
                }
                "script" => "".to_string(),
                _ => {
                    let t = get_markdown(elem);
                    let begin_marker = t.0;
                    let end_marker = t.1;

                    let mut s = String::from(&begin_marker);
                    for child in node.children() {
                        s.push_str(&traverse(child));
                    }
                    s.push_str(&end_marker);

                    if elem.name() == "blockquote" {
                        s = s
                            .trim()
                            .split('\n')
                            .map(|line| {
                                if line.contains("---") {
                                    "---".to_string()
                                } else {
                                    format!("> {}", line)
                                }
                            })
                            .join("\n");
                        s = format!("\n{}\n", s);
                    }

                    /* if elem.name() == "ul" {
                        s = s
                            .trim()
                            .split('\n')
                            .map(|line| format!("\t{}", line))
                            .join("\n");
                        s = format!("\n{}\n", s);
                    } */

                    if elem.name() == "table" {
                        let n_col = s.matches("<td>").count() / s.matches("<tr>").count();
                        let line = "|:--:".repeat(n_col);
                        s.replace("<td>", "|")
                            .replace("</td>", "")
                            .replace("<tbody>", "")
                            .replace("</tbody>", "|-\n")
                            .replace("<th>", "|**")
                            .replace("</th>", "**")
                            .replace("<tr>", &format!("{}\n", &line))
                            .replace("</tr>", "\n")
                    } else {
                        s
                    }
                }
            }
        }
        _ => {
            let mut s = String::new();
            for child in node.children() {
                s.push_str(&traverse(child));
            }
            s
        }
    }
}

async fn get_html(url: String) -> Result<String> {
    let body = reqwest::get(&url).await?.text().await?;
    let fragment = Html::parse_fragment(&body);

    let title = &fragment
        .select(
            &Selector::parse(r#"div[id="page-title"]"#).expect("Selector didn't parse correctly"),
        )
        .next()
        .ok_or_else(|| anyhow!("Iterator of parsed html didn't yield an element for title"))?
        .inner_html();

    let html = Html::parse_fragment(
        &fragment
            .select(
                &Selector::parse(r#"div[id="page-content"]"#)
                    .expect("Selector didn't parse correctly"),
            )
            .next()
            .ok_or_else(|| {
                anyhow!("Iterator of parsed html didn't yield an element for page content")
            })?
            .inner_html(),
    );

    Ok(format!(
        "# **{}**\n{}",
        title.trim(),
        traverse(html.tree.root())
    ))
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    let url = format!("{}{}", "https://scp-wiki.wikidot.com/scp-", opt.scp);

    let mut skin = MadSkin::default();
    skin.bold.set_fg(DarkRed);
    skin.headers[0].add_attr(Attribute::Reverse);
    skin.inline_code.set_bg(Reset);
    skin.inline_code.set_fg(DarkYellow);

    //println!("{}", get_html(String::from(url)).await?);
    run_app(skin, get_html(String::from(url)).await?)?;

    Ok(())
}
