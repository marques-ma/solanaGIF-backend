const bs58 = require('bs58');
const fs = require('fs');
b = bs58.decode('cciKsTM5YyWFHYUrqwbZmZhaqSxeSELXPwHb3E6RYsfZCWYirnJzyKqsm4w3mHDnLwZizE2baw5mSNFcHnfRrNr');
j = new Uint8Array(b.buffer, b.byteOffset, b.byteLength / Uint8Array.BYTES_PER_ELEMENT);
fs.writeFileSync('mykey.json', `[${j}]`);
