use clap::Parser;
use colored::Colorize;
use std::path::Path;

const DEFAULT_DEPTH: u32 = 1;
const PREFIX_FIRST: &str = "╭";
const PREFIX_MIDDLE: &str = "├";
const PREFIX_LAST: &str = "╰";

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
#[clap(name = "wdt")]
struct Cli {
    /// The depth to traverse the directory tree
    #[clap(short, long, default_value = "1")]
    depth: u32,

    /// The location to start the directory tree traversal
    #[clap(value_parser, default_value = ".")]
    location: String,

    /// Set if the tree should be traversed to all leaf nodes
    /// This will override the depth argument
    #[clap(short, long)]
    leaf: bool,
}

struct WDTArgs<'a> {
    path: &'a Path,
    depth: u32,
    indent: u32,
    leaf: bool,
}

impl<'a> WDTArgs<'a> {
    fn from_cli(cli: &'a Cli) -> Self {
        WDTArgs {
            path: Path::new(&cli.location),
            depth: cli.depth,
            indent: 0,
            leaf: cli.leaf,
        }
    }

    fn indent(&self) -> usize {
        self.indent as usize
    }

    fn go_deep(&self, path: &'a Path) -> Self {
        let depth = if self.leaf { 1 } else { self.depth - 1 };
        WDTArgs {
            path,
            depth,
            indent: self.indent + 1,
            leaf: self.leaf,
        }
    }
}

impl Default for WDTArgs<'_> {
    fn default() -> Self {
        WDTArgs {
            path: Path::new("."),
            depth: DEFAULT_DEPTH,
            indent: 0,
            leaf: false,
        }
    }
}

fn colored_prefix(prefix: &str, indent: usize) -> String {
    let color = match indent % 6 {
        0 => "blue",
        1 => "green",
        2 => "red",
        3 => "yellow",
        4 => "magenta",
        _ => "cyan",
    };

    prefix.color(color).to_string()
}

fn get_prefix_symbol(indent: usize, index: usize, total: usize) -> String {
    colored_prefix(
        match (index, total, indent) {
            (0, _, 0) => PREFIX_FIRST,
            (i, t, _) if i == t - 1 => PREFIX_LAST,
            (_, _, _) => PREFIX_MIDDLE,
        },
        indent,
    )
}

fn format_name(path: &Path, name: &str, indent: usize, index: usize, total: usize, leaf: bool) -> String {
    format!(
        "{}{} {}",
        "  ".repeat(indent),
        get_prefix_symbol(indent, index, total).bold(),
        if path.is_dir() {
            format!(
                "{}/",
                if !leaf {
                    name.dimmed().to_string()
                } else {
                    name.to_string()
                }
            )
        } else {
            name.to_string()
        }
    )
}

fn working_directory_tree(args: &WDTArgs) -> Result<(), String> {
    if !args.path.is_dir() {
        return Err(format!("{} is not a directory", args.path.display()));
    }

    if !args.leaf && args.depth == 0 {
        return Ok(());
    }

    let dir = match args.path.read_dir() {
        Ok(dir) => dir.filter_map(Result::ok),
        Err(_) => return Ok(()), // skip over read acces fails
    };

    let total = args.path.read_dir().unwrap().count();

    for (i, entry) in dir.enumerate() {
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();

        println!(
            "{}",
            format_name(&path, &name, args.indent(), i, total, args.depth == 1)
        );

        if path.is_dir() {
            let new_args = args.go_deep(&path);
            working_directory_tree(&new_args)?;
        }
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    // let args = WDTArgs::default();
    let args = WDTArgs::from_cli(&cli);

    if let Err(e) = working_directory_tree(&args) {
        eprintln!("{}", e);
    }
}
