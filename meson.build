project(
    'Perif',
    default_options: ['prefix=/usr']
)

bindir = get_option('bindir')


# Binary
install_data('target/release/perif', install_dir: bindir)

# Udev rules
subdir('udev')

# Data
subdir('data')
