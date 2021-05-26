use clap::{App, Arg};

pub fn app() -> App<'static, 'static> {
    App::new("del-files")
        .version("0.2")
        .about("Make notes from command line")
        .arg(
            Arg::with_name("directory")
                .short("d")
                .long("directory")
                .takes_value(true)
                .required(true)
                .help("Specify the directory to search in."),
        )
        .arg(
            Arg::with_name("targets")
                .short("t")
                .long("targets")
                .value_name("FILE")
                .takes_value(true)
                .multiple(true)
                .required(true)
                .help("Files or directories to delete."),
        )
        .arg(
            Arg::with_name("exclude_directories")
                .short("e")
                .long("exlude-directories")
                .takes_value(true)
                .multiple(true)
                .help("Specify the directory to exclude from search."),
        )
        .arg(
            Arg::with_name("skip_confirmation")
                .short("y")
                .long("skip-confirmation")
                .help("Skip confirming when deleting file/directory."),
        )
        .arg(
            Arg::with_name("recurse")
                .short("r")
                .long("recurse")
                .help("Search recursively for files/directories."),
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .help("Output the disk space freed."),
        )
}
