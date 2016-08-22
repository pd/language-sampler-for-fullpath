use std::env;
use std::process;
use std::io;
use std::io::prelude::*;

fn write_paths(pbcopy_stdin:&mut process::ChildStdin, paths:&Vec<String>) {
    if paths.len() == 1 {
        match pbcopy_stdin.write(&paths[0].as_bytes()) {
            Ok(_)    => (),
            Err(err) => println!("{}", err)
        }
    } else {
        for path in paths {
            match pbcopy_stdin.write(&path.as_bytes()) {
                Ok(_)    => (),
                Err(err) => println!("{}", err)
            }
        }
    }
    match pbcopy_stdin.flush() {
        Ok(_)    => (),
        Err(err) => println!("{}", err),
    }
}

fn copy_paths(paths:&Vec<String>) {
    let maybe_pbcopy = process::Command::new("/usr/bin/pbcopy")
        .stdin(process::Stdio::piped())
        .spawn();
    match maybe_pbcopy {
        Ok(mut pbcopy) => {
            match pbcopy.stdin.as_mut() {
                Some(pbcopy_stdin) => write_paths(pbcopy_stdin, &paths),
                None => (),
            }
            match pbcopy.wait() {
                Ok(_) => (),
                Err(err) => println!("{}", err),
            }
        },
        Err(err) => println!("{}", err),
    }
}

fn main() {
    let pwd                   = get_pwd();
    let args                  = get_args();
    let print_help            = args.contains(&"-h".to_string()) || args.contains(&"--help".to_string());
    let copy_output           = args.contains(&"-c".to_string()) || args.contains(&"--copy".to_string());
    let mut paths:Vec<String> = args.into_iter().filter(|path| path != "" && !path.starts_with("-")).collect();

    if print_help {
        println!("{}", "usage: fullpath *[relative-paths] [-c]");
        println!("{}", "");
        println!("{}", "  Prints the fullpath of the paths");
        println!("{}", "  If no paths are given as args, it will read them from stdin");
        println!("{}", "");
        println!("{}", "  If there is only one path, the trailing newline is omitted");
        println!("{}", "");
        println!("{}", "  The -c flag will copy the results into your pasteboard");
        process::exit(0);
    }

    if paths.is_empty() {
        let stdin = io::stdin();
        for maybe_line in stdin.lock().lines() {
            match maybe_line { Ok(line) => paths.push(line), Err(_) => (), }
        }
    }
    paths = paths.into_iter().filter(|path| path != "" && !path.starts_with("-")).map(|path| format!("{}/{}", pwd, path)).collect();

    if paths.len() == 1 {
        print!("{}", paths[0]);
        if copy_output {
            copy_paths(&paths);
        }
    } else {
        for path in &paths {
            println!("{}", path);
        }
        if copy_output {
            copy_paths(&paths);
        }
    }
}

fn get_pwd() -> String {
    match env::current_dir() {
        Ok(pwd)  => format!("{}", pwd.display()),
        Err(err) => panic!(err),
    }
}

fn get_args() -> Vec<String>  {
    let mut argv = env::args();
    argv.next(); // first arg in argv is the program name
    argv.collect()
}