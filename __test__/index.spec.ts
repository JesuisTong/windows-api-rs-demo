import test from 'ava'

import { readRegistry, writeRegistry, deleteRegistry, getProcessExists } from '../index'

if (process.platform === 'win32') {
  test('write proxy', (t) => {
    writeRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings', 'ProxyServer', 0, '127.0.0.1:29613');
    const res = readRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings', 'ProxyServer', 0);
    t.is(res, '127.0.0.1:29613')
  })

  test('delete proxy', (t) => {
    deleteRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings', 'ProxyServer');
    const res = readRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings', 'ProxyServer', 0);
    t.is(res, null)
  })

  test('write read reg', (t) => {
    writeRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run', 'test', 0, 'test')
    const res = readRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run', 'test', 0)
    t.is(res, 'test')
  })

  // test('write read reg path not exists', (t) => {
  //   writeRegistry(0, 'SOFTWARE\\4e68d05e-43d6-5a2a-b5c0-61957baec133\\fuck', 'test', 0, 'test')
  //   const res = readRegistry(0, 'SOFTWARE\\4e68d05e-43d6-5a2a-b5c0-61957baec133\\fuck', 'test', 0)
  //   t.is(res, 'test')
  // })

  test('delete reg', (t) => {
    writeRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run', 'test', 0, 'test')
    deleteRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run', 'test')
    const err = t.throws(() => {
      deleteRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run', 'test')
    })
    const res = readRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run', 'test', 0)
    t.assert(err instanceof Error)
    t.is(res, null)
  })

  test('get process exists', (t) => {
    t.is(getProcessExists('node.exe'), true)
  })
}
