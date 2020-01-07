mod out;
mod resources;
mod glade;

fn main() {

    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=assets/*");

    out::output_dir();
    glade::fix_resource_paths();
    resources::generate_resources();

}