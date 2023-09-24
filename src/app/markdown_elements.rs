use leptos::*;
use std::path::PathBuf;

const ORDERED_LIST_REGEX: &str = "\\A\\h*[0-9]*\\./gm";
const CODE_BLOCK_REGEX: &str = "\\A```";
const HEADING_REGEX: &str = "\\A#{1,6}\\h";
const BLOCKQUOTE_REGEX: &str = "/\\A>+.*/gm";
const UNORDERED_LIST_REGEX: &str = "\\A\\h*[*\\-+]";

#[component]
pub fn text(cx: Scope, content: String) -> impl IntoView {
    view! {cx,
       <p>{content}</p>
    }
}

enum MarkdownLine {
    Heading(usize),
    OrderedListLine(usize),
    UnorderedListLine(usize),
    Blockquote(usize),
    Code,
    HorizontalRule,
}

struct MarkdownModifers<'a> {
    bold: bool,
    italics: bool,
    link: Option<&'a str>,
}

struct Markdown<'a> {
    data: TextImage<'a>,
}

enum TextImage<'a> {
    Text(TextData<'a>),
    Image(ImageData<'a>),
}

struct TextData<'a> {
    content: &'a str,
    line_options: MarkdownLine,
    text_options: MarkdownModifers<'a>,
}

struct ImageData<'a> {
    alt_text: &'a str,
    image_url: PathBuf,
}
pub fn markdown_into_components(markdown: String) {
    //split the document into sections, creating a seperate section for code blocks and lists(and
    //tables later)
    //for code blocks, raise a code block flag then lower it when at the next triple backtick.
    //for lists, go until it doesnt match the list regex.
}
