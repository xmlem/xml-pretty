use std::{
    fs::{write, File},
    io::StdinLock,
    path::{Path, PathBuf},
};

use anyhow::Context;
use gumdrop::Options;
use xmlem::{display, Document};

#[derive(Debug, Options)]
struct Args {
    #[options(help = "display help information")]
    help: bool,

    #[options(free, help = "path to XML document")]
    xml_document_path: Option<PathBuf>,

    #[options(help = "output to file")]
    output_path: Option<PathBuf>,

    #[options(short = "r", long = "replace", help = "replace input file with output")]
    is_replace: bool,

    #[options(help = "number of spaces to indent (default: 2)")]
    indent: Option<usize>,

    #[options(
        short = "e",
        help = "number of spaces to pad the end of an element without separate end-tag (default: 1)"
    )]
    end_pad: Option<usize>,

    #[options(short = "l", help = "max line length (default: 120)")]
    max_line_length: Option<usize>,

    #[options(
        short = "H",
        long = "hex-entities",
        help = "Use hex entity encoding (e.g. &#xNNNN;) for all entities"
    )]
    uses_hex_entities: bool,

    #[options(
        no_short,
        long = "no-text-indent",
        help = "Do not prettify and indent text nodes"
    )]
    is_no_text_indent: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse_args_default_or_exit();

    let input_path = if let Some(path) = args.xml_document_path {
        Some(path)
    } else if atty::is(atty::Stream::Stdin) {
        eprintln!("ERROR: No XML document provided.");
        eprintln!("Run with -h for usage information.");
        return Ok(());
    } else {
        None
    };

    let output_path = if args.is_replace {
        if let Some(input_path) = input_path.as_ref() {
            Some(input_path.clone())
        } else {
            eprintln!("ERROR: cannot replace 'file' when provided stdin data.");
            return Ok(());
        }
    } else {
        args.output_path
    };

    let text = if let Some(input_path) = input_path {
        prettify_file(
            &input_path,
            args.indent,
            args.end_pad,
            args.max_line_length,
            args.uses_hex_entities,
            !args.is_no_text_indent,
        )
        .with_context(|| format!("Failed to prettify '{}'", input_path.display()))?
    } else {
        let stdin = std::io::stdin();
        let stdin = stdin.lock();
        prettify_stdin(
            stdin,
            args.indent,
            args.end_pad,
            args.max_line_length,
            args.uses_hex_entities,
            !args.is_no_text_indent,
        )
        .context("Failed to prettify from stdin")?
    };

    if let Some(path) = output_path {
        write(&path, text).with_context(|| format!("Failed to write to '{}'", path.display()))?;
    } else {
        println!("{}", text);
    }

    Ok(())
}

fn prettify_file(
    path: &Path,
    indent: Option<usize>,
    end_pad: Option<usize>,
    max_line_length: Option<usize>,
    uses_hex_entities: bool,
    indent_text_nodes: bool,
) -> anyhow::Result<String> {
    let file = File::open(path)?;
    let doc = Document::from_file(file)?;
    Ok(prettify(
        doc,
        indent,
        end_pad,
        max_line_length,
        uses_hex_entities,
        indent_text_nodes,
    ))
}

fn prettify_stdin(
    stdin: StdinLock,
    indent: Option<usize>,
    end_pad: Option<usize>,
    max_line_length: Option<usize>,
    uses_hex_entities: bool,
    indent_text_nodes: bool,
) -> anyhow::Result<String> {
    let doc = Document::from_reader(stdin)?;
    Ok(prettify(
        doc,
        indent,
        end_pad,
        max_line_length,
        uses_hex_entities,
        indent_text_nodes,
    ))
}

fn prettify(
    doc: Document,
    indent: Option<usize>,
    end_pad: Option<usize>,
    max_line_length: Option<usize>,
    uses_hex_entities: bool,
    indent_text_nodes: bool,
) -> String {
    doc.to_string_pretty_with_config(
        &display::Config::default_pretty()
            .indent(indent.unwrap_or(2))
            .end_pad(end_pad.unwrap_or(1))
            .max_line_length(max_line_length.unwrap_or(120))
            .entity_mode(if uses_hex_entities {
                display::EntityMode::Hex
            } else {
                display::EntityMode::Standard
            })
            .indent_text_nodes(indent_text_nodes),
    )
}
