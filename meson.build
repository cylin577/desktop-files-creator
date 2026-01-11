project('desktop-files-creator', 'rust',
          version: '0.1.0',
    meson_version: '>= 0.59.0',
)

i18n = import('i18n')
gnome = import('gnome')

prefix = get_option('prefix')
bindir = join_paths(prefix, get_option('bindir'))
datadir = join_paths(prefix, get_option('datadir'))
icondir = join_paths(datadir, 'icons', 'hicolor', 'scalable', 'apps')
desktopdir = join_paths(datadir, 'applications')

cargo = find_program('cargo', required: true)
sh = find_program('sh', required: true)

# Build the Rust binary using cargo
cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  input: ['Cargo.toml', 'src/main.rs', 'src/window.rs', 'resources/window.ui'],
  output: ['desktop-files-creator'],
  console: true,
  install: true,
  install_dir: bindir,
  command: [
    sh, '-c',
    '@0@ build --release --manifest-path @1@ --target-dir @2@ && cp @2@/release/desktop-files-creator @3@'.format(
        cargo.full_path(),
        meson.project_source_root() / 'Cargo.toml',
        meson.project_build_root() / 'target',
        '@OUTPUT@'
    )
  ]
)

# Install the icon
install_data(
  join_paths('data', 'com.github.cylin577.desktop-files-creator.svg'),
  install_dir: icondir
)

# Install the desktop file
desktop_file = i18n.merge_file(
  input: 'data/com.github.cylin577.desktop-files-creator.desktop.in',
  output: 'com.github.cylin577.desktop-files-creator.desktop',
  type: 'desktop',
  po_dir: 'po',
  install: true,
  install_dir: desktopdir
)

gnome.post_install(
  glib_compile_schemas: false,
  gtk_update_icon_cache: true,
  update_desktop_database: true
)