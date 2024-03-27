const assync = async () => {
  console.log("ass begin");
  await new Promise((resolve) => {
    setTimeout(() => {
      resolve();
    }, 3000);
  });
  console.log("ass end");
};
const fn1 = () => {
  console.log("fn1 start");
  assync();
  console.log("fn1 end");
};

const fn2 = async () => {
  console.log("fn2 start");
  await assync();
  console.log("fn2 end");
};

const fn3 = async () => {
  console.log("fn3 start");
  await fn1();
  console.log("fn3 end");
};

const fn4 = () => {
  console.log("fn4 start");
  return assync();
  console.log("fn4 end");
};

const fn5 = async () => {
  console.log("fn5 start");
  await fn4();
  console.log("fn5 end");
};

const fn6 = () => {
  console.log("fn6 start");
  fn4();
  console.log("fn6 end");
};

fn6();
