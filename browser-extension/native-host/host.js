// Native Messaging Host Bridge for Downbyte
const fs = require('fs');
const http = require('http');

function readMessage() {
  const header = Buffer.alloc(4);
  let bytesRead = 0;

  try {
    bytesRead = fs.readSync(0, header, 0, 4, null);
    if (bytesRead < 4) return null;
  } catch {
    return null;
  }

  const msgLen = header.readUInt32LE(0);
  const msgBuf = Buffer.alloc(msgLen);
  fs.readSync(0, msgBuf, 0, msgLen, null);

  return JSON.parse(msgBuf.toString('utf-8'));
}

function sendMessage(msg) {
  const jsonStr = JSON.stringify(msg);
  const lenBuf = Buffer.alloc(4);
  lenBuf.writeUInt32LE(Buffer.byteLength(jsonStr, 'utf-8'), 0);
  process.stdout.write(lenBuf);
  process.stdout.write(jsonStr);
}

// Forward to local Downbyte HTTP bridge
function forwardToDownbyte(payload) {
  const reqData = JSON.stringify(payload);
  const req = http.request({
    hostname: '127.0.0.1',
    port: 4192,
    path: '/download',
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Content-Length': Buffer.byteLength(reqData)
    }
  }, (res) => {
    sendMessage({ success: res.statusCode === 200 });
  });

  req.on('error', () => {
    sendMessage({ success: false, error: 'Downbyte app not running' });
  });

  req.write(reqData);
  req.end();
}

const msg = readMessage();
if (msg && msg.payload) {
  forwardToDownbyte(msg.payload);
} else {
  sendMessage({ status: 'ready' });
}

