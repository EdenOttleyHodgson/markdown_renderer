use leptos::*;
use regex::Regex;
use std::path::PathBuf;
const ORDERED_LIST_REGEX: &str = r"\A\h*[0-9]*\./gm";
const CODE_BLOCK_REGEX: &str = r"\A```";
const HEADING_REGEX: &str = r"\A#{1,6}\h";
const BLOCKQUOTE_REGEX: &str = r"\A>+.*/gm";
const UNORDERED_LIST_REGEX: &str = r"\A\h*[*\-+]";

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

enum MarkdownLine {
    Heading(usize),
    OrderedListLine(usize),
    UnorderedListLine(usize),
    Blockquote(usize),
    Code,
    HorizontalRule,
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
            UMB::OrderedList(content) => {}
            UMB::UnorderedList(content) => todo!(),
            UMB::Regular(content) => {}
        }
    }
}

enum MarkdownModifers {
    Bold,
    BoldItalic,
    Code,
    Link(String),
}

enum TextImage {
    Text(TextData),
    Image(ImageData),
}

impl TextImage {
    fn from_str(from_str: &str) -> TextImage {}
}

struct TextData {
    content: String,
    line_options: MarkdownLine,
    text_options: MarkdownModifers,
}

struct ImageData {
    alt_text: String,
    image_url: PathBuf,
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

    //safe to unwrap because the regex strings are constant.
    let code_block_regex = Regex::new(CODE_BLOCK_REGEX).unwrap();
    let unordered_list_regex = Regex::new(UNORDERED_LIST_REGEX).unwrap();
    let ordered_list_regex = Regex::new(ORDERED_LIST_REGEX).unwrap();

    let mut current_block: Vec<&str> = Vec::new();

    for line in lines {
        match block_flag {
            BlockFlags::CodeBlock => {
                if code_block_regex.is_match(&line) {
                    blocks.push(UnformattedMarkdownBlock::Code(current_block.concat()));
                    current_block = Vec::new();
                    block_flag = BlockFlags::Regular
                } else {
                    current_block.push(&line)
                }
            }
            BlockFlags::Unordered => {
                if unordered_list_regex.is_match(&line) {
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
                if ordered_list_regex.is_match(&line) {
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
                if ordered_list_regex.is_match(&line) {
                    blocks.push(UnformattedMarkdownBlock::Regular(current_block.concat()));
                    current_block = Vec::new();
                    block_flag = BlockFlags::CodeBlock;
                } else if unordered_list_regex.is_match(&line) {
                    blocks.push(UnformattedMarkdownBlock::Regular(current_block.concat()));
                    current_block = Vec::new();
                    block_flag = BlockFlags::Unordered;
                } else if code_block_regex.is_match(&line) {
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
