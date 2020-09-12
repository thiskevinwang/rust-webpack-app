import("../pkg")
  .then((rust_module) => {
    console.log("rust_module", rust_module);
  })
  .catch(console.error);
