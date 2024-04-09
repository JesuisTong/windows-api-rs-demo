const { writeRegistry, deleteRegistry, readRegistry } = require('./index')

Array.from({ length: 20 }).forEach(() => {
  try {
    deleteRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings', 'ProxyServer')
  } catch (error) {
    console.log(error)
  }
  writeRegistry(
    0,
    'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings',
    'ProxyServer',
    0,
    '127.0.0.1:29613',
  )
  readRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings', 'ProxyEnable', 4)
})
