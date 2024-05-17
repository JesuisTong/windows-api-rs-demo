const { writeRegistry, deleteRegistry, readRegistry, getProcessExists } = require('./index')

const INTERNET_SETTINGS = 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings';

// Array.from({ length: 1 }).forEach((_, idx) => {
//   writeRegistry(0, INTERNET_SETTINGS, 'ProxyEnable', 4, 0);
//   deleteRegistry(0, INTERNET_SETTINGS, 'ProxyServer');
//   // writeRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings', 'ProxyEnable', 4, 0)
//   // console.log(readRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings', 'ProxyEnable', 4));
//   // writeRegistry(
//   //   0,
//   //   'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings',
//   //   'ProxyServer1',
//   //   0,
//   //   '127.0.0.1:29613',
//   // )
//   // const d = readRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings', '127.0.0.1:29613', 0)
//   // if (d) {
//   //   // eslint-disable-next-line
//   //   console.log('readRegistry', idx, d)
//   //   deleteRegistry(0, 'SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings', '127.0.0.1:29613')
//   //   process.exit()
//   // }
// })
console.log('getProcessExists', getProcessExists('code', false));
const clearGlobalProxyReg = async () => {
  try {
      console.log(writeRegistry(0, INTERNET_SETTINGS, 'FFF', ['1','2']));
      console.log(readRegistry(0, INTERNET_SETTINGS, 'FFF'));
      console.log(readRegistry(0, INTERNET_SETTINGS, 'ProxyEnable'));
      console.log(readRegistry(0, INTERNET_SETTINGS, 'MaxConnectionsPerServer'));
      console.log(readRegistry(0, INTERNET_SETTINGS, 'ZonesSecurityUpgrade'));
      writeRegistry(0, INTERNET_SETTINGS, 'ProxyEnable', 0);
      deleteRegistry(0, INTERNET_SETTINGS, 'ProxyServer');
  } catch (error) {
      console.error('clearGlobalProxyReg error', error.message);
  }
};
clearGlobalProxyReg();
clearGlobalProxyReg();
