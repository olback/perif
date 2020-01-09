# Icons
install_subdir('icons',
    install_dir: join_paths(get_option('datadir')),
)

# Desktop file
install_data('net.olback.Perif.desktop',
    install_dir: join_paths(get_option('datadir'), 'applications')
)

# Appdata
install_data('net.olback.Perif.appdata.xml',
    install_dir: join_paths(get_option('datadir'), 'metainfo')
)

# Gschema
# install_data('net.olback.perif.gschema.xml',
#     install_dir: join_paths(get_option('datadir'), 'glib-2.0', 'schemas'),
# )

# Compile schemas
meson.add_install_script('meson_post_install.py')
