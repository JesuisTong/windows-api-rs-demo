import test from 'ava'

import { readRegistry, writeRegistry, deleteRegistry, getProcessExists } from '../index'

if (process.platform === 'win32') {
  test('write read reg', (t) => {
    writeRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run', 'test', 0, 'test')
    const res = readRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run', 'test', 0)
    t.is(res, 'test')
  })

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
