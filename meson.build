project(
    'Perif',
    default_options: ['prefix=/usr']
)

bindir = get_option('bindir')


# Binary
if get_option('install-bin')
    install_data('target/release/perif', install_dir: bindir)
endif


# Udev rules
subdir('udev')

# Data
subdir('data')
