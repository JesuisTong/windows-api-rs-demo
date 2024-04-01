var q = require('./fepro-addon.win32-x64-msvc.node')
console.log(q);
console.time(1)
console.log('readRegistry', q.readRegistry(0, 'Software\\Valve\\Steam', 'AutoLoginUser', 1));
console.timeEnd(1)
console.time(2)
console.log('readRegistry', q.readRegistry(0, 'Software\\Valve\\Steam', 'RememberPassword', 4));
console.timeEnd(2)
console.time(3)
console.log('readRegistry', q.readRegistry(0, 'Software\\Valve\\Steam', 'AutoLoginUser', 0));
console.timeEnd(3)
console.time(4)
console.log('readRegistry', q.readRegistry(0, 'Software\\WeChatAppEx\\ThirdParty', 'StatusCodes', 3));
console.timeEnd(4)
console.time(5)
console.log('readRegistry', q.readRegistry(1, 'SOFTWARE\\WOW6432Node\\Valve\\Steam', 'SteamPID', 4));
console.timeEnd(5)
console.time(6)
console.log('readRegistry', q.readRegistry(1, 'SOFTWARE\\WOW6432Node\\The Silicon Realms Toolworks\\Armadillo', '{3BE69020E11FA404}', 3));
console.timeEnd(6)
console.time(7)
console.log('writeRegistry', q.writeRegistry(0, 'Software\\4e68d05e-43d6-5a2a-b5c0-61957baec133', 'fuck', 4, 2));
console.timeEnd(7)
console.time(8)
console.log('deleteRegistry', q.deleteRegistry(0, 'Software\\4e68d05e-43d6-5a2a-b5c0-61957baec133', 'fuck'));
console.timeEnd(8)
// setTimeout(() => {
//   q.forceShowWindow('5EPRO')
// }, 5000);
