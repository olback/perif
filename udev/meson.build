sysconfdir = get_option('sysconfdir')

install_data('52-perif.rules', install_dir: join_paths(sysconfdir, 'udev', 'rules.d'))

if get_option('bluetooth')
    install_data('52-perif-bluetooth.rules', install_dir: join_paths(sysconfdir, 'udev', 'rules.d'))
endif
