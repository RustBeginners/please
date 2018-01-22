extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod platform;

use platform::probe;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "please", about = "Universal project builder")]
enum Opt {
    #[structopt(name = "build")]
    Build,
    #[structopt(name = "run")]
    Run,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    match probe() {
        None => println!("platform or language not recognized"),
        Some(p) => {
            let res = match opt {
                Opt::Build => p.build(),
                Opt::Run => p.run(),
            };

            if res {
                println!("success!");
            } else {
                println!("failed");
            }
        }
    }
}
