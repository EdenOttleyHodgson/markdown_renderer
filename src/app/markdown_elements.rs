use leptos::*;
use regex::{Match, Matches, Regex};
use std::path::PathBuf;

use lazy_static::lazy_static;
lazy_static! {
    static ref ORDERED_LIST_REGEX: Regex = Regex::new(r"\A\h*[0-9]*\./gm").unwrap();
    static ref CODE_BLOCK_REGEX: Regex = Regex::new(r"\A```").unwrap();
    static ref HEADING_REGEX: Regex = Regex::new(r"\A#{1,6}\h").unwrap();
    static ref BLOCKQUOTE_REGEX: Regex = Regex::new(r"\A>+.*/gm").unwrap();
    static ref UNORDERED_LIST_REGEX: Regex = Regex::new(r"\A\h*[*\-+]").unwrap();
    static ref HORIZONTAL_RULE_REGEX: Regex = Regex::new(r"^-{3,}$").unwrap();
    static ref IMAGE_REGEX: Regex = Regex::new(r"!\[.*\]\(.*\)").unwrap();
    static ref LINK_REGEX: Regex = Regex::new(r"\[.*\]\(.*\)").unwrap();
    static ref WORD_SPLIT_REGEX: Regex = Regex::new(r"\b").unwrap();
}
#[component]
pub fn text_line(cx: Scope, content: String) -> impl IntoView {
    //split the content based on inline formats (backtick, asterisk, underscore)
    //create a vec of these contents with the content and an struct representing the formatting
    //options that need to be applied. Render text with different css classes depending on said
    //options.
    view! {cx,
       <p>{content}</p>
    }
}

enum LineOptions {
    Heading(usize),
    OrderedListLine(usize),
    UnorderedListLine(usize),
    Blockquote(usize),
    BlockquoteHeading(usize, usize),
    Code,
    HorizontalRule,
}

impl LineOptions {
    fn new(from_str: &str) -> LineOptions {
        LineOptions::Heading(1)
    }
}

enum UnformattedMarkdownBlock {
    OrderedList(String),
    UnorderedList(String),
    Code(String),
    Regular(String),
}

enum FormattedMarkdownBlock {
    OrderedList(Vec<TextImage>),
    UnorderedList(Vec<TextImage>),
    Code(String),
    Regular(Vec<TextImage>),
}
impl FormattedMarkdownBlock {
    fn from_unformatted(block: UnformattedMarkdownBlock) -> FormattedMarkdownBlock {
        use FormattedMarkdownBlock as FMB;
        use UnformattedMarkdownBlock as UMB;
        match block {
            UMB::Code(content) => FMB::Code(content),
            UMB::OrderedList(content) => {
                let new_block_sections: Vec<TextImage> = Vec::new();

                FMB::Regular(new_block_sections)
            }
            UMB::UnorderedList(content) => {
                let new_block_sections: Vec<TextImage> = Vec::new();
                FMB::Regular(new_block_sections)
            }
            UMB::Regular(content) => {
                let mut new_block_sections: Vec<TextImage> = Vec::new();
                for line in content.lines() {
                    //check line options
                    if HORIZONTAL_RULE_REGEX.is_match(line.trim()) {
                        new_block_sections.push(TextImage::HorizontalRule);
                    } else {
                        let line_options = LineOptions::new(&line);
                        //type info included to inform rust analyzer.
                        let image_iter = IMAGE_REGEX.find_iter(&line).map(|m| m.as_str());
                        let imageless_sections = IMAGE_REGEX.split(&line);
                        for imageless_section in imageless_sections {
                            let link_iter = LINK_REGEX.find_iter(&line).map(|m| m.as_str());
                            let linkless_sections = LINK_REGEX.split(&imageless_section);
                            for linkless_section in linkless_sections {
                                let words = WORD_SPLIT_REGEX.split(linkless_section);
                                let mut current_section: Vec<&str> = Vec::new();
                                let mut current_delimiter = "";
                                for word in words {}
                            }
                        }
                        //get all the start and end indexes for images.
                        // split the line based on those.
                        // format each section.
                    }
                }
                FMB::Regular(new_block_sections)
            }
        }
    }
}

enum MarkdownModifers {
    Bold,
    BoldItalic,
    Code,
    Italic,
    Normal,
}

enum TextImage {
    Text(TextData),
    Image(ImageData),
    Link(LinkData),
    HorizontalRule,
}

impl TextImage {
    fn new(from_str: &str, line_options: LineOptions) -> TextImage {}
}

struct TextData {
    content: String,
    line_options: LineOptions,
    text_options: MarkdownModifers,
}

struct ImageData {
    alt_text: String,
    image_url: PathBuf,
}

struct LinkData {
    url: PathBuf,
    text: String,
}

enum BlockFlags {
    CodeBlock,
    Unordered,
    Ordered,
    Regular,
}

#[component]
fn document(cx: Scope, blocks: Vec<UnformattedMarkdownBlock>) -> impl IntoView {}

#[component]
fn block(cx: Scope, content: UnformattedMarkdownBlock) -> impl IntoView {
    //render line by line?
}

fn split_markdown_into_blocks(markdown: String) -> Vec<UnformattedMarkdownBlock> {
    //split the document into sections, creating a seperate section for code blocks and lists(and
    //tables later)
    //for code blocks, raise a code block flag then lower it when at the next triple backtick.
    //for lists, go until it doesnt match the list regex.

    let lines = markdown.lines();
    let mut blocks: Vec<UnformattedMarkdownBlock> = Vec::new();

    let mut block_flag = BlockFlags::Regular;

    let mut current_block: Vec<&str> = Vec::new();

    for line in lines {
        match block_flag {
            BlockFlags::CodeBlock => {
                if CODE_BLOCK_REGEX.is_match(&line) {
                    blocks.push(UnformattedMarkdownBlock::Code(current_block.concat()));
                    current_block = Vec::new();
                    block_flag = BlockFlags::Regular
                } else {
                    current_block.push(&line)
                }
            }
            BlockFlags::Unordered => {
                if UNORDERED_LIST_REGEX.is_match(&line) {
                    current_block.push(&line)
                } else {
                    blocks.push(UnformattedMarkdownBlock::UnorderedList(
                        current_block.concat(),
                    ));
                    current_block = Vec::new();
                    block_flag = BlockFlags::Regular
                }
            }
            BlockFlags::Ordered => {
                if ORDERED_LIST_REGEX.is_match(&line) {
                    current_block.push(&line)
                } else {
                    blocks.push(UnformattedMarkdownBlock::OrderedList(
                        current_block.concat(),
                    ));
                    current_block = Vec::new();
                    block_flag = BlockFlags::Regular
                }
            }
            BlockFlags::Regular => {
                if ORDERED_LIST_REGEX.is_match(&line) {
                    blocks.push(UnformattedMarkdownBlock::Regular(current_block.concat()));
                    current_block = Vec::new();
                    block_flag = BlockFlags::CodeBlock;
                } else if UNORDERED_LIST_REGEX.is_match(&line) {
                    blocks.push(UnformattedMarkdownBlock::Regular(current_block.concat()));
                    current_block = Vec::new();
                    block_flag = BlockFlags::Unordered;
                } else if CODE_BLOCK_REGEX.is_match(&line) {
                    blocks.push(UnformattedMarkdownBlock::Regular(current_block.concat()));
                    current_block = Vec::new();
                    block_flag = BlockFlags::CodeBlock;
                } else {
                    current_block.push(&line);
                }
            }
        }
    }
    blocks
}

fn split_into_sections(string: &str) -> Vec<&str> {}
