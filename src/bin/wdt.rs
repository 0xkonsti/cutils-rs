use std::path::Path;

const DEFAULT_DEPTH: u32 = 2;

struct WDTArgs<'a> {
    path: &'a Path,
    base_depth: u32,
    depth: u32,
    hide: bool,
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
            hide: self.hide,
        }
    }
}

impl Default for WDTArgs<'_> {
    fn default() -> Self {
        WDTArgs {
            path: Path::new("."),
            base_depth: DEFAULT_DEPTH,
            depth: DEFAULT_DEPTH,
            hide: false,
        }
    }
}

fn working_directory_tree(args: &WDTArgs) -> Result<(), String> {
    if !args.path.is_dir() {
        return Err(format!("{} is not a directory", args.path.display()));
    }

    if args.depth == 0 {
        return Ok(());
    }

    let dir = match args.path.read_dir() {
        Ok(dir) => dir.filter_map(Result::ok).filter(|entry| {
            entry
                .path()
                .file_name()
                .is_some_and(|name| args.hide || !name.to_string_lossy().starts_with('.'))
        }),
        Err(e) => return Err(e.to_string()),
    };

    for entry in dir {
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();
        println!("{}{}", "  ".repeat(args.indent()), name);

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
