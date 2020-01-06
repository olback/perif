project(
    'Perif',
    default_options: ['prefix=/usr']
    )

bindir = get_option('bindir')

# Binary
install_data('target/release/perif', install_dir: bindir)

# Udev rules
install_subdir('udev', install_dir: '/home/olback/Desktop/perif/test_target')

# Todo:
# * Icons
# * GSchema
# * Desktop file
# * Appdata
