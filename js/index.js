import("../pkg/index.js")
  .then((mod) => {
    mod.run(4, [2]);
  })
  .catch(console.error);
