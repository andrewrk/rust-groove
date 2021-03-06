#![feature(os)]
#![feature(io)]
#![feature(core)]
#![feature(path)]
extern crate groove;

// read or update metadata in a media file

fn main() {
    let mut stderr = std::old_io::stderr();
    let args = std::os::args();
    let exe = args[0].as_slice();

    if args.len() < 2 {
        print_usage(&mut stderr, exe);
        std::os::set_exit_status(1);
        return;
    }
    let _ = writeln!(&mut stderr, "Using libgroove version v{}", groove::version());

    let filename = Path::new(args[1].as_bytes());
    groove::set_logging(groove::Log::Info);

    {
        let file = groove::File::open(&filename).expect("error opening file");

        let mut i = 2;
        while i < args.len() {
            let arg = args[i].as_slice();
            if arg == "--update" {
                if i + 2 >= args.len() {
                    let _ = writeln!(&mut stderr, "--update requires 2 arguments");
                    print_usage(&mut stderr, exe);
                    std::os::set_exit_status(1);
                    return;
                }
                let key = args[i + 1].as_slice();
                let value = args[i + 2].as_slice();
                i += 2;
                file.metadata_set(key, value, false).ok().expect("unable to set metadata");
            } else if arg == "--delete" {
                if i + 1 >= args.len() {
                    let _ = writeln!(&mut stderr, "--delete requires 1 argument");
                    print_usage(&mut stderr, exe);
                    std::os::set_exit_status(1);
                    return;
                }
                let key = args[i + 1].as_slice();
                i += 1;
                file.metadata_delete(key, false).ok().expect("unable to delete metadata");
            } else {
                print_usage(&mut stderr, exe);
                std::os::set_exit_status(1);
                return;
            }

            i += 1;
        }

        println!("duration={}", file.duration());
        for tag in file.metadata_iter() {
            let k = tag.key().ok().unwrap();
            let v = tag.value().ok().unwrap();
            println!("{}={}", k, v);
        }
        if file.is_dirty() {
            file.save().ok().expect("unable to save file");
        }
    }

    groove::finish();
}

fn print_usage(stderr: &mut std::old_io::LineBufferedWriter<std::old_io::stdio::StdWriter>, exe: &str) {
    let _ = write!(stderr, "Usage: {} <file> [--update key value] [--delete key]\n", exe);
    let _ = write!(stderr, "Repeat --update and --delete as many times as you need to.\n");
}
