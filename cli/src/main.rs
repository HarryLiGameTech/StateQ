mod preprocessor;

extern crate core;

use std::collections::BTreeMap;
use std::env::temp_dir;
use std::{env, fs};
use std::fs::File;
use std::io::Write;
use std::process::{Command, exit};
use bat::line_range::{LineRange, LineRanges};
use bat::PrettyPrinter;
use clap::{ArgEnum, Parser};
use colored::Colorize;
use uuid::Uuid;
use stateq_compiler::CompileErrType;
use crate::preprocessor::HostLanguage;

extern crate stateq_compiler;

#[derive(PartialEq, Eq, Clone, ArgEnum)]
enum Subcommand {
    Build,
}

#[derive(Parser)]
#[clap(version)]
struct Args {
    #[clap(arg_enum)]
    pub subcommand: Subcommand,

    /// The source file to compile
    #[clap(short = 'i', long)]
    pub file: String,

    /// The output file
    #[clap(short = 'o', long)]
    pub output: Option<String>,

    /// The host-language interface file to generate
    #[clap(short = 'n', long = "inc")]
    pub generate_include: Option<String>,

    /// Keep generated host-language source file
    #[clap(short = 'k', long = "keep-intermediate-src")]
    pub keep_intermediate_src: bool,

    /// Suspend host-language compiler warnings
    #[clap(short = 'q', long = "quiet", default_value = "true")]
    pub quiet: bool,

    /// QIVM library path
    #[clap(short = 'l', long = "qivm-lib-path")]
    pub lib_path: Option<String>,

    /// Optimization level
    #[clap(short = 'O', long = "opt-level")]
    pub optimization_level: Option<u32>,

    /// C compiler
    #[clap(short = 'c', long = "cc")]
    pub c_compiler: Option<String>,

    /// C compiler flags
    #[clap(long = "cc-flags")]
    pub c_compiler_flags: Option<String>,
}

fn print_error_src(src_path: &str, line: i32, column: i32) {
    println!(" File `{}` line {} col {}:",
         src_path.green(),
         line.to_string().cyan(),
         column.to_string().cyan(),
    );

    let line_from = if line < 3 { 0 } else { line - 3 } as usize;
    let line_to = (line + 3) as usize;

    let mut printer = PrettyPrinter::new();
    printer
        .input_file(src_path)
        .header(false)
        .line_numbers(true)
        .line_ranges(LineRanges::from(vec![
            LineRange::new(line_from, line_to)
        ]))
        .highlight(line as usize);
    printer.print().unwrap();
}

fn print_err(err_type: CompileErrType, message: &str) {
    println!("[{}] {}", match &err_type {
        CompileErrType::Note => "Note".cyan().bold(),
        CompileErrType::Error => "Error".red().bold(),
        CompileErrType::Warning => "Warning".yellow().bold(),
        CompileErrType::Help => "Help".green().bold(),
    }, message);
}

fn raise_error(message: &str) -> ! {
    print_err(CompileErrType::Error, message);
    exit(1);
}

macro_rules! raise_error {
    ($($arg:tt)*) => {
        raise_error(&format!($($arg)*));
    };
}

fn main() {
    let args = Args::parse();

    let mut config = BTreeMap::<String, String>::new();

    let tmp_dir = temp_dir().join(Uuid::new_v4().to_string());
    fs::create_dir_all(&tmp_dir).unwrap();

    let source_ext = args.file.split('.').rev().nth(1).unwrap();
    let host_lang = HostLanguage::from_extension(source_ext).unwrap_or_else(|| {
        raise_error!("Unsupported source file extension: {}", source_ext);
    });

    let file_name = args.file
        .split('/').rev().next().unwrap()
        .split('.').next().unwrap().to_string();

    let code = fs::read_to_string(&args.file).unwrap_or_else(|_| {
        raise_error!("Unable to read file {}", args.file);
    });
    let embedded_source = preprocessor::EmbeddedStateqSource::new(host_lang, code);

    let stateq_source_path = tmp_dir.join(file_name.to_string() + ".qc");
    File::create(tmp_dir.join(file_name.to_string() + ".qc")).unwrap_or_else(|_| {
        raise_error!("Unable to create temporary file");
    }).write_all(embedded_source.get_embedded_source().as_bytes()).unwrap_or_else(|_| {
        raise_error!("Unable to write to temporary file");
    });

    config.insert("workdir".into(), tmp_dir.to_str().unwrap().into());
    config.insert("targets".into(), tmp_dir.join("target.c").to_str().unwrap().to_string());

    let compile_result = stateq_compiler::compile(stateq_source_path.to_str().unwrap(), &config);
    for err in compile_result.errors.iter() {
        print_err(err.err_type, &err.message);
        if !&err.source.is_empty() {
            print_error_src(&err.source, err.line, err.column);
        }
    }

    if compile_result.errors.iter().any(|err| matches!(err.err_type, CompileErrType::Error)) {
        exit(1);
    }

    let stateq_home_dir = env::var("STATEQ_HOME").unwrap_or_else(|_| {
        raise_error!("STATEQ_HOME environment variable is not set");
    });

    let compiled_source = fs::read_to_string(tmp_dir.join("target.c")).unwrap_or_else(|_| {
        raise_error!("Unable to read the compiled source file");
    });
    let full_target_source = embedded_source.replace_embedded_source(&compiled_source);
    File::create(format!("{}.target.c", file_name)).unwrap_or_else(|_| {
        raise_error!("Unable to create target source file {}.target.c", file_name);
    }).write_all(full_target_source.as_bytes()).unwrap_or_else(|_| {
        raise_error!("Unable to write to target source file {}.target.c", file_name);
    });

    let mut cc = Command::new(args.c_compiler.unwrap_or("gcc".into()));
    cc.args([
        file_name.to_string() + ".target.c",
        format!("-I{}/include", stateq_home_dir),
        format!("-L{}/lib", stateq_home_dir),
        "-lquantcrt".to_string(),
        "-lqivm".to_string(),
        "-lqil".to_string(),
        "-lm".to_string(),
        "-Wl,-rpath=./".to_string(),
        "-o".to_string(), file_name,
        format!("-O{}", args.optimization_level.unwrap_or(2)),
    ]).spawn().unwrap_or_else(|_| {
        raise_error!("Unable to spawn C compiler process");
    }).wait().unwrap_or_else(|_| {
        raise_error!("Unable to wait for the C compiler");
    });
}
