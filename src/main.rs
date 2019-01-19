use molt::interp::Interp;
use molt::types::InterpResult;
use molt::types::ResultCode;
use std::env;
use std::fs;

fn main() {
    // FIRST, get the command line arguments.
    let args: Vec<String> = env::args().collect();

    // NEXT, create and initialize the interpreter.
    let mut interp = Interp::new();
    interp.add_command("ident", cmd_ident);

    // NEXT, if there's at least one (other than the binary name), then it's a script.
    // TODO: capture the remaining arguments and make 'arg0' and 'argv' available.
    if args.len() > 1 {
        let filename = &args[1];

        match fs::read_to_string(filename) {
            Ok(script) => execute_script(&mut interp, script, &args),
            Err(e) => println!("{}", e),
        }
    } else {
        // Just run the interactive shell.
        // TODO: should be `molt::shell()`
        molt::shell::shell(&mut interp, "% ");
    }
}

fn execute_script(interp: &mut Interp, script: String, args: &[String]) {
    let arg0 = &args[1];
    let argv = if args.len() > 2 {
        args[2..].join(" ") // TODO: Should be joined as a Tcl list!
    } else {
        String::new()
    };

    interp.set_var("arg0", arg0);
    interp.set_var("argv", &argv);

    match interp.eval(&script) {
        Ok(_) => (),
        Err(ResultCode::Error(msg)) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
        Err(result) => {
            eprintln!("Unexpected eval return: {:?}", result);
            std::process::exit(1);
        }
    }
}

/// Command used for dev testing.  It's temporary.
fn cmd_ident(_interp: &mut Interp, argv: &[&str]) -> InterpResult {
    molt::check_args(argv, 2, 2, "value")?;

    Ok(argv[1].into())
}
