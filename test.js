const { writeRegistry, deleteRegistry, readRegistry } = require('./index')

Array.from({ length: 500 }).forEach((_, idx) => {
  writeRegistry(
    0,
    'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings',
    'ProxyServer1',
    0,
    '127.0.0.1:29613',
  )
  const d = readRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings', '127.0.0.1:29613', 0)
  if (d) {
    console.log('readRegistry', idx, d)
    deleteRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings', '127.0.0.1:29613')
    process.exit()
  }
})
