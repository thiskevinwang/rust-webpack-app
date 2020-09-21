import("../pkg")
  .then((mod) => {
    console.log("mod", mod);
    mod.start_websocket();
    mod
      .fetch("thiskevinwang/coffee-code-climb")
      .then((res) => console.log("res", res))
      .catch((err) => console.error("err", err));
  })
  .catch(console.error);
