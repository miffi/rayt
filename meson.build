project('rayt', 'c',
  version : '0.1',
  default_options : ['c_std=c11', 'warning_level=3'])

subdir('src')

exe = executable('rayt', src,
  install : true)

test('basic', exe)
