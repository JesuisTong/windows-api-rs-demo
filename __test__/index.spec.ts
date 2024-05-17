import test from 'ava'

import { readRegistry, writeRegistry, deleteRegistry, getProcessExists } from '../index'

const key = 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings'
if (process.platform === 'win32') {
  test('write|read reg_sz', (t) => {
    writeRegistry(0, key, 'ProxyServer', '127.0.0.1:29613')
    const res = readRegistry(0, key, 'ProxyServer')
    t.is(res, '127.0.0.1:29613')
  })

  test('write|read dword', (t) => {
    writeRegistry(0, key, 'dwd', 1)
    const res = readRegistry(0, key, 'dwd')
    t.is(res, 1)
  })

  test('write|read qword', (t) => {
    writeRegistry(0, key, 'qwd', 100)
    const res = readRegistry(0, key, 'qwd')
    t.is(res, 100)
  })

  test('write|read multi_sz', (t) => {
    writeRegistry(0, key, 'msz', ['1', '2'])
    const res = readRegistry(0, key, 'msz') as string[]
    t.assert(res.join('') === '12', 'res is 12')
  })

  test('write|read binary', (t) => {
    writeRegistry(0, key, 'binary', Buffer.from([1, 2, 3]))
    const res = readRegistry(0, key, 'binary')
    t.assert(typeof res === 'object', 'res is object')
  })

  test('delete', (t) => {
    deleteRegistry(0, key, 'ProxyServer')
    deleteRegistry(0, key, 'binary')
    deleteRegistry(0, key, 'msz')
    deleteRegistry(0, key, 'qwd')
    deleteRegistry(0, key, 'dwd')
    const res = readRegistry(0, key, 'ProxyServer')
    t.is(res, null)
    t.throws(() => deleteRegistry(0, key, 'ProxyServer'))
  })

  test('get process exists', (t) => {
    t.is(getProcessExists('node', false), true)
    t.is(getProcessExists('node.exe', true), true)
  })
}
