use colored::Colorize;
use std::path::Path;

const DEFAULT_DEPTH: u32 = 1;
const PREFIX_FIRST: &str = "╭";
const PREFIX_MIDDLE: &str = "├";
const PREFIX_LAST: &str = "╰";

struct WDTArgs<'a> {
    path: &'a Path,
    base_depth: u32,
    depth: u32,
}

impl<'a> WDTArgs<'a> {
    fn indent(&self) -> usize {
        (self.base_depth - self.depth) as usize
    }

    fn go_deep(&self, path: &'a Path) -> Self {
        WDTArgs {
            path,
            base_depth: self.base_depth,
            depth: self.depth - 1,
        }
    }
}

impl Default for WDTArgs<'_> {
    fn default() -> Self {
        WDTArgs {
            path: Path::new("."),
            base_depth: DEFAULT_DEPTH,
            depth: DEFAULT_DEPTH,
        }
    }
}

fn get_prefix_symbol(indent: usize, index: usize, total: usize) -> String {
    match (index, total, indent) {
        (0, _, 0) => PREFIX_FIRST,
        (i, t, _) if i == t - 1 => PREFIX_LAST,
        (_, _, _) => PREFIX_MIDDLE,
    }
    .to_string()
}

fn format_name(path: &Path, name: &str, indent: usize, index: usize, total: usize, leaf: bool) -> String {
    format!(
        "{}{} {}",
        "  ".repeat(indent),
        get_prefix_symbol(indent, index, total).dimmed(),
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

    if args.depth == 0 {
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
    let args = WDTArgs::default();

    if let Err(e) = working_directory_tree(&args) {
        eprintln!("{}", e);
    }
}
