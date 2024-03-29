fetch("./libweb.min.wasm").then(response =>
    response.arrayBuffer()
  ).then(bytes =>
    WebAssembly.instantiate(bytes, { env: { cos: Math.cos } })
  ).then(results => {
    console.log("got instance");
    console.log(results);
    console.log(results.instance.exports);
    let module = {};
    let mod = results.instance;
    module.update  = mod.exports.update;
    module.alloc   = mod.exports.alloc;
    module.dealloc = mod.exports.dealloc;
    module.fill    = mod.exports.fill;
  
    var width  = 500;
    var height = 500;
    var canvas2_width = 500;
    var canvas2_height = 500;
    let byteSize = width * height * 4;
    var pointer = module.alloc( byteSize );
    var buffer = new Uint8Array(mod.exports.memory.buffer, pointer, byteSize);
  
  
  
  
    console.log(module);
    var canvas = document.getElementById('screen');
    if (canvas.getContext) {
      var ctx = canvas.getContext('2d');
  
      var pointer = module.alloc( byteSize );
      var buffer = new Uint8Array(mod.exports.memory.buffer, pointer, byteSize);
  
      image = ctx.getImageData(0, 0, width, height)
      data = image.data
      module.fill(pointer, width*height, 0);
      console.log(image)
      console.log("filled buffer")
  
      var usub = new Uint8ClampedArray(mod.exports.memory.buffer, pointer, byteSize);
      console.log("usub: ");
      var img = new ImageData(usub, width, height);
  
      console.log(usub);
      data.set(usub);
      console.log(data)
      console.log(image.data)
      console.log("set image data")
  
      ctx.putImageData(image, 0, 0)
      console.log("put image data")
  
      var start = null;
      function step(timestamp) {
        var progress;
        if (start === null) start = timestamp;
        progress = timestamp - start;
        if (progress > 100) {
          module.fill(pointer, width*height, timestamp);
  
          start = timestamp
  
          window.requestAnimationFrame(draw);
        } else {
          window.requestAnimationFrame(step);
        }
      }
  
      function draw() {
        ctx.putImageData(img, 0, 0)
        window.requestAnimationFrame(step);
      }
  
      window.requestAnimationFrame(step);
    }
  
  });
  
  