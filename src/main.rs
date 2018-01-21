extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "please", about = "Universal project builder")]
enum Opt {
    #[structopt(name = "build")]
    Build {
    },
    #[structopt(name = "run")]
    Run {
    },
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
