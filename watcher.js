const fs = require('fs')
const { exec } = require("child_process")
console.log('Watching your files...')

exec('./build.sh', (err, stdout, stderr) => {
  console.log('Finished compiling rust code');
  console.log(stderr)
  console.log(`====\n ${Date()} \n==== \n\n`);
})

fs.watch('./src', { encoding: 'buffer' }, (eventType, filename) => {
  console.log('Building the rust code...');
  exec('./build.sh', (err, stdout, stderr) => {
    console.log('Finished compiling rust code');
    console.log(stderr)
    console.log(`====\n ${Date()} \n==== \n\n`);
  })
});