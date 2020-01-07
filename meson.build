project(
    'Perif',
    default_options: ['prefix=/usr']
)

bindir = get_option('bindir')
sysconfdir = get_option('sysconfdir')

# Binary
install_data('target/release/perif', install_dir: bindir)

# Udev rules
install_subdir('udev', install_dir: sysconfdir)

subdir('data')
