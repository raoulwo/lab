// Optional params are specified using a `?`, all required
// params must be specified *before* optional ones.
function log(message: string, userId?: string) {
  let time = new Date().toLocaleTimeString();
  console.log(time, message, userId ?? 'Anonymous');
}

log('Page loaded');
log('User signed in', '1');

// You can define default values for params as well, semantically
// this is equivalent to making the params optional.
function log1(message: string, userId = 'Anonymous') {
  let time = new Date().toLocaleTimeString();
  console.log(time, message, userId);
}

// When specifying a default value, you don't have to explicitly
// type it anymore since it is inferred.
