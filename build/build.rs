mod out;
mod resources;
mod glade;
mod version;

fn main() {

    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=assets/*");

    out::output_dir();
    version::get_version();
    glade::fix_resource_paths();
    resources::generate_resources();

}
