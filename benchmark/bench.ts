import { execSync } from 'node:child_process'

import b from 'benny'

import { getProcessExists, readRegistry, writeRegistry, deleteRegistry } from '../index'

async function run() {
  await b.suite(
    'getProcessExists node.exe',

    b.add('Native getProcessExists exact node.exe', () => {
      getProcessExists('node.exe', true)
    }),

    b.add('Native getProcessExists not exact node.exe', () => {
      getProcessExists('node.exe', false)
    }),

    b.add('cmd getProcessExists node.exe', () => {
      execSync('TASKLIST /FI "USERNAME ne NT AUTHORITY\\SYSTEM" /FI "STATUS eq running"  /FI "IMAGENAME eq node.exe"')
    }),

    b.cycle(),
    b.complete(),
  )

  await b.suite(
    'writeRegistry test',

    b.add('Native writeRegistry test', () => {
      writeRegistry(0, 'Console', 'test', 0)
    }),

    b.add('cmd writeRegistry test', () => {
      execSync('reg add HKCU\\Console /v test /t REG_DWORD /d 0 /f')
    }),

    b.cycle(),
    b.complete(),
  )

  await b.suite(
    'readRegistry test',

    b.add('Native readRegistry test', () => {
      readRegistry(0, 'Console', 'test')
    }),

    b.add('cmd readRegistry test', () => {
      execSync('reg query HKCU\\Console /v test')
    }),

    b.cycle(),
    b.complete(),
  )

  await b.suite(
    'deleteRegistry test',

    b.add('Native deleteRegistry test', () => {
      writeRegistry(0, 'Console', 'test', 0)
      deleteRegistry(0, 'Console', 'test')
    }),

    b.add('cmd deleteRegistry test', () => {
      writeRegistry(0, 'Console', 'test', 0)
      execSync('reg delete HKCU\\Console /v test /f')
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
